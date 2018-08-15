extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum States {
    Undefined,
    ScanningForUnitTests,
    ParsingUnitTests,
    ScanningForDocTests,
    ParsingDocTests,
    Finished,
}

fn set_state(old_state: &mut States, new_state: States) {
    println!("Machine state change: {:?} -> {:?}", old_state, new_state);
    *old_state = new_state;
}

fn parse_input(mut state: &mut States) -> Result<(), io::Error> {
    let file = try!(File::open("./test_data/test_00.input"));
    let reader = BufReader::new(&file);
    set_state(&mut state, States::ScanningForUnitTests);

    for line in reader.lines() {
        let l = line.unwrap();
        match state {
            States::ScanningForUnitTests => {
                let re = Regex::new(r"^running (?P<UnitTestCount>\d*) tests$").unwrap();
                match re.is_match(&l) {
                    false => {}
                    true => {
                        let caps = re.captures(&l).unwrap(); // Assume unwrap() is safe since regex matched
                        println!(
                            "There are {} unit-tests",
                            (&caps["UnitTestCount"]).parse::<i64>().unwrap()
                        );

                        set_state(&mut state, States::ParsingUnitTests);
                    }
                }
            }
            States::ParsingUnitTests => {
                let re = Regex::new(r"^test (?P<UnitTestName>.+) ... ok$").unwrap();
                match re.is_match(&l) {
                    false => {
                        println!("Finished parsing unit-tests");
                        set_state(&mut state, States::ScanningForDocTests);
                    }
                    true => {
                        let caps = re.captures(&l).unwrap(); // Assume unwrap() is safe since regex matched
                        println!("Found test: {}", &caps["UnitTestName"]);
                    }
                }
            }
            _ => println!("{}", l),
        }
    }
    Ok(())
}

fn main() {
    let mut state = States::Undefined;
    let x = parse_input(&mut state);
    match x {
        Err(c) => println!("Parsing error: {}", c),
        Ok(_) => println!("Parsing OK"),
    }
}
