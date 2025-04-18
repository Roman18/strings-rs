fn main() {
    let data = b"\x0a";

    let min_len = 4;
    let mut start = 0;
    let mut end = start + min_len;

    while end < data.len(){
        let slice = &data[start..end];
        if contains_non_printable(slice){
            if slice.len() == min_len{
                start += 1;
                end += 1;
            }else{
                print_byte_pretty(&data[start..end-1]);
                start = end;
                end = start + min_len;
            }
        }else{
            end += 1
        }
    }
}



fn contains_non_printable(data: &[u8]) -> bool{
    let min: u8 = 33;
    let max: u8 = 126;
    for byte in data{
        if *byte < min || *byte > max{
            return true;
        }
    } 

    false
}

fn print_byte_pretty(data: &[u8]) -> (){
    for byte in data{
        print!("{}", *byte as char);
    }
    println!();
}