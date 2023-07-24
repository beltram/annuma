use crate::cli::department::Department;
use console::style;
use spinoff::Spinner;

pub fn new_spinner(department: &Department, commune: &impl std::fmt::Display) -> Spinner {
    Spinner::new(
        spinoff::spinners::Aesthetic,
        format!(
            "Searching farmers in commune {} from departement {} ...",
            style(commune).cyan(),
            style(department).cyan(),
        ),
        spinoff::Color::Cyan,
    )
}
