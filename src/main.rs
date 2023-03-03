use std::io;
use crate::text_processor::{split};

pub mod text_processor;
fn main() {
    let mut user_input:String = String::new();
    println!("Please input your text ?!");
    
    io::stdin().read_line(&mut user_input).expect("Err reading buffer");
    let words = split(user_input.trim());

    for word in words {
        println!("Word: {}",word);
    }
}


