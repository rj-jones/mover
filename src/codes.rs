pub enum EC {
    Success,
    IOGeneric,
    IValue,
    LogDirCreation,
    LogFilePath,
    LogFilePermission,
    LogFileOther,
    LogFileUknown,
    LogFlush,
    LogWrite,
    PathValidation,
    ToDirCreation,
    SuppliedArgs,
}

pub fn get_exit_code(ec: &EC) -> i32 {
    match ec {
        EC::Success => 0,
        EC::IOGeneric => 1,
        EC::IValue => 2,
        EC::LogDirCreation => 3,
        EC::LogFilePath => 4,
        EC::LogFilePermission => 5,
        EC::LogFileOther => 6,
        EC::LogFileUknown => 7,
        EC::LogFlush => 8,
        EC::LogWrite => 9,
        EC::PathValidation => 10,
        EC::ToDirCreation => 11,
        EC::SuppliedArgs => 12,
    }
}

pub fn get_exit_msg(ec: &EC) -> String {
    match ec {
        EC::Success => String::from("Sucess."),
        EC::IOGeneric => String::from("Error: General IO."),
        EC::IValue => String::from("Error: Invalid value given for the option /i."),
        EC::LogDirCreation => String::from("Error: Unable to create the log directory."),
        EC::LogFilePath => String::from("Error: Unable open or create the log file. One of the directory components of the file path does not exist."),
        EC::LogFilePermission => String::from("Error: Unable open or create the log file. The user lacks permission to get the specified access rights for the file. The user lacks permission to open one of the directory components of the specified path."),
        EC::LogFileOther => String::from("Error: Unable open or create the log file. One of the directory components of the specified file path was not a directory. Filesystem-level errors: full disk, write permission requested on a read-only file system, exceeded disk quota, too many open files, too long filename."),
        EC::LogFileUknown => String::from("Error: Unable open or create the log file. Unknown Error."),
        EC::LogFlush => String::from("Error: Unable to flush log buffer."),
        EC::LogWrite => String::from("Error: Unable to write to log buffer."),
        EC::PathValidation => String::from("Error: The 'From' and 'To' paths cannot be the same."),
        EC::ToDirCreation => String::from("Error: Failed to create a new directory."),
        EC::SuppliedArgs => String::from("Error: Invalid arguments given."),
    }
}
