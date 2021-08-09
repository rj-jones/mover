use super::{get_exit_code, get_exit_msg, EC};
// use super::exit_codes::*; Why does this not work here but in options.rs?
use chrono::offset;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, ErrorKind, Write};
use std::path::PathBuf;

pub struct Logger {
    directory: PathBuf,
    entries: Vec<String>,
    flagged_paths: Vec<PathBuf>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            ..Default::default()
        }
    }
}

impl Default for Logger {
    fn default() -> Logger {
        Logger {
            directory: PathBuf::from("C:\\mover\\logs"),
            entries: Vec::<String>::new(),
            flagged_paths: Vec::<PathBuf>::new(),
        }
    }
}

impl Logger {
    /**
     * Logs and error and flags the given file or directory path. Before any errors are logged,
     * their path must not be flagged for the log to be written.
     */
    pub fn log_error<S: Into<String>>(&mut self, path: PathBuf, msg: S) {
        if !self.flagged_paths.contains(&path) {
            let mut entry = self.get_date_time();
            entry.push_str("\t");
            entry.push_str(msg.into().as_str());
            entry.push_str("\n\n");
            self.entries.push(entry);
            self.flagged_paths.push(path);
        }
    }

    /**
     * Returns the program's log directory path.
     */
    pub fn dir_as_str(&self) -> &str {
        // TODO: This will not produce a crash while hard-coded. Once the program starts accepting a
        // new log location through the /log option, it will need to be validated before being used.
        self.directory.to_str().unwrap()
    }

    /**
     * Optionally takes a final log entry and flushes any buffered logs, writing them to the disk.
     */
    pub fn log_and_flush<S: Into<String>>(&mut self, msg: S) {
        let msg = msg.into();
        if !&msg.is_empty() {
            let mut entry = self.get_date_time();
            entry.push_str("\t");
            entry.push_str(msg.as_str());
            entry.push_str("\n\n");
            self.entries.push(entry);
        }
        self.flush_logs();
    }

    /**
     * Adds a log entry to the log buffer. This log function takes a 'from' and 'to' directory and
     * formats it for entry.
     */
    pub fn log_transfer<F: Into<String>, T: Into<String>>(&mut self, from: F, to: T, copy: bool) {
        let from_string = from.into();
        let mut entry = self.get_date_time();
        if copy {
            entry.push_str("\nCopied From:\t\"");
        } else {
            entry.push_str("\nMoved From:\t\"");
        }
        entry.push_str(from_string.as_str());
        if copy {
            entry.push_str("\"\nCopied To:\t\"");
        } else {
            entry.push_str("\"\nMoved To:\t\"");
        }
        entry.push_str(to.into().as_str());
        entry.push_str("\"\n\n");
        self.entries.push(entry);

        // Clear any flagged paths after a successful transfer.
        self.remove_flagged_path(PathBuf::from(from_string));
    }

    pub fn log_info<T: Into<String>>(&mut self, info: T) {
        let mut entry = self.get_date_time();
        entry.push_str("\t");
        entry.push_str(info.into().as_str());
        entry.push_str("\n\n");
        self.entries.push(entry);
    }

    /**
     * Writes all logs to disk.
     */
    pub fn flush_logs(&mut self) {
        if !self.entries.is_empty() {
            let mut log_file = self.get_buf_writer().unwrap();
            for entry in &self.entries {
                match write!(log_file, "{}", entry) {
                    Ok(()) => print!("{}", entry),
                    Err(e) => {
                        print!(
                            "{}\t{}\n{}\n\n",
                            self.get_date_time(),
                            get_exit_msg(&EC::LogWrite),
                            e
                        );
                        std::process::exit(get_exit_code(&EC::LogWrite));
                    }
                };
            }
            match log_file.flush() {
                Ok(()) => self.clear_log_buffer(),
                Err(e) => {
                    print!(
                        "{}\t{}\n{}\n\n",
                        self.get_date_time(),
                        get_exit_msg(&EC::LogFlush),
                        e
                    );
                    std::process::exit(get_exit_code(&EC::LogFlush));
                }
            }
        }
    }

    pub fn remove_flagged_path(&mut self, path: PathBuf) {
        if self.flagged_paths.contains(&path) {
            if let Some(index) = self.flagged_paths.iter().position(|x| x == &path) {
                self.flagged_paths.remove(index);
            }
        }
    }

    fn clear_log_buffer(&mut self) {
        self.entries.clear();
    }

    /**
     * This function should never return None, the program will quit with an error message before
     * getting to the end of the function.
     */
    fn get_buf_writer(&self) -> Option<BufWriter<fs::File>> {
        let log_path = self.get_log_path();

        if !log_path.exists() {
            match fs::create_dir_all(self.directory.clone()) {
                Ok(()) => {}
                Err(e) => {
                    print!(
                        "{}\t{}\n{}\n\n",
                        self.get_date_time(),
                        get_exit_msg(&EC::LogDirCreation),
                        e
                    );
                    std::process::exit(get_exit_code(&EC::LogDirCreation));
                }
            };
        }

        match OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&log_path)
        {
            Ok(file) => {
                let file = BufWriter::new(file);
                return Some(file);
            }
            Err(e) => {
                let exit_code = match e.kind() {
                    ErrorKind::NotFound => EC::LogFilePath,
                    ErrorKind::PermissionDenied => EC::LogFilePermission,
                    ErrorKind::Other => EC::LogFileOther,
                    _ => EC::LogFileUknown,
                };
                print!("{}\n{}\n\n", get_exit_msg(&exit_code), e); // Must use std::print
                std::process::exit(get_exit_code(&exit_code));
            }
        };
    }

    fn get_date(&self) -> String {
        offset::Local::today()
            .naive_local()
            .format("%F")
            .to_string()
    }

    fn get_date_time(&self) -> String {
        offset::Local::now()
            .naive_local()
            .format("%F %I:%M:%S%P")
            .to_string()
    }

    fn get_log_path(&self) -> PathBuf {
        let mut name: String = self.get_date();
        name.push_str(".txt");
        let mut path = self.directory.clone();
        path.push(name.as_str());
        path
    }
}
