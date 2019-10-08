use reqwest::get;
use std::fs::{OpenOptions};
use std::io::prelude::*;
use std::time::SystemTime;
//use std::io::{Error, ErrorKind};
use chrono::prelude::*;

fn main() {

    let update;

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("data.html").unwrap(); 
    
    let last_filemod = file.metadata().unwrap().modified().unwrap();

    //println!("{:?}", last_filemod);

    match SystemTime::now().duration_since(last_filemod) {
        Ok(n) => {
            println!("Secs since last file: {}", n.as_secs());
            update = n.as_secs() > 86400 || n.as_secs() == 0;
        },
        Err(error) => panic!("RIP: {}", error),
    }

    //2019-10-07
    let time:String = Utc::today().naive_utc().to_string();
    println!("{}", time);


    if update {
        let body = reqwest::get("https://www.rust-lang.org").unwrap()
            .text().unwrap();
        file.write_all(body.as_bytes()).unwrap();
    }

}
