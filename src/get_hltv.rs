use std::fs::{OpenOptions, File};
use std::io::{prelude::*};
use std::time::SystemTime;
use chrono::{Duration, Utc};

pub fn update() -> File{

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
            //println!("Secs since last file: {}", n.as_secs());
            update = n.as_secs() > 86400 || n.as_secs() == 0;
        },
        Err(error) => panic!("RIP: {}", error),
    }

    //2019-10-07
    let today = Utc::today().naive_utc();
    //println!("{}", today);
    
    let past = today.checked_sub_signed(Duration::weeks(12)).unwrap();
    //println!("{}", past);
    
    //update = true;

    let url = format!("https://www.hltv.org/stats/teams?startDate={}&endDate={}&rankingFilter=Top50", past, today);
    //println!("{}", url);

    if update {
        println!("Database has to get updated!");
        let body = reqwest::get(url.as_str()).unwrap()
            .text().unwrap();
        file.write_all(body.as_bytes()).unwrap();
    }

    return file;

}
