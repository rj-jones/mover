use super::codes::*;
use super::quit;
use std::path::PathBuf;
use std::{thread, time};

#[path = "logger.rs"]
mod logger;
use logger::*;

pub struct Options {
    from_dir: PathBuf,
    to_dir: PathBuf,
    interval: usize,
    copy: bool,
    overwrite: bool,
    logger: Logger,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            from_dir: PathBuf::from(""),
            to_dir: PathBuf::from(""),
            interval: 1,
            copy: false,
            overwrite: false,
            logger: Logger::new(),
        }
    }
}

impl Options {
    pub fn from_dir(&self) -> PathBuf {
        self.from_dir.clone()
    }
    pub fn to_dir(&self) -> PathBuf {
        self.to_dir.clone()
    }

    pub fn from_dir_str(&self) -> &str {
        self.from_dir.to_str().unwrap()
    }
    pub fn to_dir_str(&self) -> &str {
        self.to_dir.to_str().unwrap()
    }

    pub fn logger(&mut self) -> &mut Logger {
        &mut self.logger
    }

    pub fn interval(&self) -> usize {
        self.interval
    }

    pub fn copy(&self) -> bool {
        self.copy
    }

    pub fn overwrite(&self) -> bool {
        self.overwrite
    }

    pub fn new(args: &Vec<String>) -> Options {
        let mut options = Options {
            ..Default::default()
        };
        if args.len() <= 2 {
            quit(&EC::SuppliedArgs, &mut options, String::new());
        }
        for arg_index in 2..args.len() {
            // Get directory paths. Index 1 & 2 should always be 'from' & 'to' paths.
            if arg_index == 2 {
                let arg_from_dir = args[arg_index - 1].to_string();
                let arg_to_dir = args[arg_index].to_string();
                options.from_dir = PathBuf::from(arg_from_dir);
                options.to_dir = PathBuf::from(arg_to_dir);
                options.validate_paths();
            }
            // Get options
            match args[arg_index].as_str() {
                // Interval option
                "/i" => {
                    if arg_index + 1 <= args.len() {
                        match args[arg_index + 1].parse::<usize>() {
                            Ok(i) => {
                                options.validate_interval(i);
                                options.interval = i;
                            }
                            Err(e) => quit(&EC::SuppliedArgs, &mut options, e.to_string()),
                        }
                    }
                }
                // Copy option
                "/c" => options.copy = true,
                // Overwrite option
                "/o" => options.overwrite = true,
                // Unknown values
                _ => {}
            }
        }
        options.print_args();
        options
    }

    // TODO: Make waiting for a correct path an option.
    pub fn validate_paths(&mut self) {
        if !self.from_dir.exists() {
            self.logger.log_and_flush(format!(
                "The path: \"{}\" does not exist. \nWaiting until fixed...\n",
                self.from_dir.to_str().unwrap()
            ));
            let mut err = false;
            while !self.from_dir.exists() {
                err = true;
                thread::sleep(time::Duration::from_secs(1u64));
            }
            if err {
                self.logger
                    .log_and_flush("'From' path restored, continuing...\n");
            }
        }
        if !self.to_dir.exists() {
            self.logger.log_and_flush(format!(
                "The path: \"{}\" does not exist. \nWaiting until fixed...\n",
                self.to_dir.to_str().unwrap()
            ));
            let mut err = false;
            while !self.to_dir.exists() {
                err = true;
                thread::sleep(time::Duration::from_secs(1u64));
            }
            if err {
                self.logger
                    .log_and_flush("'To' path restored, continuing...\n");
            }
        }
        if self.from_dir() == self.to_dir() {
            quit(&EC::PathValidation, self, "");
        }
    }

    fn print_args(&mut self) {
        let mut init_info = String::new();
        init_info.push_str("\n--INITIALIZED---------------------------------------------------------------------------------------\n");
        init_info.push_str(format!("From Path:    {}\n", self.from_dir.to_str().unwrap()).as_str());
        init_info.push_str(format!("To Path:      {}\n", self.to_dir.to_str().unwrap()).as_str());
        init_info.push_str(format!("Interval:     {}\n", self.interval).as_str());
        init_info.push_str(format!("Copy:         {}\n", self.copy.to_string()).as_str());
        init_info.push_str(format!("Overwrite:    {}\n", self.overwrite.to_string()).as_str());
        init_info.push_str(format!("Logs:         {}\n", self.logger.dir_as_str()).as_str());
        init_info.push_str("--Press 'Ctrl + C' to quit--------------------------------------------------------------------------");
        self.logger().log_and_flush(&init_info);
    }

    fn validate_interval(&mut self, i: usize) {
        if i < 1 || i > usize::MAX {
            quit(&EC::IValue, self, "");
        }
    }
}
