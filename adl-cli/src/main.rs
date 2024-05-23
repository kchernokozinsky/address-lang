mod commands;

use clap::{Parser, Subcommand};
use colored::*;
use commands::{codegen, interpret, parse, run};
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Parse {
        input: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    Codegen {
        input: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    Run {
        #[arg(short, long)]
        bytecode: Option<String>,

        #[arg(short, long)]
        file: Option<String>,
    },
    Interpret {
        input: String,
    },
}

fn main() {
    print_welcome_message();

    loop {
        let mut input = String::new();
        print!("{}", "adl-cli> ".blue());
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("{}", "Error reading input. Please try again.".red());
            continue;
        }

        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        if trimmed_input.eq_ignore_ascii_case("exit") {
            println!("{}", "Goodbye!".yellow());
            break;
        }

        let args =
            match Args::try_parse_from(format!("adl-cli {}", trimmed_input).split_whitespace()) {
                Ok(args) => args,
                Err(e) => {
                    eprintln!("{}", e.to_string().red());
                    continue;
                }
            };

        match args.cmd {
            Commands::Parse { input, output } => parse::run(input, output),
            Commands::Codegen { input, output } => codegen::run(input, output),
            Commands::Run { bytecode, file } => {
                if let Some(bytecode) = bytecode {
                    run::run_bytecode(bytecode);
                } else if let Some(input) = file {
                    run::compile_and_run(input);
                }
            }
            Commands::Interpret { input } => interpret::run(input),
        }
    }
}

fn print_welcome_message() {
    let welcome_message = r#"
    _____/\\\\\\\\\____        __/\\\\\\\\\\\\____        __/\\\_____________        
    ___/\\\\\\\\\\\\\__        _\/\\\////////\\\__        _\/\\\_____________       
     __/\\\/////////\\\_        _\/\\\______\//\\\_        _\/\\\_____________      
      _\/\\\_______\/\\\_        _\/\\\_______\/\\\_        _\/\\\_____________     
       _\/\\\\\\\\\\\\\\\_        _\/\\\_______\/\\\_        _\/\\\_____________    
        _\/\\\/////////\\\_        _\/\\\_______\/\\\_        _\/\\\_____________   
         _\/\\\_______\/\\\_        _\/\\\_______/\\\__        _\/\\\_____________  
          _\/\\\_______\/\\\_        _\/\\\\\\\\\\\\/___        _\/\\\\\\\\\\\\\\\_ 
           _\///________\///__        _\////////////_____        _\///////////////__

    
    
                                 Welcome to adl-cli 

    usage: adl-cli [command] [options]

    commands: 

        - run   

        - interpret

        - codegen

        - parse


"#;

    println!("{}", welcome_message.blue());
}
