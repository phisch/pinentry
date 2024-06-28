use std::io::{BufRead, Read, Stdin, Stdout, Write};

use client_request::ClientRequest;

pub mod client_request;
pub mod option;
pub mod response;

pub struct Pinentry {
    timeout: Option<i32>,
    description: Option<String>,
    prompt: Option<String>,
    title: Option<String>,
    ok_button: Option<String>,
    cancel_button: Option<String>,
    not_ok_button: Option<String>,
    error: Option<String>,
    repeat: bool,
    quality_bar: bool,
    quality_bar_tooltip: Option<String>,
    generate_pin: bool,
    generate_pin_tooltip: Option<String>,
    key_info: Option<String>,
}

impl Pinentry {
    pub fn new() -> Pinentry {
        Pinentry {
            timeout: None,
            description: None,
            prompt: None,
            title: None,
            ok_button: None,
            cancel_button: None,
            not_ok_button: None,
            error: None,
            repeat: false,
            quality_bar: false,
            quality_bar_tooltip: None,
            generate_pin: false,
            generate_pin_tooltip: None,
            key_info: None,
        }
    }

    /*
    pub fn run(&mut self, stdin: &mut dyn Read, mut stdout: Write) {
        loop {
            let mut input = String::new();
            stdin.read(&mut input).unwrap();

            let command = ClientRequest::parse(input.as_str());

            print!("Your command was: {:?}", command);

            let input = input.trim();
            if input == "quit" {
                break;
            }
            println!("You entered: {}", input);
        }
    }
    */

    pub fn run<T: BufRead, U: Write>(&mut self, mut input: T, mut output: U) {
        // read a line from input
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        writeln!(output, "Your command was: {:?}", line).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, Cursor};

    use super::*;

    #[test]
    fn test_pinentry() {
        // use mock stdin and stdout
        let mut pinentry = Pinentry::new();
        
        let mock_input = "mocked user input\n";
        let cursor = Cursor::new(mock_input);

        let mut output_buffer = Cursor::new(Vec::new());

        // run the pinentry
        pinentry.run(cursor, &mut output_buffer);


        // print the output
        let output = String::from_utf8(output_buffer.into_inner()).unwrap();
        println!("output was: {}", output);

    }
}
