use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use potoo_ap_csp_pseudocode_interpreter::{Host, Interpreter, Value, check, parse};

struct CliHost {
    random_state: u64,
}

impl Host for CliHost {
    fn input(&mut self) -> Result<Value, String> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|error| error.to_string())?;
        let input = input.trim_end_matches(['\r', '\n']);
        if let Ok(number) = input.parse::<f64>() {
            return Ok(Value::Number(number));
        }
        match input.to_ascii_lowercase().as_str() {
            "true" => Ok(Value::Boolean(true)),
            "false" => Ok(Value::Boolean(false)),
            _ => Ok(Value::String(input.to_owned())),
        }
    }

    fn display(&mut self, value: &Value) -> Result<(), String> {
        print!("{value} ");
        io::stdout().flush().map_err(|error| error.to_string())
    }

    fn random_integer(&mut self, lower: i64, upper: i64) -> Result<i64, String> {
        let span = (upper as i128 - lower as i128 + 1) as u128;
        let threshold = (u64::MAX as u128 + 1) % span;
        let offset = loop {
            self.random_state ^= self.random_state << 13;
            self.random_state ^= self.random_state >> 7;
            self.random_state ^= self.random_state << 17;
            if (self.random_state as u128) >= threshold {
                break (self.random_state as u128) % span;
            }
        };
        Ok((lower as i128 + offset as i128) as i64)
    }
}

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| "usage: potoo-ap-csp <source-file>".to_owned())?;
    let source = std::fs::read_to_string(&path).map_err(|error| format!("{path}: {error}"))?;
    let program = parse(&source).map_err(|diagnostics| {
        diagnostics
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    let checked = check(program).map_err(|diagnostics| {
        diagnostics
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos() as u64
        | 1;
    let mut interpreter = Interpreter::new(CliHost { random_state: seed });
    interpreter
        .run(&checked)
        .map_err(|error| error.to_string())?;
    Ok(())
}
