use chrono::offset;
use same_file::*;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufWriter, ErrorKind, Write};
use std::path::Path;
use std::path::PathBuf;
use std::{env, fs, process, thread, time};

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static LOG_DIR: &str = "C:\\mover\\logs";

fn main() {
    // Collect any arguments given.
    let args: Vec<String> = env::args().collect();

    let mut handles_b_list: Vec<Handle> = Vec::new();

    if args.len() == 2 {
        qt("");
    } else if args.len() < 3 {
        qt("Please provide the required arguments.");
    } else {
        // The interval to check for files to be moved.
        let mut interval: usize = 1;

        // Map the arguments to the Paf struct.
        let mut from_dir = String::new();
        let mut to_dir = String::new();

        for arg_index in 2..args.len() {
            if arg_index == 2 {
                // The 'from' and 'to' path have been given.
                from_dir = args[arg_index - 1].to_string();
                to_dir = args[arg_index].to_string();
                validate_path(&from_dir);
                validate_path(&to_dir);
            }
            if arg_index == 3 {
                // The interval '/i' has been given, check for a valid value.
                if args.len() < 5 {
                    qt("Please provide a value for /i");
                } else {
                    // TODO: Handle the error for parsing the string.
                    match args[arg_index + 1].parse::<usize>() {
                        Ok(i) => {
                            interval = i;
                            validate_interval(interval);
                        }
                        Err(e) => qt(format!("Failed to parse interval.\nError: {}", e)),
                    }
                }
            }
        }

        // Display to the user their supplied values.
        print_args(&from_dir, &to_dir, interval);

        // Create the logs directory
        match fs::create_dir_all(LOG_DIR) {
            Ok(()) => {}
            Err(e) => qt(format!(
                "Unable to create the logs directory.\nError: {}",
                e
            )),
        };

        // Start the main loop of the program that checks for files to
        // be moved at the specified interval.
        loop {
            thread::sleep(time::Duration::from_secs(interval as u64));
            match move_files(&from_dir, &to_dir, &mut handles_b_list) {
                Ok(()) => continue,
                Err(e) => qt(format!("Error: {}", e)),
            }
        }
    }
}

/**
 * Quits the program with an optional message and displays the
 * program's about info.
 */
fn qt<S: Into<String>>(msg: S) {
    print_about_info(&msg.into());
    process::exit(0x0100);
}

// Check if the user supplied value for the /i option is valid.
fn validate_interval(i: usize) {
    if i < 1 || i > usize::MAX {
        qt(format!(
            "Invalid value for /i <interval-in-seconds>\nValid values: 1-{}",
            usize::MAX
        ));
    }
}

// Check if a given path is valid. Wait until valid to continue.
fn validate_path(path_string: &String) {
    let path = Path::new(path_string.as_str());
    if !path.exists() {
        log_err(&format!(
            "Path doesn't exit: {}\nWaiting...\n\n",
            path_string
        ));
        println!("Path doesn't exit: {}\nWaiting...\n\n", path_string);
        while !path.exists() {
            thread::sleep(time::Duration::from_secs(1u64));
        }
    }
}

fn new_log_entry(from: &str, to: &str) -> String {
    let mut log_data: String = offset::Local::now()
        .naive_local()
        .format("%F %I:%M:%S%P")
        .to_string();
    log_data.push_str("\nFrom: ");
    log_data.push_str(from);
    log_data.push_str("\nTo: ");
    log_data.push_str(to);
    log_data.push_str("\n\n");
    log_data
}

fn get_log_file() -> Option<BufWriter<fs::File>> {
    // Create the log file's path.
    let mut log_file_name: String = offset::Local::today()
        .naive_local()
        .format("%F")
        .to_string();
    log_file_name.push_str(".txt");
    let mut log_file_path = PathBuf::from(LOG_DIR);
    log_file_path.push(log_file_name.as_str());

    // Try to return the BufWriter
    match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&log_file_path)
    {
        Ok(file) => {
            let file = BufWriter::new(file);
            return Some(file);
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => qt(format!("Failed to create a log file.\nError: One of the directory components of the file path does not exist.\nDetail: {}", e)),
            ErrorKind::PermissionDenied => qt(format!("Failed to create a log file.\nError: The user lacks permission to get the specified access rights for the file. The user lacks permission to open one of the directory components of the specified path.\nDetail: {}", e)),
            ErrorKind::Other => qt(format!("Failed to create a log file.\nError: One of the directory components of the specified file path was not, in fact, a directory. Filesystem-level errors: full disk, write permission requested on a read-only file system, exceeded disk quota, too many open files, too long filename, too many symbolic links in the specified path (Unix-like systems only), etc.\nDetail: {}", e)),
            _ => qt(format!("Failed to create a log file. Unknown Error: {}", e)),
        },
    };
    None
}

/**
 * Only use this to write errors immeadiately to the log file.
 */
