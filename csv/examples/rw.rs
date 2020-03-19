use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::prelude::*;
use csv::Reader;

fn read_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(filename)?;
    let mut vec: Vec<HashMap<String, String>> = Vec::new();

    for result in rdr.records() {
        let mut entry: HashMap<String, String> = HashMap::new();
        let record = result?;
        let timestamp = &record[0];
        let close = &record[4];
        let volume = &record[5];
        let t1 = Utc.datetime_from_str(timestamp, "%Y-%m-%d %H:%M").unwrap();
        let t2 = t1.timestamp().to_string();
        entry.insert(
            "measurement".to_string(),
            file_stem(filename).unwrap().to_string(),
        );
        entry.insert("timestamp".to_string(), t2);
        entry.insert("close".to_string(), close.to_string());
        entry.insert("volume".to_string(), volume.to_string());
        vec.push(entry);
    }
    println!("{:?}\n", vec);
    Ok(())
}

fn dir_reader(mydir: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mypath = Path::new(&mydir);
    let mut vec: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(mypath)? {
        let entry = entry?;
        let path = entry.path();
        vec.push(path);
    }

    Ok(vec)
}

fn file_stem(filename: &str) -> Option<&str> {
    let path = Path::new(filename);
    let name = path.file_stem().unwrap().to_str();
    name
}

fn rw_processor(dirin: String,dirout: String) -> Result<(), Box<dyn Error>> {
    // this reads in a vector of files from directory in
    let vec = dir_reader(dirin).unwrap();
    // for each file name in the directory
    for name in vec {
        let filename = name.to_str().unwrap();
        // returns a vector of hashmaps
        let vec = read_csv(filename);
        // write the line protocol using the measurement for the filename
        let _x = write_lp(dirout,vec);
    }

    Ok(())
}

fn main() {
    let dirin = String::from("./examples/data");
    let dirout = String::from("/tmp/out");
    let _ = rw_processor(dirin,dirout);
}