use std::time::SystemTime as st;
use std::fs;
use std::io::Error;

pub fn log(message: &str, log_name: &str) -> Result<(), Error>
{
    let time_stamp = st::now();
    let time_stamp = format!("{:?}", time_stamp);
    let message = time_stamp + message;
    let write_log = fs::write(log_name, message.clone());
    match write_log
    {
        Ok(_) => Ok(()),
        Err(e) => panic!("Failed to log {e}")

    }
}