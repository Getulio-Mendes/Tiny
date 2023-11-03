use std::env;
use std::io::ErrorKind;
use std::fs::File;

mod lexical;
mod syntatic;
mod interpreter;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("Usage: tiny <filename>.tiny");
    return;
  }

  let f: File = match File::open(&args[1]){
    Ok(file) => file,
    Err(error) => match error.kind() {  
      ErrorKind::NotFound => panic!("File does not exist"),
      ErrorKind::PermissionDenied => panic!("No permission to read file"),
      _ => panic!("Unknow error"),
      }
    };

  let lexes = lexical::lexical_analizer::lexical_analysis(f);
  syntatic::syntatic_analizer::start(lexes);
}

