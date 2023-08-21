use crate::model::Coord;

pub async fn find_geocode(address: &str) -> anyhow::Result<Option<Coord>> {
    let client = reqwest::Client::builder().build()?;
    let resp = client
        .get("https://nominatim.openstreetmap.org/search")
        .query(&[("q", address), ("format", "geojson")])
        .header("user-agent", "Rust-Geocoding")
        .send()
        .await?;
    let body = resp
        .json::<geocoding::openstreetmap::OpenstreetmapResponse<f64>>()
        .await?;

    let coords = body
        .features
        .iter()
        .map(|r| r.geometry.coordinates)
        .map(|(lng, lat)| Coord { x: lng, y: lat })
        .collect::<Vec<_>>();

    if let Some(coord) = coords.first() {
        return Ok(Some(*coord));
    }
    println!("Could not geocode {address}");
    Ok(None)
}
