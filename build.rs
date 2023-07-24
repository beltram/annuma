use itertools::Itertools;
use quote::quote;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=communes-departement-region.csv");

    let csv_path = "communes-departement-region.csv";
    let dept_out_file = "src/cli/department.rs";

    let mut reader = csv::Reader::from_path(csv_path)?;
    let departements = reader
        .deserialize::<Commune>()
        .filter_map(|c| c.ok())
        .map(|c| c.sanitize())
        .group_by(|c| {
            let d = c.departement.clone();
            let c = u32::from_str(&c.code_departement).unwrap_or(0);
            (d, c)
        });

    let mut quote_communes = vec![];
    let mut quote_impl_number = vec![];
    let mut quote_impl_commune = vec![];

    let quote_dept =
        departements
            .into_iter()
            .fold(vec![], |mut acc, ((departement, code_departement), c)| {
                let dept = heck::AsUpperCamelCase(&departement).to_string();
                if !dept.is_empty() {
                    let departement = hygiene(&dept);

                    let communes = c
                        .into_iter()
                        .map(|c| heck::AsUpperCamelCase(&c.name).to_string())
                        .unique()
                        .map(|name| {
                            let n = hygiene(&name);
                            // #[clap(about = #name)]
                            quote! {
                                #n
                            }
                        })
                        .collect::<Vec<_>>();

                    quote_communes.push(quote! {
                        #[derive(Debug, Copy, Clone, clap::ValueEnum, strum::Display)]
                        pub enum #departement {
                            #(#communes),*
                        }
                    });

                    quote_impl_number.push(quote! {
                        Department::#departement { .. } => #code_departement
                    });
                    quote_impl_commune.push(quote! {
                        Department::#departement { commune } => Box::new(commune)
                    });

                    let about_dept = format!("{code_departement}-{dept}");
                    let q = quote! {
                        #[clap(about = #about_dept)]
                        #departement {
                            #[arg(value_enum)]
                            commune: #departement,
                        }
                    };
                    acc.push(q);
                }
                acc
            });

    // flatten
    let quote_communes = vec![quote!(#(#quote_communes)*)];
    let quote_dept = vec![quote!(#(#quote_dept),*)];
    let quote_impl_number = vec![quote!(#(#quote_impl_number),*)];
    let quote_impl_commune = vec![quote!(#(#quote_impl_commune),*)];

    let q = quote! {
        #[derive(Debug, clap::Subcommand, strum::Display)]
        pub enum Department {
            #(#quote_dept),*
        }

        impl Department {
            pub fn number(&self) -> u32 {
                match self {
                    #(#quote_impl_number),*
                }
            }

            pub fn commune(&self) -> Box<&dyn std::fmt::Display> {
                match self {
                    #(#quote_impl_commune),*
                }
            }
        }

        #(#quote_communes)*
    };

    std::fs::write(dept_out_file, code(q))?;

    Ok(())
}

fn code(token: proc_macro2::TokenStream) -> String {
    let syntax_tree = syn::parse2(token).unwrap();
    prettyplease::unparse(&syntax_tree)
}

fn hygiene(s: &str) -> proc_macro2::TokenStream {
    s.parse().unwrap()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
struct Commune {
    #[serde(rename = "nom_commune")]
    name: String,
    #[serde(rename = "nom_commune_complet")]
    fullname: String,
    #[serde(rename = "nom_departement")]
    departement: String,
    #[serde(rename = "code_departement")]
    code_departement: String,
}

impl Commune {
    fn sanitize(self) -> Self {
        Self {
            name: Self::sanitize_french(&self.name),
            fullname: Self::sanitize_french(&self.fullname),
            departement: Self::sanitize_french(&self.departement),
            code_departement: self.code_departement,
        }
    }

    fn sanitize_french(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'à' | 'â' | 'ä' => 'a',
                'è' | 'é' | 'ê' | 'ë' => 'e',
                'î' | 'ï' => 'u',
                'ô' | 'ö' | 'œ' => 'u',
                'û' | 'ü' | 'ù' => 'u',
                'ÿ' => 'y',
                'ç' => 'c',
                _ => c,
            })
            .join("")
    }
}