fn log_err(data: &String) {
    // Create the log file's path.
    let mut log_file_name: String = offset::Local::today()
        .naive_local()
        .format("%F")
        .to_string();
    log_file_name.push_str(".txt");
    let mut log_file_path = PathBuf::from(LOG_DIR);
    log_file_path.push(log_file_name.as_str());

    match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&log_file_path)
    {
        Ok(mut file) => match write!(file, "{}", data) {
            Ok(()) => {},
            Err(e) => qt(format!("Unable to write log. Log: {}\nError: {}", data, e)),
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => qt(format!("Failed to create a log file.\nError: One of the directory components of the file path does not exist.\nDetail: {}", e)),
            ErrorKind::PermissionDenied => qt(format!("Failed to create a log file.\nError: The user lacks permission to get the specified access rights for the file. The user lacks permission to open one of the directory components of the specified path.\nDetail: {}", e)),
            ErrorKind::Other => qt(format!("Failed to create a log file.\nError: One of the directory components of the specified file path was not, in fact, a directory. Filesystem-level errors: full disk, write permission requested on a read-only file system, exceeded disk quota, too many open files, too long filename, too many symbolic links in the specified path (Unix-like systems only), etc.\nDetail: {}", e)),
            _ => qt(format!("Failed to create a log file. Unknown Error: {}", e)),
        },
    };
}

fn move_files(
    from_dir: &String,
    to_dir: &String,
    blisted_handles: &mut Vec<Handle>,
) -> io::Result<()> {
    let from_dir_path = PathBuf::from(&from_dir);
    let to_dir_path = PathBuf::from(&to_dir);
    let mut log_file = get_log_file().unwrap(); // Use unwrap because errors are handeled within the function.

    // Check if dir still exists
    validate_path(&from_dir);
    validate_path(&to_dir);

    for entry in fs::read_dir(from_dir_path)? {
        let entry: fs::DirEntry = entry?;
        let from_file_path = entry.path().to_owned();
        let mut to_file_path = to_dir_path.to_owned();
        to_file_path.push(entry.file_name());

        let from_str = from_file_path.to_str().unwrap();
        let to_str = to_file_path.to_str().unwrap();

        // For log blacklist (prevent logging the same thing).
        let file_handle: Handle = Handle::from_path(&from_file_path)?;
        let log_entry = new_log_entry(from_str, to_str);

        match fs::copy(&from_file_path, &to_file_path) {
            Ok(file_size) => {
                if file_size == entry.metadata()?.len() {
                    // Successfully copied the file. Try to delete the old one and remove
                    // it from the blacklist if necessary.
                    if blisted_handles.contains(&file_handle) {
                        let index = blisted_handles
                            .iter()
                            .position(|x| *x == file_handle)
                            .unwrap();
                        blisted_handles.remove(index);
                    }
                    match fs::remove_file(&from_file_path) {
                        Ok(()) => match write!(log_file, "{}", log_entry) {
                            Ok(()) => {
                                println!("{}", log_entry);
                                continue;
                            }
                            Err(e) => qt(format!(
                                "Unable to write to log file.\n{}\nError: {}",
                                log_entry, e
                            )),
                        },
                        // TODO: Handle in use error (32).
                        Err(e) => qt(format!("Failed to remove file {}\nError: {}", log_entry, e)),
                    }
                }
            }
            Err(e) => {
                if e.raw_os_error() == Some(32) {
                    // File in use by another program.
                    continue;
                } else if e.raw_os_error() == Some(112) {
                    // Not enough disk space. Use blacklist to prevent log from being spammed.
                    if blisted_handles.contains(&file_handle) {
                        continue;
                    } else {
                        blisted_handles.push(file_handle);
                        log_err(&format!(
                            "===== FAILED: NOT ENOUGH DISK SPACE TO COPY =====\n{}",
                            log_entry
                        ));
                        continue;
                    }
                } else {
                    qt(format!("Failed to copy file {}\nError: {}", log_entry, e));
                }
            }
        }
    }

    log_file.flush().unwrap();

    Ok(())
}

/**
 * Prints the program description and instructions, optionally takes a msg
 * parameter for additional information.
 */
fn print_about_info(msg: &String) {
    if msg.len() > 0 {
        println!("                                                                ");
        println!("--MSG-----------------------------------------------------------");
        println!("{}", msg);
        println!("----------------------------------------------------------------");
        log_err(msg);
    }
    println!("                                                                ");
    println!(
        "--INFO--{}---------------------------------------------------",
        VERSION
    );
    println!("mover  -  Automatic file copy tool. The mover tool will check a ");
    println!("          given directory for any files and copy all of them to ");
    println!("          the specified directory at a set interval. Logs are   ");
    println!("          stored at \"{}\".", LOG_DIR);
    println!("                                                                ");
    println!("mover <from-directory> <to-directory> [/i <interval-in-seconds>]");
    println!("                                                                ");
    println!("    <from-directory>        Set the directory to check and copy ");
    println!("                            files from.                         ");
    println!("                                                                ");
    println!("    <to-directory>          Set the directory to send the copied");
    println!("                            copied files to.                    ");
    println!("                                                                ");
    println!("    </i>                    Set the interval in seconds for how ");
    println!("                            often to check the from-directory.  ");
    println!("                            Defaults to 1 second.               ");
    println!("----------------------------------------------------------------");
    println!("                                                                ");
}

fn print_args(from_dir: &String, to_dir: &String, interval: usize) {
    println!("                                                                ");
    println!("----------------------------------------------------------------");
    println!("From Path:\t{}", from_dir);
    println!("To Path:\t{}", to_dir);
    println!("Interval:\t{}", interval);
    println!("Log Files:\t{}", LOG_DIR);
    println!("Press 'Ctrl + C' to quit...");
    println!("----------------------------------------------------------------");
    println!("                                                                ");
}
