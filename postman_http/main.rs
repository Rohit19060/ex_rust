extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Read;
// use std::io::Write;
// use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct API {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Postman {
    item: [API],
}

fn main() {
    let mut file_content = String::new();

    let mut data_file = File::open("GitHub API.postman_collection.json").unwrap();

    data_file.read_to_string(&mut file_content).unwrap();

    let res = serde_json::from_str(file_content.as_str());

    if res.is_ok() {
        let p: Postman = res.unwrap();
        println!("{}", p.item[0]);
    } else {
        print!("Sorry! could not parse json",);
    }

    // Access parts of the data by indexing with square brackets.

    // let mut write_file = File::create("api.http").expect("Creation failed");

    // // Write contents to the file
    // write_file
    //     .write("Hello, World!".as_bytes())
    //     .expect("write failed");

    // println!("Created a file data.txt");
}
