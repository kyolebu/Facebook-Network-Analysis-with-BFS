use std::io::BufRead;
use std::fs::File;

pub fn read_file(txt: &str) -> Vec<(usize, usize)> {
    
    // open and read lines from the file
    let mut file: Vec<(usize, usize)> = Vec::new();
    let open = File::open(txt).expect("Could not open file");
    let lines = std::io::BufReader::new(open).lines();
    
    // extract values and input them into a tuple
    for line in lines {
        let val = line.expect("Error reading");
        let values: Vec<&str> = val.trim().split(' ').collect();
        let node = values[0].parse::<usize>().unwrap();
        let edge = values[1].parse::<usize>().unwrap();
        file.push((node, edge));
    }
    return file;
}