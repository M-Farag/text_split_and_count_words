use std::io;
use crate::text_processor::*;

pub mod text_processor;
fn main() {
    let mut user_input:String = String::new();
    println!("Please input your text ?!");

    io::stdin().read_line(&mut user_input).expect("Err reading buffer");

    // Clean text
    let user_input = character_replacer(user_input, (',',' '));
    let user_input = character_replacer(user_input, ('.',' '));
    let user_input = character_replacer(user_input, ('?',' '));
    let user_input = character_replacer(user_input, ('"',' '));

    // Segment text into words
    let words = split(user_input.trim());

    // Count words
    let words_count = words_counter(words);

    for (word, count) in words_count
    {
        println!("Word: {}, Count: {}",word,count);
    }
}


