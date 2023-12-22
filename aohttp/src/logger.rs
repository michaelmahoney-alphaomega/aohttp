use std::time::SystemTime as st;
use std::fs;
use std::io::Error;
use chrono::{DateTime, Utc, SecondsFormat};

pub fn log(message: &mut String, log_name: &str) -> Result<&'static str, Error>
{
    let  time_stamp = st::now();
    let time_stamp: DateTime<Utc> = time_stamp.into();
    let mut time_stamp: String = time_stamp.to_rfc3339_opts(SecondsFormat::Secs, true);
    // let time_stamp = &mut time_stamp;
    // let  time_stamp: &String = time_stamp.to_owned();
    time_stamp += message;
    message = &mut time_stamp;
    // let message = message.as_mut_str();
    let write_log = fs::write(log_name, message);
    
    match write_log
    {
        Ok(_) => return Ok(message),
        Err(e) => panic!("ERROR: log function failed. Please see error: {e}")
    } 
}