use rand::Rng;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Story {
    adventure: Vec<String>,
    romcom: Vec<String>,
    family: Vec<String>,
    fantasy: Vec<String>,
}

impl Story {
    fn get_genre(&self, genre: Genre) -> &[String] {
        match genre {
            Genre::Adventure => &self.adventure,
            Genre::RomCom => &self.romcom,
            Genre::Family => &self.family,
            Genre::Fantasy => &self.fantasy,
        }
    }
}

#[derive(Copy, Clone)]
enum Genre {
    Adventure,
    RomCom,
    Family,
    Fantasy,
}

impl Genre {
    fn parse(input: &str) -> Result<Genre, String> {
        match input.to_uppercase().trim() {
            "ADVENTURE" => Ok(Genre::Adventure),
            "ROMCOM" => Ok(Genre::RomCom),
            "FAMILY" => Ok(Genre::Family),
            "FANTASY" => Ok(Genre::Fantasy),
            _ => Err(String::from(
                "Please enter a valid story genre: adventure, romcom, family, or fantasy",
            )),
        }
    }
}

fn print_variable_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn vec_to_string(items: &[String]) -> () {
    let joined = items.join("");
    println!("{}", joined);
}

fn main() {
    let mut input = String::new();
    println!("Select a type of story (adventure, romcom, family, fantasy): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let story_genre = match Genre::parse(&input) {
        Ok(genre) => genre,
        Err(err) => panic!("{err}"),
    };

    let contents = match std::fs::read_to_string("./data/stories.json") {
        Ok(contents) => contents,
        Err(err) => panic!("could not read file: {}", err),
    };

    let data: Story = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(err) => panic!("Error: {}", err),
    };

    let chosen_stories = data.get_genre(story_genre);

    // next we need to get all the <> fields and then ask the user to replace them
    let mut place_holders: Vec<String> = Vec::new();
    let pick_one = rand::thread_rng().gen_range(0..chosen_stories.len());

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

    for placeholder in place_holders.iter() {
        println!("Enter a {}: ", placeholder);
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        replacements.insert(
            placeholder.to_string(),
            user_input.clone().trim().to_string(),
        );
        println!("{placeholder}: {user_input}");
    }

    for placeholder in place_holders.iter() {
        println!("old word: {}", placeholder);
        println!("new word: {}", replacements[placeholder]);
        chosen_story = chosen_story.replacen(placeholder, &replacements[placeholder].clone(), 1);
    }

    println!("Your new story:\n{}", chosen_story);
}
