// External libraries
use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // input argument handling and parsing
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    // running the program
    if let Err(e) = run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}

// Function for printing out file output
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // file access checking
    println!("Searching for \"{}\"", config.query);
    println!("In file {} with content:", config.filename);
    // read file into memory
    let content: String = fs::read_to_string(&config.filename)?;
    println!("{}", content);
    Ok(())
}

// Argument struct for grepping
struct Config {
    query: String,
    filename: String,
}

// Config implementation
impl Config {
    // Config constructor parse args into query and filename
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Tool requires 2 input arguments <query> <file name>");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}
