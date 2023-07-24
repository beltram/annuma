use crate::job::Job;

/*#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Commune {
    Guerande,
    Vallet,
    SaintNazaire,
}

impl std::fmt::Display for Commune {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", heck::AsKebabCase(format!("{self:?}")))
    }
}*/

#[derive(Debug, Clone)]
pub struct Farmer {
    pub title: String,
    pub label: String,
    pub address: String,
    pub job: Job,
    pub coord: Option<Coord>,
}

impl Farmer {
    pub fn name(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        format!(
            "Name: {}\nJob: {}\nAddress: {}",
            self.title, self.label, self.address
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}
