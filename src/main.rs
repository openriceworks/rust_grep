extern crate getopts;

use std::fs;
use std::io::{self,BufReader,BufRead,Read};
use std::env;
use getopts::Options;

fn print_usage(program: &str,opts:Options){
    println!();
    let brief = format!("Usage: {} FILE [options]",program);
    print!("{}",opts.usage(&brief));
}

fn stdin_mode(pattern: &str) {

    let input = io::stdin();
    let mut input_buffer = BufReader::new(input);

    //let result = pattern_match(pattern,&mut input_buffer);
    //let mut match_line : Vec<String> = Vec::new();
    loop{
        match pattern_match(pattern,&mut input_buffer){
            Some(o) => print!("{}",o),
            _ => break,
        }
    }
}

fn file_mode(pattern: &str,filepath: String){
    let mut input_buffer = BufReader::new(fs::File::open(filepath).unwrap());
    loop{
        match pattern_match(pattern,&mut input_buffer){
            Some(o) => print!("{}",o),
            _ => break,
        }
    }
}

fn pattern_match<R: Read>(pattern: &str , input_buffer:&mut BufReader<R>) -> Option<String>{

    let mut buffer = String::new();
    match input_buffer.read_line(&mut buffer){
        Ok(0) => {
            return None;
        },
        Ok(_) => {
            if buffer.contains(&pattern){
                return Some(buffer);
            }
            return Some("".to_string());
        },
        Err(e) =>{
            println!("{}",e);
            return None;
        },
    }

}

fn opt_sets(opts: &mut Options){
    opts.optflag("h","help","print this help menu");

    //未実装
    opts.optflag("c","count","print only a count of matching lines per FILE ");

}


fn main() {

    let args : Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opt_sets(&mut opts);

    let matches = match opts.parse(&args[1..]){
        Ok(m) => { m }
        Err(f) => {panic!(f.to_string())}
    };

    if matches.opt_present("h"){
        print_usage(&program,opts);
        return ;
    }

    let pattern = if !matches.free.is_empty(){
        matches.free[0].clone()
    }else{
        return;
    };

    let filename = if !matches.free.is_empty(){
        matches.free.get(1).clone()
    }else{
        None
    };

    match filename {
        Some(o) => file_mode(&pattern,o.to_string()),
        _ => stdin_mode(&pattern),
    }
}
