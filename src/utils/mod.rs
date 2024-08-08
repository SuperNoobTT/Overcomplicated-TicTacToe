
use std::{
    str::FromStr, 
    io::{stdin, Write}
};

///What
/// Lol this is pretty poorly written but it works :L
/// Used to get inputs, do some logic, and print error statements!
pub(super) fn input_helper<IN, OUT, F>(request: &str, mut logic: F) -> OUT 
    where 
        F: FnMut(IN) -> Result<OUT, &'static str>,
        IN: FromStr
    {
        loop {
            let mut input: String = String::new();
            println!("{}", request);
            std::io::stdout().flush().expect("Failed to flush :thonk:");
            stdin().read_line(&mut input).expect("Failed to read line, maybe too long?");
            let input = input.trim_end();
            if let Ok(num) = input.parse() {
                match logic(num) {
                    Ok(result) => return result,
                    Err(msg) => eprintln!("{msg}")
                }
            } else {
                eprintln!("Could not parse your input, please ensure you've entered a valid input");
            }
        }
    }