use std::fs;
use std::io;
use std::path::Path;

pub fn setup(logging_path: &Path) -> Result<(), fern::InitError> {
    let level_filter = match std::env::var("XI_LOG") {
        Ok(level) => match level.to_lowercase().as_ref() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            _ => log::LevelFilter::Info,
        },
        // Default to info
        Err(_) => log::LevelFilter::Info,
    };

    create_log_directory(logging_path)?;

    let fern_dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message,
            ))
        })
        .level(level_filter)
        .chain(fern::log_file(logging_path)?);

    // Start fern
    fern_dispatch.apply()?;
    info!("Logging with fern is set up to level {}", level_filter);
    info!("Writing logs to: {}", logging_path.display());

    Ok(())
}

/// This function tries to create the parent directories for a file
///
/// It wraps around the `parent()` function of `Path` which returns an `Option<&Path>` and
/// `fs::create_dir_all` which returns an `io::Result<()>`.
///
/// This allows you to use `?`/`try!()` to create the dir and you recive the additional custom error for when `parent()`
/// returns nothing.
///
/// # Errors
/// This can return an `io::Error` if `fs::create_dir_all` fails or if `parent()` returns `None`.
/// See `Path`'s `parent()` function for more details.
/// # Examples
/// ```
/// use std::path::Path;
/// use std::ffi::OsStr;
///
/// let path_with_file = Path::new("/some/directory/then/file");
/// assert_eq!(Some(OsStr::new("file")), path_with_file.file_name());
/// assert_eq!(create_log_directory(path_with_file).is_ok(), true);
///
/// let path_with_other_file = Path::new("/other_file");
/// assert_eq!(Some(OsStr::new("other_file")), path_with_other_file.file_name());
/// assert_eq!(create_log_directory(path_with_file).is_ok(), true);
///
/// // Path that is just the root or prefix:
/// let path_without_file = Path::new("/");
/// assert_eq!(None, path_without_file.file_name());
/// assert_eq!(create_log_directory(path_without_file).is_ok(), false);
/// ```
fn create_log_directory(path_with_file: &Path) -> io::Result<()> {
    let log_dir = path_with_file.parent().ok_or_else(|| io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "Unable to get the parent of the following Path: {}, Your path should contain a file name",
            path_with_file.display(),
        ),
    ))?;
    fs::create_dir_all(log_dir)?;
    Ok(())
}
