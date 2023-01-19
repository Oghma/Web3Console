use std::io::{self, Write};

mod eval;
mod lexer;
mod parser;
mod utilities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Web3Console!");
    let eval_commands =
        eval::EvalCommand::new(String::from("https://bsc-dataseed.binance.org")).unwrap();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        let user_command = user_input.trim();

        if user_command == "q" {
            break;
        }
        if user_command == "" {
            continue;
        }

        match lexer::tokenize(&user_command) {
            Ok(tokens) => {
                let command = parser::parse(&tokens).unwrap();
                match eval_commands.eval(&command).await {
                    Ok(out) => println!("{}", out),
                    Err(err) => println!("{}", err),
                }
            }
            Err(err) => println!("{}", err),
        };
    }

    Ok(())
}
