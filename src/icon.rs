#![allow(dead_code)]

pub struct Icon(u32);

impl Icon {
    pub fn code(&self) -> String {
        self.0.to_string()
    }
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

pub const NORMAL: Icon = Icon(1899);
pub const COW: Icon = Icon(1553);
pub const MILK: Icon = Icon(1703);
pub const ANIMAL: Icon = Icon(1507);
pub const SHEEP: Icon = Icon(1774);
pub const CHICKEN: Icon = Icon(1545);
pub const TRACTOR: Icon = Icon(1883);
pub const WEAT: Icon = Icon(1587);

pub const ELEC: Icon = Icon(1660);
