#[warn(special_module_name)]
mod lib;
mod tests;
use anyhow::{anyhow, Result};
use lib::evaluator::Evaluator;
use lib::lexer::Lexer;
use lib::parser::Parser;
use lib::token::Token;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("No input file specified"))?;
    // let mut file = File::open(filename)?;
    let input = read_file_without_comments(filename)?;

    println!("\n\nthe code you provided \n \n{}", input);

    let mut lexer = Lexer::new(input);
    // println!("1");
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        tokens.push(token.clone());
        if token == Token::Eof {
            break;
        }
    }
    // println!("2");
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;

    let mut evaluator = Evaluator::new();

    println!("\nthe result of the code \n\n");
    let _result = evaluator.eval(stmts)?;

    Ok(())
}

fn read_file_without_comments<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path)?;
    let mut input = String::new();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some(comment_pos) = line.find("//") {
            let line = &line[..comment_pos];
            input.push_str(line.trim());
        } else {
            input.push_str(line.trim());
        }
        input.push('\n');
    }

    Ok(input)
}
