use crate::model::Coord;

pub async fn find_geocode(
    address: &str,
    _commune: &str,
    _dept_name: &str,
    _code_postal: &str,
) -> anyhow::Result<Option<Coord>> {
    // https://nominatim.openstreetmap.org/search.php?city=Flee&state=Sarthe&country=France&postalcode=72500&polygon_geojson=1&format=jsonv2

    let client = reqwest::Client::builder().build()?;

    /*let resp = client
    .get("https://nominatim.openstreetmap.org/search.php")
    .query(&[
        ("city", commune),
        ("state", dept_name),
        ("country", "France"),
        ("postalcode", code_postal),
        // ("polygon_geojson", "1"),
        ("format", "geojson"),
    ])
    .header("user-agent", "Rust-Geocoding")
    .send()
    .await?;*/

    let resp = client
        .get("https://nominatim.openstreetmap.org/search")
        .query(&[("q", address), ("format", "geojson")])
        .header("user-agent", "Rust-Geocoding")
        .send()
        .await?;

    // println!("> {}", serde_json::to_string_pretty(&resp.json::<serde_json::Value>().await.unwrap()).unwrap());

    let body = resp
        .json::<geocoding::openstreetmap::OpenstreetmapResponse<f64>>()
        .await
        .unwrap();

    let coords = body
        .features
        .iter()
        .map(|r| r.geometry.coordinates)
        .map(|(lng, lat)| Coord { x: lng, y: lat })
        .collect::<Vec<_>>();

    if let Some(coord) = coords.first() {
        return Ok(Some(*coord));
    }

    Ok(None)
}
