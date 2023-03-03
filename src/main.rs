use std::io;
use crate::text_processor::{split,words_counter};

pub mod text_processor;
fn main() {
    let mut user_input:String = String::new();
    println!("Please input your text ?!");

    io::stdin().read_line(&mut user_input).expect("Err reading buffer");
    let words = split(user_input.trim());

    let words_count = words_counter(words);

    println!("Word  count");
    for (word, count) in words_count
    {
        println!("{}    {}",word,count);
    }
}


