use serde_json::Result;
use serde::{Deserialize, Serialize};
use reqwest;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::io;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Story {
    adventure: Vec<String>,
    romcom: Vec<String>,
    family: Vec<String>,
    horror: Vec<String>,
    fantasy: Vec<String>
}

fn print_variable_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}


fn parse_input(input: &String) -> String {
    let mut story_genre = String::new();
    match input.to_uppercase().trim() {
        "ADVENTURE" => {story_genre = "adventure".to_string()},
        "ROMCOM" => {story_genre = "romcom".to_string()},
        "FAMILY" => {story_genre = "family".to_string()},
        "HORROR" => {story_genre = "horror".to_string()},
        "FANTASY" => {story_genre = "fantasy".to_string()},
        _ => panic!("Please enter a valid story genre: adventure, romcom, family, horror, or fantasy")
    };
    return story_genre;
}

fn vec_to_string(items: &[String]) -> () {
    let items_iterator = items.iter();
    for item in items_iterator {
        println!("{}", item);
    }
}

fn main() {

    let mut input = String::new();
    println!("Select a type of story (adventure, romcom, family, horror, fantasy): ");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut story_genre = String::new();
    story_genre = parse_input(&input);

    // first we need to get the data from the server
    let resp = match reqwest::blocking::get("https://httpbin.org/ip") {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    };

    let mut file: File = match File::open("./data/stories.json") {
        Ok(file) => file,
        Err(err) => panic!("could not open: {}", err)
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    match file.read_to_string(&mut contents) {
        Ok(_) => print!("Read from file"),
        Err(err) => panic!("could not read file: {}", err)
    }

    let data: Story = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(err) => panic!("Error: {}", err)
    };

    let mut chosen_stories: Vec<String> = Vec::new();
    match story_genre.as_str() {
        "adventure" => chosen_stories = data.adventure,
        "romcom" => chosen_stories = data.romcom,
        "family" => chosen_stories = data.family,
        "horror" => chosen_stories = data.horror,
        "fantasy" => chosen_stories = data.fantasy,
        _ => panic!("should not be here...")
    };

    // next we need to get all the <> fields and then ask the user to replace them
    let mut place_holders: Vec<String> = Vec::new();
    // print_variable_type(&original_story);
    let pick_one = rand::thread_rng().gen_range(0..2);
    
    let mut chosen_story = chosen_stories[pick_one].clone();
    println!("Random pick: {}", chosen_story);
    let mut adding_chars = false;
    let mut temp_item = String::new();
    let mut replacements = HashMap::new();
    for character in chosen_story.chars() {
        if (character == '<') {
            adding_chars = true;
            temp_item.push(character);
        } else if (character == '>') {
            adding_chars = false;
            temp_item.push(character);
            place_holders.push(temp_item.clone());
            temp_item = String::new();
        } else if (adding_chars) {
            temp_item.push(character);
        }
    }
    // vec_to_string(&place_holders);
    let mut user_input = String::new();
    for placeholder in place_holders.iter() {
        println!("Enter a {}: ", placeholder);
        user_input = "".to_string();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");
        replacements.insert(placeholder.to_string(), user_input.clone().trim().to_string());
        println!("{placeholder}: {user_input}");
    }

    let placeholder_iterator = place_holders.iter();
    for placeholder in placeholder_iterator {
        println!("old word: {}", placeholder);
        println!("new word: {}", replacements[placeholder]);
        chosen_story = chosen_story.replacen(placeholder, &replacements[placeholder].clone(), 1);
    }

    println!("Your new story:\n{}", chosen_story);
}

