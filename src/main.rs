use serde::{Serialize, Deserialize};
use std::vec::Vec;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TokenYaml {
    version: String,
    tokens: Vec<String>,
}

fn main() {

    let file = File::open("./tokens.yaml").expect("unable to open the file...");
    let buf = BufReader::new(file);
    let tokens:TokenYaml = serde_yaml::from_reader(buf).expect("unable to read the yaml...");
    let mut total = 0;
    let _ret: Vec<i32> = tokens.tokens.iter().map(|x| {
        let split = x.split(":").collect::<Vec<&str>>();
        let i:i32 = split[1].parse().unwrap();
        total += i;
        return i;
    }).collect();

    if total != 1000000 {
        panic!("not the right number of tokens!");
    }
    println!("Internal checks passed! have a nice day!");
}
