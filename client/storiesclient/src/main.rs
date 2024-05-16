use serde_json::Result;
use serde::{Deserialize, Serialize};
use reqwest;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

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

fn main() {
    // first we need to get the data from the server
    let resp = match reqwest::blocking::get("https://httpbin.org/ip") {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    };
    // let data: Story = match serde_json::from_str(&resp) {
    //     Ok(data) => data,
    //     Err(err) => panic!("Error: {}", err)
    // };
    // let data: Story = serde_json::from_str(resp.as_str())?;
    // println!("{}", data.origin);

//    let mut file = File::open("../data/stories.json");
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

    let original_story = data.adventure;

    // next we need to get all the <> fields and then ask the user to replace them
    let mut place_holders: Vec<String> = Vec::new();
    let mut new_words: Vec<String> = Vec::new();
    let add_char: bool = true;
    let mut word_temp: String = String::from("");
    // print_variable_type(&original_story);
    let pick_one = rand::thread_rng().gen_range(0..2);
    println!("random number: {}", pick_one);
    let story_iter = original_story.iter();
    for story in story_iter {
        println!("{}", story);
    }
    println!("Random pick: {}", original_story[pick_one]);
}
