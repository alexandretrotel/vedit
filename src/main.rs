use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input video file
    input: PathBuf,

    /// Margin in seconds (passed to auto-editor)
    #[arg(long)]
    margin: f32,

    /// Playback speed (e.g. 1.25, 1.5, 2.0)
    #[arg(long)]
    speed: f32,

    /// Output filename
    #[arg(long)]
    output: PathBuf,
}

/// Runs auto-editor, ffmpeg, and outputs the altered video.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let margin_arg = format!("{}sec", args.margin);

    let status = Command::new("auto-editor")
        .arg(&args.input)
        .arg("--margin")
        .arg(margin_arg)
        .status()?;

    if !status.success() {
        return Err("auto-editor failed".into());
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
        return Err("ffmpeg failed".into());
    }

    Ok(())
}

/// Turns `video.mp4` into `video_ALTERED.mp4`
fn altered_filename(input: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let stem = input
        .file_stem()
        .ok_or("Invalid input filename")?
        .to_string_lossy();

    let ext = input
        .extension()
        .ok_or("Missing file extension")?
        .to_string_lossy();

    let altered = input.with_file_name(format!("{stem}_ALTERED.{ext}"));
    Ok(altered)
}
