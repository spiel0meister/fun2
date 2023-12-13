use std::env::args;
use std::fs::read_to_string;
use std::io::{self, Error, ErrorKind};

mod tokenizer;

fn get_filepath() -> io::Result<String> {
    let args_: Vec<String> = args().collect();

    if args_.len() < 2 {
        return Err(Error::new(ErrorKind::Other, "Not enough args"));
    }

    let file_path = &args_[1];

    Ok(file_path.clone())
}

fn main() -> io::Result<()> {
    let filepath = get_filepath()?;
    let content = read_to_string(filepath)?;
    let mut main_tokenizer = tokenizer::Tokenizer::new(content);
    let tokens = main_tokenizer.tokenize();

    println!("{:?}", tokens);

    Ok(())
}
