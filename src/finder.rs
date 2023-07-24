use crate::model::{Farmer, Town};
use crate::scrap::scrap_farmer;

// https://www.annuaire-mairie.fr/entreprise-agriculture-sylviculture-peche-guerande.html
const HOST: &str = "www.annuaire-mairie.fr";
const PATH: &str = "entreprise-agriculture-sylviculture-peche";

fn build_uri(town: &Town) -> anyhow::Result<reqwest::Url> {
    let uri = format!("https://{HOST}/{PATH}-{town}.html");
    Ok(reqwest::Url::parse(&uri)?)
}

pub async fn find_farmer(town: &Town) -> anyhow::Result<Vec<Farmer>> {
    let response = reqwest::get(build_uri(town)?).await?;
    let html = response.text().await?;
    let farmers = scrap_farmer(html)?;
    Ok(farmers)
}
