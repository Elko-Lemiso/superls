use clap::{ App, Arg };
use std::path::Path;
use walkdir::{ DirEntry, WalkDir };
use colored::*;

fn main() {
    let matches = App::new("superls")
        .arg(
            Arg::with_name("dir")
                .long("dir")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true)
                .help("The directory to recurse through")
        )
        .arg(
            Arg::with_name("ignore")
                .long("ignore")
                .takes_value(true)
                .multiple(true)
                .allow_invalid_utf8(true)
                .help("Files and filetypes to ignore")
        )
        .arg(
            Arg::with_name("extension")
                .long("extension")
                .takes_value(true)
                .allow_invalid_utf8(true)
                .help("Only return files with a certain extension")
        )
        .get_matches();

    let dir = matches.value_of_os("dir").unwrap();
    let path = Path::new(dir);

    let ignore: Vec<String> = matches
        .values_of_os("ignore")
        .unwrap_or_default()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();

    let extension = matches.value_of_os("extension").map(|s| s.to_string_lossy().into_owned());

    walk_and_print(&path, 0, &ignore, extension.as_deref());
}

fn walk_and_print(path: &Path, depth: usize, ignore: &[String], extension: Option<&str>) {
    if path.is_dir() {
        let entries: Vec<_> = WalkDir::new(path)
            .min_depth(1)
            .into_iter()
            .filter_entry(|e| should_walk(e, ignore, extension))
            .filter_map(|e| e.ok())
            .collect();

        let should_print = entries
            .iter()
            .any(|entry| {
                entry.file_type().is_file() &&
                    extension.map_or(true, |ext| {
                        entry.file_name().to_string_lossy().ends_with(ext)
                    })
            });

        if should_print {
            let depth_indicator = " ".repeat(depth * 4);
            let dir_name = path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("."))
                .to_string_lossy();
            println!("{}\\_ {}", depth_indicator, dir_name.blue());

            for entry in &entries {
                if entry.file_type().is_dir() {
                    walk_and_print(entry.path(), depth + 1, ignore, extension);
                } else if
                    entry.file_type().is_file() &&
                    extension.map_or(true, |ext| {
                        entry.file_name().to_string_lossy().ends_with(ext)
                    })
                {
                    let depth_indicator = " ".repeat((depth + 1) * 4);
                    println!(
                        "{}{} {}",
                        depth_indicator,
                        "\\_".blue(),
                        entry.file_name().to_string_lossy().green()
                    );
                }
            }
        }
    }
}


fn should_walk(entry: &DirEntry, ignore: &[String], extension: Option<&str>) -> bool {
    let file_name = entry.file_name().to_string_lossy().to_string();

    if ignore.iter().any(|i| file_name == *i) {
        return false;
    }

    if let Some(parent) = entry.path().parent() {
        if ignore.iter().any(|i| parent.ends_with(i)) {
            return false;
        }
    }

    if let Some(ext) = extension {
        if entry.file_type().is_file() && !file_name.ends_with(ext) {
            return false;
        }
    }

    true
}

