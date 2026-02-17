use std::path::{Path, PathBuf};

pub(crate) fn altered_filename(input: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
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

pub(crate) fn format_status_error(cmd: &str, status: std::process::ExitStatus) -> String {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;

        if let Some(code) = status.code() {
            format!("{cmd} failed with exit code {code}")
        } else if let Some(signal) = status.signal() {
            format!("{cmd} terminated by signal {signal}")
        } else {
            format!("{cmd} failed with unknown status")
        }
    }

    #[cfg(not(unix))]
    {
        if let Some(code) = status.code() {
            format!("{cmd} failed with exit code {code}")
        } else {
            format!("{cmd} failed with unknown status")
        }
    }
}
