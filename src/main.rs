// External crate imports
extern crate clap;
extern crate colored;
extern crate regex;
extern crate walkdir;

// Standard and external library imports
use clap::{App, Arg};
use colored::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

// Prints the directory or file entry with an icon and color.
fn print_entry(entry: &DirEntry, depth: usize) {
    if entry.file_type().is_dir() {
        println!(
            "{}{} {}",
            "  ".repeat(depth),
            "🗂".blue(),
            entry.file_name().to_string_lossy().blue()
        );
    } else {
        println!(
            "{}{} {}",
            "  ".repeat(depth),
            "📄".truecolor(255, 105, 180),
            entry.file_name().to_string_lossy().truecolor(255, 105, 180)
        );
    }
}

// Determines if a directory is empty or only contains ignored/filtered items.
fn is_directory_empty_or_filtered<P: AsRef<Path>>(
    path: P,
    desired_extensions: &Option<HashSet<String>>,
    ignored_extensions: &HashSet<String>,
    ignored_dirs: &HashSet<String>,
) -> bool {
    let walker = WalkDir::new(path).min_depth(1).max_depth(1).into_iter();
    for entry in walker.filter_map(Result::ok) {
        if !should_skip(&entry, desired_extensions, ignored_extensions, ignored_dirs) {
            return false; // Directory contains a non-ignored item
        }
    }
    true // All items in the directory are ignored or the directory is empty
}

// Determines if a given entry should be skipped based on set criteria.
fn should_skip(
    entry: &DirEntry,
    desired_extensions: &Option<HashSet<String>>,
    ignored_extensions: &HashSet<String>,
    ignored_dirs: &HashSet<String>,
) -> bool {
    is_ignored_dir(entry, ignored_dirs)
        || is_ignored_extension(entry, ignored_extensions)
        || !is_desired_extension(entry, desired_extensions)
}

// Checks if the directory entry should be ignored.
fn is_ignored_dir(entry: &DirEntry, ignored_dirs: &HashSet<String>) -> bool {
    entry.file_type().is_dir() && ignored_dirs.contains(entry.file_name().to_str().unwrap())
}

// Checks if the file entry with the given extension should be ignored.
fn is_ignored_extension(entry: &DirEntry, ignored_extensions: &HashSet<String>) -> bool {
    entry.path().extension().map_or(false, |ext| {
        ignored_extensions.contains(ext.to_str().unwrap())
    })
}

// Checks if the file entry with the given extension should be displayed.
fn is_desired_extension(entry: &DirEntry, desired_extensions: &Option<HashSet<String>>) -> bool {
    if let Some(desired_extensions) = desired_extensions {
        if entry.file_type().is_file() {
            return entry.path().extension().map_or(false, |ext| {
                desired_extensions.contains(ext.to_str().unwrap())
            });
        }
    }
    true
}

// Searches for a specific pattern within a file and prints matching lines.
fn grep_file(path: &Path, pattern: &Regex) -> io::Result<bool> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if pattern.is_match(&line) {
            println!("{}:{}: {}", path.display(), index + 1, line);
            return Ok(true);
        }
    }
    Ok(false)
}

// Main recursive function to list and potentially grep directories and files.
fn list_directory<P: AsRef<Path>>(
    path: P,
    depth: usize,
    desired_extensions: &Option<HashSet<String>>,
    ignored_extensions: &HashSet<String>,
    ignored_dirs: &HashSet<String>,
    pattern: &Option<Regex>,
) {
    if path.as_ref().is_dir()
        && is_directory_empty_or_filtered(
            &path,
            desired_extensions,
            ignored_extensions,
            ignored_dirs,
        )
    {
        return; // Don't process or print this directory further if it's empty or filtered out
    }

    let walker = WalkDir::new(&path).min_depth(1).max_depth(1).into_iter();
    for entry in walker
        .filter_map(Result::ok)
        .filter(|e| !should_skip(e, desired_extensions, ignored_extensions, ignored_dirs))
    {
        print_entry(&entry, depth);

        if entry.file_type().is_file() {
            if let Some(ref pattern) = pattern {
                if let Err(err) = grep_file(entry.path(), pattern) {
                    eprintln!("Error reading {}: {}", entry.path().display(), err);
                }
            }
        } else if entry.file_type().is_dir() {
            list_directory(
                entry.path(),
                depth + 1,
                desired_extensions,
                ignored_extensions,
                ignored_dirs,
                pattern,
            );
        }
    }
}

// Program entry point: handles argument parsing and initiates directory listing.
fn main() {
    // Define the CLI arguments using clap
    let matches = App::new("superls")
        .version("1.0")
        .about("Outputs the structure of a repository with icons and offers grep functionality.")
        .arg(
            Arg::with_name("DIRECTORY")
                .help("The directory to parse")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("ignore-extensions")
                .short("e")
                .long("ignore-extensions")
                .multiple(true)
                .takes_value(true)
                .help("File extensions to ignore"),
        )
        .arg(
            Arg::with_name("ignore-dirs")
                .short("d")
                .long("ignore-dirs")
                .multiple(true)
                .takes_value(true)
                .help("Directories to ignore"),
        )
        .arg(
            Arg::with_name("only-extensions")
                .short("o")
                .long("only-extensions")
                .multiple(true)
                .takes_value(true)
                .help("Only display files with these extensions"),
        )
        .arg(
            Arg::with_name("grep")
                .short("g")
                .long("grep")
                .takes_value(true)
                .help("Pattern to search for within files"),
        )
        .get_matches();

    // Extract values from parsed CLI arguments
    let path = matches.value_of("DIRECTORY").unwrap();
    let ignored_extensions: HashSet<_> = matches
        .values_of("ignore-extensions")
        .unwrap_or_default()
        .map(String::from)
        .collect();
    let ignored_dirs: HashSet<_> = matches
        .values_of("ignore-dirs")
        .unwrap_or_default()
        .map(String::from)
        .collect();
    let desired_extensions: Option<HashSet<_>> = matches
        .values_of("only-extensions")
        .map(|vals| vals.map(String::from).collect());
    let grep_pattern = matches
        .value_of("grep")
        .map(|pattern| Regex::new(pattern).expect("Invalid regex pattern"));

    // Begin directory listing (and potentially grep) based on provided parameters
    list_directory(
        path,
        0,
        &desired_extensions,
        &ignored_extensions,
        &ignored_dirs,
        &grep_pattern,
    );
}
