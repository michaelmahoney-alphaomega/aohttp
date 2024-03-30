use std::time::SystemTime;
use std::fs;
use std::io::Write;
use std::io::Error;
use chrono::{DateTime, Utc, SecondsFormat};

pub fn log<'a>(message: &'a mut String, log_path: &str) -> Result<(), Error>
{
    //create the log file if it doesn't exist
    {
        let _ = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(log_path);
    }
    
    // make a timestamp String - likely a better way but I'm sick of fighting with the typing system
    let time_stamp = SystemTime::now();
    let time_stamp: DateTime<Utc> = time_stamp.into();
    let mut time_stamp = time_stamp.to_rfc3339_opts(SecondsFormat::Secs, true);

    // prepend the timestamp to the logging message
    time_stamp.push_str(" ");
    let time_stamp = time_stamp.as_str();
    message.insert_str(0, time_stamp);

    let out_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(log_path);  

    match out_file {
        Ok(mut file) => writeln!(file, "{}",message),
        Err(e) => panic!("ERROR: log function failed: {e}")} 
}