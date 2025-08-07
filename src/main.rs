use std::process::exit;

//use upto 255 d as u8 is used

fn main() {
    let args = std::env::args().collect();
    let mut c = parse_arguments(&args).unwrap_or_else(|err|{println!("User error: {}",err);exit(1)});
    let k = match c.func.as_str(){
        "enc" => {enc(&c.msg,&mut c.shift)},
        "dec" => {dec(&c.msg,&mut c.shift)},
        _ => {println!("Incorrect Function Type - Encode[enc]/Decode[dec]");exit(1)}
    };
    let output = k.join(" ");
    println!("{}",output)

}

struct Config{
    msg: Vec<String>,
    shift: i32,
    func: String
}

fn parse_arguments(arg: &Vec<String>) -> Result<Config,&'static str>{ 
    if arg.len() < 3{ Err("Invalid Input")}
    else {
        let mut m = Vec::new();
        for i in 2..arg.len()-1{
            m.push(arg[i].clone());
        }
        Ok(Config { msg: m, func: arg[1].clone() ,shift: arg[arg.len()-1].parse().expect("Please provide a valid number") })
    }
    
}

fn enc(m: &Vec<String>, d: &mut i32) -> Vec<String>{
    let mut ciphervec: Vec<String> = Vec::new();
    if *d < 0{*d = 26 + *d;}
    for i in m{
        let mut cipher = String::from("");
        for(_,mut c) in i.char_indices(){
            if c.is_uppercase(){
                c = c.to_ascii_lowercase();
                let shift = ((c as u8 - b'a') as i32 + *d).rem_euclid(26);
                let n = (shift as u8) + b'a';
                cipher.push((n as char).to_ascii_uppercase());
            }else{
            let shift = ((c as u8 - b'a') as i32 + *d).rem_euclid(26);
            let n = (shift as u8) + b'a';
            cipher.push(n as char);}
        }
        ciphervec.push(cipher);
    }
    ciphervec
}

fn dec(c: &Vec<String>, d: &mut i32) -> Vec<String>{
    *d = *d * -1;
    let mut messagevec: Vec<String> = Vec::new();
    if *d < 0{*d = 26 + *d;}
    for i in c{
        let mut message = String::from("");
        for(_,mut c) in i.char_indices(){
            if c.is_uppercase(){
                c = c.to_ascii_lowercase();
                let shift = ((c as u8 - b'a') as i32 + *d).rem_euclid(26);
                let n = (shift as u8) + b'a';
                message.push((n as char).to_ascii_uppercase());
            }else{
            let shift = ((c as u8 - b'a') as i32 + *d).rem_euclid(26);
            let n = (shift as u8) + b'a';
            message.push(n as char);}
        }
        messagevec.push(message);
    }
    messagevec
}