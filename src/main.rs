use std::io;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input() -> Result<(), io::Error> {
    let file = try!(File::open("./test_data/test_00.input"));
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let l = line.unwrap();
        println!("{}", l); 
    }               
    Ok(())
}

fn main() {
    let x = parse_input();
    match x {
        Err(c) => println!("Parsing error: {}", c),
        Ok(_) => println!("Parsing OK")
    }
}
