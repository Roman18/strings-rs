use std::{
    fs,
    error::Error,
    io::Read,
};

fn main() -> Result<(), Box<dyn Error>>{
    let args = std::env::args().collect::<Vec<_>>();
    let file_name = match args.get(1){
        Some(f) => f,
        None => return Err("Usage: ./string-rs file.txt".into()),
    };
    let mut data = Vec::new();
    let mut fd = fs::File::open(file_name)?;
    let _ = fd.read_to_end(&mut data)?;

    let min_len = 4;
    let mut start = 0;
    let mut end = start + min_len;

    while end < data.len(){
        let slice = &data[start..end];
        if end == data.len() - 1  && !contains_non_printable(slice){
            print_bytes_pretty(&data[start..=end]);
        }
        if contains_non_printable(slice){
            if slice.len() == min_len{
                start += 1;
                end += 1;
            }else{
                print_bytes_pretty(&data[start..end-1]);
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

fn print_bytes_pretty(data: &[u8]) -> (){
    for byte in data{
        print!("{}", *byte as char);
    }
    println!();
}