extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

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

fn parse_input(
    mut state: &mut States,
    input_file: &str,
    output_file: &str,
) -> Result<(), io::Error> {
    print!("Opening input file '{}'...", input_file);
    let file = try!(File::open(input_file));
    println!("DONE");
    print!("Opening output file '{}'...", output_file);
    let mut file_result = try!(File::create(output_file));
    println!("DONE");
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
                    Regex::new(r"^test (?P<TestName>.+) ... (?P<TestResult>ok|FAILED)$").unwrap();
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
                        let name = String::from(&caps["TestName"]);
                        let state = String::from(&caps["TestResult"]);
                        let ok = if state == "ok" { true } else { false };
                        let failmessage = "Unsupported".to_string();
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

    // Output XML
    print!("Writing output file...");
    writeln!(file_result, "<?xml version=\"1.0\" ?>");
    writeln!(file_result, "<testsuites>");
    let mut suite_id = 0;
    for suite in test.test_suites {
        let failures = suite.test_cases.iter().filter(|&n| n.ok == false).count();
        writeln!(
            file_result,
            "\t<testsuite errors=\"0\" failures=\"{}\" id=\"{}\" name=\"{}\" tests=\"{}\">",
            failures,
            suite_id,
            suite.name,
            suite.test_cases.len()
        );
        for case in suite.test_cases {
            writeln!(file_result, "\t\t<testcase name=\"{}\"/>", case.name);
            if case.ok == false {
                writeln!(
                    file_result,
                    "\t\t\t<failure message=\"{}\">Assertion failed</failure>",
                    case.failmessage
                );
            }
        }
        writeln!(file_result, "\t</testsuite>");
        suite_id += 1;
    }
    writeln!(file_result, "</testsuites>");
    println!("DONE!");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Incorrect number of arguments.");
        println!("Usage:");
        println!("\t cargo2junit input_file output_file");
        std::process::exit(-1);
    }

    let mut state = States::Undefined;
    let x = parse_input(&mut state, &args[1], &args[2]);
    match x {
        Err(c) => {
            println!("Parsing error: {}", c);
            std::process::exit(-2);
        }
        Ok(_) => println!("Parsing OK"),
    }
}
