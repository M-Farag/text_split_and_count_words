use std::{collections::HashMap, char};

pub fn split(text_input:&str) ->Vec<&str>
{
    let mut words:Vec<&str> = Vec::new();
    let mut start_index:usize = 0;

    for (end_index,letter) in text_input.chars().enumerate()
    {
        if letter == ' '
        {
            words.push( &text_input[start_index..end_index]);
            start_index = end_index + 1;
        }
    }
    words.push(&text_input[start_index..]);
    words
}

pub fn words_counter(words:Vec<&str>) -> HashMap<&str,usize>
{
    let mut words_count:HashMap<&str,usize> = HashMap::new();
    for word in words {
        let counter = words_count.entry(word).or_insert(0);
        *counter += 1;
    }
    words_count
}

pub fn character_replacer(user_input:String,replacements:(char,char)) -> String
{
    let mut new_string:String = String::new();

    for mut letter in user_input.chars()
    {
        if letter == replacements.0
        {
            letter = replacements.1
        }
        new_string.push(letter);
    }

    new_string
}