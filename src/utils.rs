use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn return_bufreader(file: &str) -> BufReader<File> {
    match std::fs::File::open(file) {
        Ok(file) => BufReader::new(file),
        Err(e) => {
            eprintln!("Error reading the file: {}. Error: {}", file, e);
            std::process::exit(1)
        }
    }
}

pub fn read_config_file(buffer: BufReader<File>) -> HashMap<String, u16> {
    let mut data: HashMap<String, u16> = HashMap::new();
    for line in buffer.lines() {
        let line_str = line.expect("Error");
        let data_vec = line_str.split(':').collect::<Vec<&str>>();
        if data_vec.len() == 2 {
            data.insert(
                data_vec.get(0).unwrap().to_string(),
                data_vec.get(1).unwrap().parse().unwrap_or_else(|_| {
                    panic!("Error reading the port for the line {:?}", data_vec)
                }),
            );
        }
    }
    data
}
