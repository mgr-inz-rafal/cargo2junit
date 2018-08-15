extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone)]
struct TestCase {
    name: String,
    ok: bool,
    failmessage: String,
}

#[derive(Clone, Default)]
struct TestSuite {
    name: String,
    test_cases: Vec<TestCase>,
}

#[derive(Default)]
struct Test {
    test_suites: Vec<TestSuite>,
}

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

fn get_name_from_state(state: &States) -> &'static str {
    match state {
        States::ScanningForDocTests | States::ParsingDocTests => "doc",
        States::ScanningForUnitTests | States::ParsingUnitTests => "unit",
        _ => "UNKNOWN",
    }
}

fn parse_input(mut state: &mut States) -> Result<(), io::Error> {
    let file = try!(File::open("./test_data/test_01.input"));
    let reader = BufReader::new(&file);
    set_state(&mut state, States::ScanningForUnitTests);

    let mut test = Test {
        ..Default::default()
    };

    let mut current_test_suite = TestSuite {
        name: "Unit-Tests".to_string(),
        ..Default::default()
    };

    for line in reader.lines() {
        let l = line.unwrap();
        match state {
            States::ScanningForUnitTests | States::ScanningForDocTests => {
                let re = Regex::new(r"^running (?P<TestCount>\d*) tests$").unwrap();
                match re.is_match(&l) {
                    false => {}
                    true => {
                        let caps = re.captures(&l).unwrap(); // Assume unwrap() is safe since regex matched
                        println!(
                            "There are {} {}-tests",
                            (&caps["TestCount"]).parse::<i64>().unwrap(),
                            get_name_from_state(&state)
                        );

                        match state {
                            States::ScanningForUnitTests => {
                                set_state(&mut state, States::ParsingUnitTests)
                            }
                            States::ScanningForDocTests => {
                                set_state(&mut state, States::ParsingDocTests)
                            }
                            _ => {}
                        }
                    }
                }
            }
            States::ParsingUnitTests | States::ParsingDocTests => {
                let re =
                    Regex::new(r"^test (?P<UnitTestName>.+) ... (?P<UnitTestResult>ok|FAILED)$")
                        .unwrap();
                match re.is_match(&l) {
                    false => {
                        let re = Regex::new(r"^test result: .+$").unwrap();
                        match re.is_match(&l) {
                            false => {}
                            true => {
                                println!("Finished parsing {}-tests", get_name_from_state(&state));
                                match state {
                                    States::ParsingUnitTests => {
                                        test.test_suites.push(current_test_suite.clone());
                                        current_test_suite = TestSuite {
                                            name: "Doc-Tests".to_string(),
                                            ..Default::default()
                                        };
                                        set_state(&mut state, States::ScanningForDocTests)
                                    }
                                    States::ParsingDocTests => {
                                        test.test_suites.push(current_test_suite.clone());
                                        set_state(&mut state, States::Finished)
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    true => {
                        let caps = re.captures(&l).unwrap(); // Assume unwrap() is safe since regex matched
                        let name = String::from(&caps["UnitTestName"]);
                        let ok = true;
                        let failmessage = "Dupa".to_string();
                        println!("Found test: {}", name);

                        let new_test = TestCase {
                            name,
                            ok,
                            failmessage,
                        };
                        current_test_suite.test_cases.push(new_test);
                    }
                }
            }
            _ => {}
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
