use std::io::{self,BufReader,BufRead};
use std::env;

fn main() {

    let arg = env::args().skip(1).next();
    if arg.is_none(){
        println!("print how to use this command(tbd)");
        return ;
    }
    let pattern = arg.unwrap();

    let input = io::stdin();
    let mut input_buffer = BufReader::new(input);

    loop{
        let mut buffer = String::new();
        match input_buffer.read_line(&mut buffer){
            Ok(0) => break,  //EOF
            Ok(_) => {
                if buffer.contains(&pattern){
                    print!("{}",buffer);
                }
            },
            Err(_) => break,
        }
    }
}
