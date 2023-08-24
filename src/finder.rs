use crate::{geocode::find_geocode, job::Job, model::Farmer, scrap::scrap_farmer};
use console::style;
use spinoff::Spinner;
use std::sync::{Arc, Mutex};

const HOST: &str = "www.annuaire-mairie.fr";
const PATH1: &str = "entreprise-agriculture-sylviculture-peche";

fn build_uris(commune: &impl std::fmt::Display, departement: u32) -> anyhow::Result<reqwest::Url> {
    let commune = heck::AsKebabCase(format!("{commune}")).to_string();
    let uri = format!("https://{HOST}/{PATH1}-{commune}-{departement}.html");
    Ok(reqwest::Url::parse(&uri)?)
}

pub async fn find_farmer(
    commune: &impl std::fmt::Display,
    departement_number: u32,
    departement_name: String,
    only_jobs: Option<Vec<Job>>,
    exclude_jobs: Option<Vec<Job>>,
) -> anyhow::Result<Vec<Farmer>> {
    let response = reqwest::get(build_uris(commune, departement_number)?).await?;
    let html = response.text().await?;
    let farmers = scrap_farmer(html)?;

    let mut farmers = if let Some(oj) = only_jobs {
        farmers
            .into_iter()
            .filter(|f| oj.contains(&f.job))
            .collect()
    } else if let Some(ej) = exclude_jobs {
        farmers
            .into_iter()
            .filter(|f| !ej.contains(&f.job))
            .collect()
    } else {
        farmers
    };

    for f in farmers.iter_mut() {
        let addr = &f.address;
        // f.address = format!("{departement_name}, {commune}, {addr}");
        f.address = format!("{addr}, {commune}, {departement_name}");
        // f.address = format!("{addr}");
    }

    Ok(farmers)
}

pub async fn geocode_farmers(
    farmers: Vec<Farmer>,
    s: &mut Spinner,
    commune: &str,
    dept_name: &str,
    code_postal: &str,
) -> anyhow::Result<Vec<Farmer>> {
    use futures::stream::{StreamExt as _, TryStreamExt as _};

    let n = farmers.len();
    let i = Arc::new(Mutex::new(0u32));
    let parallel = usize::MAX;
    let s = Arc::new(Mutex::new(s));
    let farmers = futures_util::stream::iter(farmers.into_iter().map(|mut f| async {
        f.coord = find_geocode(&f.address, commune, dept_name, code_postal)
            .await
            .ok()
            .flatten();
        if let Some((mut s, mut i)) = s.lock().ok().zip(i.lock().ok()) {
            *i += 1;
            s.update_text(format!(
                "Geocoded {}/{} farmers...",
                style(i).cyan(),
                style(n).cyan()
            ));
        }
        anyhow::Ok(f)
    }))
    .buffer_unordered(parallel)
    .try_collect::<Vec<Farmer>>()
    .await?;

    Ok(farmers)
}
