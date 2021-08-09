use file_diff::*;
use std::path::PathBuf;
use std::thread::sleep;
use std::{env, fs, process, time};

#[path = "codes.rs"]
mod codes;
use codes::*;

#[path = "options.rs"]
mod options;
use options::*;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut options = Options::new(&env::args().collect());

    loop {
        sleep(time::Duration::from_secs(options.interval() as u64));
        match move_content(&mut options) {
            Ok(()) => continue,
            Err(e) => quit(&EC::IOGeneric, &mut options, e.to_string()),
        }
    }
}

fn move_content(options: &mut Options) -> Result<(), std::io::Error> {
    // Create a list of directories to be moved. Add to this list recursively.
    let mut directories = Vec::new();
    // The user supplied root directory will be the first directory added.
    directories.push(options.from_dir());

    // Get the number of components to where the root directory is located. We can skip this number
    // of components to deal with paths relative to the given root directory.
    let from_dir_parent_level = options.from_dir().components().count();

    // Subdirectories that will need to be removed if /copy is not specified.
    let mut sub_dirs = Vec::<PathBuf>::new();

    // Recursively process the directories added to the list.
    while let Some(working_path) = directories.pop() {
        // Paths need to be validated every iteration at the beginning of the directories loop, and
        // at the beginning of the files loop.
        options.validate_paths();

        // Use 'from_dir_parent_level' to make a relative path like "..\child\dir"
        let child_dir: PathBuf = working_path
            .components()
            .skip(from_dir_parent_level)
            .collect();

        // Create the 'to' directory path.
        let to_dir = if child_dir.components().count() == 0 {
            options.to_dir()
        } else {
            options.to_dir().join(&child_dir)
        };

        // Create the 'to' directory on the file system.
        if fs::metadata(&to_dir).is_err() {
            match fs::create_dir_all(&to_dir) {
                Ok(()) => {}
                Err(e) => quit(&EC::ToDirCreation, options, e.to_string()),
            };
        }

        // Attempt to copy all of the files within the current directory. If /copy is specified, the
        // files will not be deleted after successful copies. Add any sub directories to our
        // directories list to be processed.
        for entry in fs::read_dir(&working_path)? {
            options.validate_paths(); // Also needed here.
            let entry = entry?;
            let from_path = entry.path();
            if from_path.is_dir() {
                // Is a directory.
                directories.push(from_path.clone());
                if !options.copy() {
                    sub_dirs.push(from_path);
                }
            } else {
                // Is a file.
                if let Some(filename) = from_path.file_name() {
                    let to_path = to_dir.join(filename);
                    // This function name isn't good. Should read, if not identical -> do something.
                    // Don't want to keep copying files that are identical.
                    if !diff(from_path.to_str().unwrap(), to_path.to_str().unwrap()) {
                        if to_path.exists() {
                            // The file exists, but contain different contents. Refer to the /o option.
                            if options.overwrite() {
                                copy_file(&from_path, &to_path, options);
                            }
                        } else {
                            copy_file(&from_path, &to_path, options);
                        }
                    }
                }
            }
        }

        // Write all buffered logs to the file system.
        options.logger().flush_logs();
    }

    // Remove empty directories.
    if !sub_dirs.is_empty() {
        sub_dirs.reverse();
        for dir in sub_dirs {
            let dir_str = dir.to_str().unwrap();
            match fs::remove_dir(&dir) {
                Ok(()) => options.logger().remove_flagged_path(dir),
                Err(e) => options.logger().log_error(
                    dir.clone(),
                    format!(
                        "Attempted to remove the directory at \"{}\". Error Message: {}",
                        dir_str, e
                    ),
                ),
            }
        }
    }

    Ok(())
}

/**
 * Attempts to copy a file. Any errors are logged to a buffer inside of Options::Logger. If /copy is
 * specified, try to delete the original afterwards.
 */
fn copy_file(from_path: &PathBuf, to_path: &PathBuf, options: &mut Options) {
    let from_path_str = from_path.to_str().unwrap();
    match fs::copy(from_path, to_path) {
        Ok(_file_size) => {
            if !options.copy() {
                // Copy complete, try to remove it and then log it.
                remove_file(options, from_path, to_path);
            } else {
                // Copy complete, log it.
                options
                    .logger()
                    .log_transfer(from_path_str, to_path.to_str().unwrap(), true);
            }
        }
        Err(e) => options.logger().log_error(
            from_path.clone(),
            format!(
                "Attempted to copy the file at \"{}\". Error Message: {}",
                from_path_str, e
            ),
        ),
    }
}

/**
 * Attempts to remove a file. This should be used after a successful copy.
 * If any errors are encountered, log it and keep running.
 */
fn remove_file(options: &mut Options, from_path: &PathBuf, to_path: &PathBuf) {
    let from_path_str = from_path.to_str().unwrap();
    match fs::remove_file(from_path) {
        Ok(()) => options
            .logger()
            .log_transfer(from_path_str, to_path.to_str().unwrap(), false),
        Err(e) => options.logger().log_error(
            from_path.clone(),
            format!(
                "Attempted to remove the file at \"{}\". Error Message: {}",
                from_path_str, e
            ),
        ),
    }
}

/**
 * Prints the programs about info, similar to the README documentation.
 * TODO: Add /log and /log:o (logs in file structure based on year)
 */
fn print_about_info() {
    println!("--INFO--{}---------------------------------------------------------------------------------------", VERSION);
    println!("mover.exe <from-directory> <to-directory> [/i <interval-in-seconds>] [/copy]                        ");
    println!("                                                                                                    ");
    println!("<from-directory>              Required - The directory to check and move content from.              ");
    println!("                                                                                                    ");
    println!("<to-directory>                Required - The directory move content to.                             ");
    println!("                                                                                                    ");
    println!("[/i <interval-in-seconds>]    Optional - The interval (in seconds) at which to check the            ");
    println!("                                         from-directory for content. Anything found will be moved   ");
    println!("                                         into the to directory. If you use this option, you must    ");
    println!("                                         supply the value <interval-in-seconds>.                    ");
    println!("                                                                                                    ");
    println!("                                         Example: mover.exe \"..\\from\" \"..\\to\" /i 5            ");
    println!("                                                                                                    ");
    println!("[/copy]                       Optional - By default directories and their content will be copied and");
    println!("                                         the originals will be deleted, essentially moving them. By ");
    println!("                                         using the /copy option, the originals will not be deleted  ");
    println!("                                         after being copied.                                        ");
    println!("                                                                                                    ");
    println!("                                         Example: mover.exe \"..\\from\" \"..\\to\" /copy           ");
    println!("----------------------------------------------------------------------------------------------------");
}

/**
 * Exits the program with the provided error code. On exit, this will also display the program information.
 * TODO: Handle empty string values in a better way. (Maybe a macro?)
 */
pub fn quit<T: Into<String>>(ec: &EC, options: &mut Options, err: T) {
    let err = err.into();
    let msg = if err.is_empty() {
        format!("{}\n", get_exit_msg(ec))
    } else {
        format!("{}\n{}\n", get_exit_msg(ec), err)
    };
    options.logger().log_and_flush(&msg);
    print_about_info();
    process::exit(get_exit_code(ec));
}
