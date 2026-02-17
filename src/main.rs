use std::process::Command;

use clap::Parser;

use crate::{
    cli::CliArgs,
    utils::{altered_filename, format_status_error},
};

mod cli;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    let margin_arg = format!("{}sec", args.margin);

    let status = Command::new("auto-editor")
        .arg(&args.input)
        .arg("--margin")
        .arg(margin_arg)
        .status()?;

    if !status.success() {
        return Err(format_status_error("auto-editor", status).into());
    }

    let altered = altered_filename(&args.input)?;

    let setpts = format!("setpts=PTS/{}", args.speed);
    let atempo = format!("atempo={}", args.speed);

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&altered)
        .arg("-filter:v")
        .arg(setpts)
        .arg("-filter:a")
        .arg(atempo)
        .arg(&args.output)
        .status()?;

    if !status.success() {
        return Err(format_status_error("ffmpeg", status).into());
    }

    Ok(())
}
