
use clap::{
    Parser,
    ValueEnum
};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub file_name: String,
    #[arg(short, default_value = "4", help="Min length of string")]
    pub n: usize,
    #[arg(short, default_value = "decimal", value_enum, help="Print offset befor each line")]
    pub t: Option<Raddix>,
    #[arg(short, help="Print the name of the file before each string")]
    pub f: bool,


}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Raddix{
    Octal,
    Decimal,
    Hex,    
}