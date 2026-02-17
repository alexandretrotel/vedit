use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    pub input: PathBuf,

    #[arg(long)]
    pub margin: f32,

    #[arg(long)]
    pub speed: f32,

    #[arg(long)]
    pub output: PathBuf,
}
