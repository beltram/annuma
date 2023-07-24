use crate::cli::department::Department;
use crate::cli::{Annuma, Commands};
use crate::finder::geocode_farmers;
use crate::{finder::find_farmer, mymap::create_klm};
use anyhow::anyhow;
use console::style;
use std::path::PathBuf;

mod cli;
mod color;
mod finder;
mod geocode;
mod icon;
mod job;
mod model;
mod mymap;
mod scrap;
mod spinner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_eyre::install().map_err(|_| anyhow!("Error with eyre setup"))?;
    use clap::Parser as _;
    match Annuma::parse().cmd {
        Commands::Map { department } => my_map(department).await?,
        Commands::PosteSource => println!("In progress..."),
        Commands::Completion {} => cli::completion::generate_zsh_completion()?,
    }
    Ok(())
}

async fn my_map(department: Department) -> anyhow::Result<()> {
    let commune = department.commune();

    let mut spinner = spinner::new_spinner(&department, &commune);
    tokio::time::sleep(core::time::Duration::from_secs(1)).await;

    let farmers = find_farmer(&commune, department.number(), None, None).await?;

    let farmers_size = farmers.len();

    if farmers_size == 0 {
        spinner.fail(&format!(
            "Could not find any farmer in {}",
            style(commune).cyan()
        ));
        return Err(anyhow!(""));
    }

    spinner.update_text(format!(
        "Found {} farmers. Now geocoding them...",
        style(farmers_size).cyan()
    ));

    let farmers = geocode_farmers(farmers, &mut spinner).await?;

    let filename = format!("{commune}.kml");
    let path = PathBuf::from(&filename);
    std::fs::File::create(&path)?;
    create_klm(&path, &commune, farmers)?;

    let fullpath = std::fs::canonicalize(path)?;
    spinner.success(&format!(
        "Finished 🎊\nNow redirecting you to MyMaps to import {}",
        style(format!("{fullpath:?}")).cyan()
    ));

    tokio::time::sleep(core::time::Duration::from_secs(2)).await;

    webbrowser::open("https://www.google.com/maps/d/u/0/home")?;

    Ok(())
}
