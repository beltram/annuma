use crate::color::Color;
use crate::mymap::StyleKind;
use kml::types::*;
use kml::Kml;

const BALOON_TEXT: &str = "<![CDATA[<h3>$[name]</h3>]]>";
const HREF: &str = "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png";

pub fn new_style(kind: StyleKind, color: &Color, icon: &crate::icon::Icon) -> Kml {
    let id = format!("icon-{icon}-{color}-nodesc-{kind}");
    Kml::Style(Style {
        id: Some(id),
        balloon: Some(baloon_style()),
        icon: Some(icon_style(color)),
        label: Some(label_style()),
        line: None,
        poly: None,
        list: None,
        attrs: Default::default(),
    })
}

fn label_style() -> LabelStyle {
    LabelStyle {
        id: None,
        color: crate::color::BLACK.to_string(),
        color_mode: Default::default(),
        scale: 0.0,
        attrs: Default::default(),
    }
}

fn baloon_style() -> BalloonStyle {
    BalloonStyle {
        id: None,
        bg_color: None,
        text_color: crate::color::BLACK.to_string(),
        text: Some(BALOON_TEXT.to_string()),
        display: false,
        attrs: Default::default(),
    }
}

fn icon_style(color: &Color) -> IconStyle {
    IconStyle {
        id: None,
        scale: 1.0,
        heading: 0.0,
        hot_spot: Some(Vec2 {
            x: 32.0,
            y: 64.0,
            xunits: Units::Pixels,
            yunits: Units::InsetPixels,
        }),
        icon: Icon {
            href: HREF.to_string(),
            attrs: Default::default(),
        },
        color: color.to_string(),
        color_mode: Default::default(),
        attrs: Default::default(),
    }
}
