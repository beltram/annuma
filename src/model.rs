use anyhow::anyhow;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Town {
    Guerande,
    SaintNazaire,
}

impl std::fmt::Display for Town {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", heck::AsKebabCase(format!("{self:?}")))
    }
}

#[derive(Debug, Clone)]
pub struct Farmer {
    pub title: String,
    pub label: String,
    pub address: String,
    pub job: Job,
}

#[derive(Clone, Copy)]
pub enum Job {
    ElevageChevaux,
    ElevageAutresBovinsEtBuffles,
    ElevageVacheLaitiere,
    CultureElevageAssocies,
    ElevageAutresAnimaux,
    ElevageOvinsCaprins,
    CultureCerealesLegumineusesGrainesOleagineuses,
}

impl FromStr for Job {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "Élevage de chevaux et d'autres équidés" => Ok(Self::ElevageChevaux),
            "Élevage d'autres bovins et de buffles" => Ok(Self::ElevageAutresBovinsEtBuffles),
            "Élevage de vaches laitières" => Ok(Self::ElevageVacheLaitiere),
            "Culture et élevage associés" => Ok(Self::CultureElevageAssocies),
            "Élevage d'autres animaux" => Ok(Self::ElevageAutresAnimaux),
            "Élevage d'ovins et de caprins" => Ok(Self::ElevageOvinsCaprins),
            "Culture de céréales (à l'exception du riz), de légumineuses et de graines oléagineuses" => Ok(Self::CultureCerealesLegumineusesGrainesOleagineuses),
            _ => Err(anyhow!("Unknown job '{s}'")),
        }
    }
}

impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let job = match self {
            Job::ElevageChevaux => "Élevage de chevaux et d'autres équidés",
            Job::ElevageAutresBovinsEtBuffles =>"Élevage d'autres bovins et de buffles",
            Job::ElevageVacheLaitiere =>"Élevage de vaches laitières",
            Job::CultureElevageAssocies =>"Culture et élevage associés",
            Job::ElevageAutresAnimaux =>"Élevage d'autres animaux",
            Job::ElevageOvinsCaprins =>"Élevage d'ovins et de caprins",
            Job::CultureCerealesLegumineusesGrainesOleagineuses =>"Culture de céréales (à l'exception du riz), de légumineuses et de graines oléagineuses",
        };
        write!(f, "{job}")
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
