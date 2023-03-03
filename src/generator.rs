use colored::*;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use spinners::{Spinner, Spinners};

use crate::openai::OpenAI;

pub trait CodeGenerator {
    fn name() -> &'static str;
    fn format(&self, code: &str) -> String;
}

pub struct Generator<T: CodeGenerator> {
    openai: OpenAI,
    code_generator: T,
}

impl<T: CodeGenerator> Generator<T> {
    pub fn new(code_generator: T) -> Result<Self, String> {
        let openai = OpenAI::new()?;
        Ok(Self {
            openai,
            code_generator,
        })
    }

    pub fn get_input() -> Result<String, io::Error> {
        let mut input = String::new();
        print!("{}:>{}", T::name().green(), " ".blue());
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    pub async fn generate_and_print_code(&mut self, input: &str) {
        let mut sp = Spinner::new(Spinners::Dots12, "\t\tOpenAI is Thinking...".into());

        let prompt = format!("Generate code for the given statement. {}", input);
        match self.openai.generate_code(&prompt).await {
            Ok(code) => {
                // stopping the spinner
                sp.stop();

                let formatted = self.code_generator.format(&code);
                self.print_sql_code(&formatted);
            }
            Err(err) => {
                sp.stop();
                self.print_error(&format!("Failed to generate code: {}", err));
            }
        }
    }

    fn print_sql_code(&self, sql_code: &str) {
        let separator = "=".repeat(80);
        println!("\n{}", separator);
        self.print_with_delay(sql_code);
        println!("\n{}", separator);
    }

    fn print_with_delay(&self, code: &str) {
        // Delay between printing each character
        let delay = Duration::from_millis(50);
        for c in code.chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();
            thread::sleep(delay);
        }
    }

    fn print_error(&self, message: &str) {
        let error_msg = format!("Error: {}", message);
        let separator = "-".repeat(error_msg.len());
        println!("\n{}", separator.red());
        self.print_with_delay(&error_msg);
        println!("\n{}", separator.red());
    }

    pub async fn run(&mut self) -> Result<(), String> {
        println!("{esc}c", esc = 27 as char);

        loop {
            match Self::get_input() {
                Ok(input) if input.trim().is_empty() => continue,
                Ok(input) => self.generate_and_print_code(&input).await,
                Err(error) => return Err(format!("Error: {}", error)),
            }
        }
    }
}
