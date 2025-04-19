use std::{
    fs,
    error::Error,
    io::Read,
};

use clap::Parser;

mod args_parser;
use args_parser::{Args, Raddix};

fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();
    let min_len = args.n;

    if min_len < 1{
        return Err(format!("strings-rs: minimum string length is too small: {}", min_len).into());
    }

    let mut data = Vec::new();
    let mut fd = fs::File::open(&args.file_name)?;
    let _ = fd.read_to_end(&mut data)?;

    let mut start = 0;
    let mut end = start + min_len;

    while end < data.len(){
        let slice = &data[start..end];
        if end == data.len() - 1  && !contains_non_printable(slice){
            print_bytes_pretty(&data[start..=end], &args, start);
        }
        if contains_non_printable(slice){
            if slice.len() == min_len{
                start += 1;
                end += 1;
            }else{
                print_bytes_pretty(&data[start..end-1], &args, start);
                start = end;
                end = start + min_len;
            }
        }else{
            end += 1
        }
    }
    Ok(())
}



fn contains_non_printable(data: &[u8]) -> bool{
    let min: u8 = 32;
    let max: u8 = 126;
    for byte in data{
        if *byte < min || *byte > max{
            return true;
        }
    } 

    false
}

fn print_bytes_pretty(data: &[u8], args: &Args, offset: usize) -> (){
    if args.f{
        print!("{}:  ",
            std::path::Path::new(&args.file_name)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap()
        );
    }
    if let Some(r) = args.t{
        match r {
            Raddix::Octal   => print!("{:o} ", offset),
            Raddix::Decimal => print!("{} ", offset),
            Raddix::Hex     => print!("{:x} ", offset),
        };
    }
    for byte in data{
        print!("{}", *byte as char);
    }
    println!();
}