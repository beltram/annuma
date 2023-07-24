use crate::{geocode::find_geocode, job::Job, model::Farmer, scrap::scrap_farmer};
use console::style;
use spinoff::Spinner;
use std::sync::{Arc, Mutex};

const HOST: &str = "www.annuaire-mairie.fr";
const PATH: &str = "entreprise-agriculture-sylviculture-peche";

fn build_uri(commune: &impl std::fmt::Display, departement: u32) -> anyhow::Result<reqwest::Url> {
    let uri = format!("https://{HOST}/{PATH}-{commune}-{departement}.html");
    Ok(reqwest::Url::parse(&uri)?)
}

pub async fn find_farmer(
    commune: &impl std::fmt::Display,
    departement: u32,
    only_jobs: Option<Vec<Job>>,
    exclude_jobs: Option<Vec<Job>>,
) -> anyhow::Result<Vec<Farmer>> {
    let response = reqwest::get(build_uri(commune, departement)?).await?;
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
        f.address = format!("{}, {commune}", &f.address);
    }

    Ok(farmers)
}

pub async fn geocode_farmers(farmers: Vec<Farmer>, s: &mut Spinner) -> anyhow::Result<Vec<Farmer>> {
    use futures::stream::{StreamExt as _, TryStreamExt as _};

    let n = farmers.len();
    let i = Arc::new(Mutex::new(0u32));
    let parallel = usize::MAX;
    let s = Arc::new(Mutex::new(s));
    let farmers = futures_util::stream::iter(farmers.into_iter().map(|mut f| async {
        f.coord = find_geocode(&f.address).await.ok().flatten();
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
