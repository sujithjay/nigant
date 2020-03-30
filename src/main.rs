#![deny(warnings)]
extern crate reqwest;

use structopt::StructOpt;
use reqwest::Error;

mod parser;
mod request;
mod output;


#[derive(StructOpt, Debug)]
#[structopt(name = "nigant", about = "A thesaurus for the terminal")]
pub struct Cli {
    /// Word to Look-up
    word: String,

    /// Fetch Synonyms for the Word
    #[structopt(short, long)]
    synonyms: bool,

    /// Fetch Etymology of the Word
    #[structopt(short, long)]
    etymology: bool,


    /// Activate Debug Mode
    #[structopt(short, long)]
    debug: bool,

    /// Verbose Mode (-v, -vv, -vvv, & more)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::from_args();
    let response = request::request(&args.word).await?;
    match response.as_str() {
        "404" => 
            output::print_not_found(args.word),
        _ => 
            output::print(&parser::parse(&response), args.word),
    }
    
    Ok(())
}
