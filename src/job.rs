use crate::color::Color;
use crate::icon::Icon;
use crate::mymap::style::new_style;
use crate::mymap::style_map::new_style_map;
use crate::mymap::StyleKind;
use crate::{color, icon};
use anyhow::anyhow;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Job {
    ElevageAutresBovinsEtBuffles,
    ElevageVacheLaitiere,
    CultureElevageAssocies,
    ElevageOvinsCaprins,
    ElevageVolailles,
    ElevageDePorcins,
    CultureCereales,
}

impl Job {
    pub fn id(&self) -> String {
        format!("#icon-{}-{}-nodesc", self.icon(), self.color())
    }
    
    pub fn styles(&self) -> Vec<kml::Kml> {
        let color = self.color();
        let icon = self.icon();

        let normal_style = new_style(StyleKind::Normal, &color, &icon);
        let highlight_style = new_style(StyleKind::Highlight, &color, &icon);
        let style_map = new_style_map(&color, &icon);

        vec![normal_style, highlight_style, style_map]
    }

    pub fn color(&self) -> Color {
        match self {
            Job::ElevageAutresBovinsEtBuffles => color::DARK_RED,
            Job::ElevageVacheLaitiere => color::DARK_BLUE,
            Job::CultureElevageAssocies => color::PURPLE,
            Job::ElevageOvinsCaprins => color::BORDEAUX,
            Job::ElevageVolailles => color::GREY,
            Job::ElevageDePorcins => color::YELLOW,
            Job::CultureCereales => color::TURQUOISE,
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Job::ElevageAutresBovinsEtBuffles => icon::COW,
            Job::ElevageVacheLaitiere => icon::MILK,
            Job::CultureElevageAssocies => icon::ANIMAL,
            Job::ElevageOvinsCaprins => icon::SHEEP,
            Job::ElevageVolailles => icon::CHICKEN,
            Job::ElevageDePorcins => icon::TRACTOR,
            Job::CultureCereales => icon::WEAT,
        }
    }
}

impl FromStr for Job {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "Élevage d'autres bovins et de buffles" => Ok(Self::ElevageAutresBovinsEtBuffles),
            "Élevage de vaches laitières" => Ok(Self::ElevageVacheLaitiere),
            "Culture et élevage associés" => Ok(Self::CultureElevageAssocies),
            "Élevage d'ovins et de caprins" => Ok(Self::ElevageOvinsCaprins),
            "Élevage de volailles" => Ok(Self::ElevageVolailles),
            "Élevage de porcins" => Ok(Self::ElevageDePorcins),
            "Culture de céréales (à l'exception du riz), de légumineuses et de graines oléagineuses" => Ok(Self::CultureCereales),
            _ => Err(anyhow!("Unknown job '{s}'")),
        }
    }
}

impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let job = match self {
            Job::ElevageAutresBovinsEtBuffles =>"Élevage d'autres bovins et de buffles",
            Job::ElevageVacheLaitiere =>"Élevage de vaches laitières",
            Job::CultureElevageAssocies =>"Culture et élevage associés",
            Job::ElevageOvinsCaprins =>"Élevage d'ovins et de caprins",
            Job::ElevageVolailles =>"Élevage de volailles",
            Job::ElevageDePorcins =>"Élevage de porcins",
            Job::CultureCereales =>"Culture de céréales (à l'exception du riz), de légumineuses et de graines oléagineuses",
        };
        write!(f, "{job}")
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
