# README  

## mover

Automatic file copy tool. The mover tool will check a given directory for any files and move all of them to the specified directory at a set interval. By default logs are store at "C:\mover\logs".

### Usage

`mover.exe <from-directory> <to-directory> [/i <interval-in-seconds>] [/c] [/o]`  
<br />
`<from-directory>`  
Required - The directory to check and move content *from*.  
<br />
`<to-directory>`  
Required - The directory move content *to*.  
<br />
`[/i <interval-in-seconds>]`  
Optional - The interval (in seconds) at which to check the *from-directory* for content. Anything found will be moved into the *to-directory*. **If you use this option, you must supply the value `<interval-in-seconds>`**. The default value is 1 second.  
<br />
Example: `mover.exe "..\from" "..\to" /i 5`  
<br />
`[/c]`  
Optional - By default, directories and their content will be copied and the originals will be deleted, essentially moving them. By using the /c option, **the originals will not be deleted** after being copied.  
<br />
Example: `mover.exe "..\from" "..\to" /c`  
<br />
`[/o]`
Optional - By default, files in the *to-directory* with the same name and relative path as files in the *from-directory* **will not be overwritten**, even if the file contents are different. By using the /o option, files in the *to-directory* **will be overwritten**.  
<br />
Example: `mover.exe "..\from" "..\to" /o`  
<br />

## Version 1.0.0

- Added option /c - Keeps the original files that were moved (no removal of originals after copying).
- Added option /o - Overwrites files in the to-directory that have the same relative path and name, but different file contents.
- Added more detailed logging for invalid/missing paths, and start up info. No more empty logs.
- Better path validation for user supplied arguments.
- Fixed crash when no arguments are given.
- Fixed issue copying directories.

## Version 1.0.1

- Adjusted the README.md file.
- Fixed outdated program info that prints to the console.
