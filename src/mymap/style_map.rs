use crate::color::Color;
use crate::mymap::StyleKind;
use kml::types::*;
use kml::Kml;

pub fn new_style_map(color: &Color, icon: &crate::icon::Icon) -> Kml {
    let style_url = format!("#icon-{icon}-{color}-nodesc");
    let normal = Pair {
        key: StyleKind::Normal.to_string(),
        style_url: format!("{style_url}-{}", StyleKind::Normal),
        attrs: Default::default(),
    };
    let highlight = Pair {
        key: StyleKind::Highlight.to_string(),
        style_url: format!("{style_url}-{}", StyleKind::Highlight),
        attrs: Default::default(),
    };

    let id = format!("icon-{icon}-{color}-nodesc");
    Kml::StyleMap(StyleMap {
        id: Some(id),
        pairs: vec![normal, highlight],
        attrs: Default::default(),
    })
}
