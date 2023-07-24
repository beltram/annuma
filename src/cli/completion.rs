use crate::cli::Annuma;
use clap::CommandFactory as _;

const ZSH_FILE: &str = "/usr/local/share/zsh/site-functions/_annuma";

pub fn generate_zsh_completion() -> anyhow::Result<()> {
    let mut file = std::fs::File::create(ZSH_FILE)?;
    clap_complete::generate(
        clap_complete::shells::Zsh,
        &mut Annuma::command(),
        "annuma",
        &mut file,
    );
    Ok(())
}
