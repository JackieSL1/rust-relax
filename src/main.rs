mod tokenizer;
mod parser;
mod table;

use std::io::{self, Write};
use std::collections::HashMap;
use crate::tokenizer::*;
use crate::table::Table;


fn process_table_tokens(tokens: Vec<Vec<Token>>) -> Result<Vec<Vec<Token>>, String> {
    println!("Processing Tokens: {tokens:?}");

    let table_name = tokens[0][0].clone();
    let mut result: Vec<Vec<Token>> = vec![vec![table_name]];
    let mut tokens = tokens.iter()
        .flatten()
        .skip_while( |token| {*token != &Token::OpenCurly} )
        .skip(1)
        .peekable();

    println!("Current tokens {:?}", tokens);
    while let Some(token) = tokens.next() {
        let mut row: Vec<Token> = Vec::new();
        match token {
            Token::CloseCurly => {return Ok(result);},
            Token::Comma | Token::EOF => {},
            Token::Symbol(_) | Token::Number(_) | Token::String(_)=> {
                row.push(token.clone());
                println!("Current token {:?}", token);
                if *tokens.peek().unwrap() != &Token::Comma {
                    println!("pushing {:?}", row);
                    result.push(row.clone());
                    row.clear();
                }
            },
            _ => { return Err(format!("error: unable to parse token {:?} while processing table", token)); },
        };
    }
    Ok(result)
}

fn main() {
    // let test_string = r#"
    // Student = {ID, Name, Age, Major
    //     "1", "Alice", 20, "Computer Science"
    //     "2", "Bob", 22, "Physics"
    //     "3", "Charlie", 21, "Mathematics"
    // }
    // "#;
    // let test_string = "select id=a Students";

    // let tokens: Vec<tokenizer::Token> = tokenizer::get_tokens(test_string.chars());

    // println!("Input String:\n{}", test_string);
    // println!("{:?}", tokens);
    //

    let mut tables: HashMap<String, Table> = HashMap::new();
    tables.insert("a".to_string(), Table::new(vec!["test".to_string()]));

    loop {
        print!("> ");
        io::stdout().flush().expect("error: unable to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        let mut tokens: Vec<Token> = get_tokens(input.chars());
        //println!("Tokens: {tokens:?}");
        // TODO: Handle newline
        if tokens.len() == 2 {
            if let Token::Symbol(command) = &tokens[0] {
                match command.to_lowercase().as_str() {
                    "quit" | "exit" => {
                        println!("Exiting... Have a nice day!");
                        break;
                    },
                    "help" | "h" => {
                        println!("Need help?");
                    },
                    _ => {},
                }
            }
        }
        if tokens[0] == Token::EOF {
            println!("Exiting... Have a nice day!");
            break;
        } else if tokens[1] == Token::Equals {
            let mut table_tokens = vec![tokens.clone()];

            while tokens[0] != Token::EOF {
                let mut input = String::new();
                print!("\t");
                io::stdout().flush().expect("error: unable to flush stdout");
                io::stdin().read_line(&mut input).expect("error: unable to read user input");
                tokens = get_tokens(input.chars());
                table_tokens.push(tokens.clone());
                //println!("Tokens: {tokens:?}");
            }

            let table_tokens = process_table_tokens(table_tokens);

            println!("Table Tokens: {table_tokens:?}");
        } else {
            tokens.pop(); // Remove EOF

            let tree = parser::parse(&tokens);
            println!("Tree: {tree:?}");

            tree.eval(&tables);
        }
    }

    //Table::new(vec!["1".to_string(), "2".to_string()]);
}
