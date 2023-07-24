use crate::model::{Coord, Farmer};
use itertools::Itertools;
use kml::{types::*, Kml};
use std::path::PathBuf;

pub mod style;
pub mod style_map;

const KML_PREFIX: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
"#;
const KML_SUFFIX: &str = "</kml>";

#[derive(Debug, Copy, Clone, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum StyleKind {
    Normal,
    Highlight,
}

pub fn create_klm(
    path: &PathBuf,
    commune: &impl ToString,
    farmers: Vec<Farmer>,
) -> anyhow::Result<()> {
    let mut doc_elements = vec![];
    doc_elements.push(name(commune));
    doc_elements.push(description(commune));
    doc_elements.extend(build_styles(&farmers));

    let coord_size = farmers.len();
    let mut points = farmers
        .iter()
        .filter_map(|f| f.coord.zip(Some(f)))
        .try_fold(Vec::with_capacity(coord_size), |acc, (coord, f)| {
            build_placemark(acc, coord, f)
        })?;
    points.push(layer_name(commune));
    let folder = Kml::<f64>::Folder {
        elements: points,
        attrs: Default::default(),
    };
    doc_elements.push(folder);
    let doc: Kml = Kml::Document {
        attrs: Default::default(),
        elements: doc_elements,
    };
    let doc = format!("{KML_PREFIX}{doc}{KML_SUFFIX}");
    std::fs::write(path, doc)?;
    Ok(())
}

fn build_styles(farmers: &[Farmer]) -> Vec<Kml> {
    farmers
        .iter()
        .map(|f| f.job)
        .unique()
        .flat_map(|j| j.styles())
        .collect::<Vec<_>>()
}

fn build_placemark(mut acc: Vec<Kml>, coord: Coord, f: &Farmer) -> anyhow::Result<Vec<Kml>> {
    let point = Point {
        coord: kml::types::Coord {
            x: coord.x,
            y: coord.y,
            z: None,
        },
        extrude: false,
        altitude_mode: Default::default(),
        attrs: Default::default(),
    };

    let style_ref = Element {
        name: "styleUrl".to_string(),
        attrs: Default::default(),
        children: Default::default(),
        content: Some(f.job.id()),
    };

    let placemark = Kml::Placemark(Placemark {
        name: Some(f.name()),
        description: Some(f.description()),
        geometry: Some(Geometry::Point(point)),
        attrs: Default::default(),
        children: vec![style_ref],
    });

    acc.push(placemark);
    Ok(acc)
}

fn name(commune: &impl ToString) -> Kml {
    Kml::Element(Element {
        name: "name".to_string(),
        attrs: Default::default(),
        children: Default::default(),
        content: Some(commune.to_string()),
    })
}

fn layer_name(commune: &impl ToString) -> Kml {
    Kml::Element(Element {
        name: "name".to_string(),
        attrs: Default::default(),
        children: Default::default(),
        content: Some(commune.to_string()),
    })
}

fn description(commune: &impl ToString) -> Kml {
    Kml::Element(Element {
        name: "description".to_string(),
        attrs: Default::default(),
        children: Default::default(),
        content: Some(commune.to_string()),
    })
}
