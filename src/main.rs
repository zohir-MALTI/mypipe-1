extern crate clap;
use clap::{App, Arg};
use std::process::Command;
use std::process::Stdio;

fn main() {

    // Building the App 
    let matches = App::new("My Pipe")
        .version("version : 1.0")
        .author("Zohir MALTI <zohirmalti@gmail.com>")
        .arg(
            Arg::with_name("in")
                .long("input")
                .short("in")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("out")
                .long("output")
                .short("out")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Getting the input/output values

    let input = matches.value_of("in").unwrap();
    let output = matches.value_of("out").unwrap();

    // Building the pipe

    let input_args = Command::new(input)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Input loading error");

    let output_args = Command::new(output)
        .stdin(input_args.stdout.unwrap())
        .output()
        .expect("Output loading error");

    // print the result
    println!("{}", String::from_utf8_lossy(&(output_args).stdout));
}
