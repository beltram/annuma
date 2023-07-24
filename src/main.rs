use crate::finder::find_farmer;
use crate::model::Town;

mod finder;
mod model;
mod scrap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let farmers = find_farmer(&Town::Guerande).await?;
    for f in farmers {
        println!("> {:?}", f);
    }
    Ok(())
}
