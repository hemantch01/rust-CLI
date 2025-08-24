use std::env;
use std::fs;
use std::process;
use std::error::Error;
use rust_CLI::search;
fn main() {
   let arguments:Vec<String> = env::args().collect();
   let config1 = Config::build(&arguments).unwrap_or_else(|err|{
    println!("Problem parsing arguments: {err}");
    process::exit(1);
   });
    
   println!("Searching for {}",config1.query);
   println!("In file {}",config1.file_path);
  if let Err(e) = run(config1){
    println!("Application error: {e}");
    process::exit(1);
  };
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path:String
}

impl Config {
    fn build(args:&[String])-> Result<Self,&'static str>{
        if args.len()<3 {
           return Err("Err:: not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
       Ok(Config { query: query, file_path: file_path } )
    }
}

fn run(config: Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("with texts:\n{contents}");
    Ok(())
}

