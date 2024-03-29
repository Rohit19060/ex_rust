#![allow(unused)]

use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{stdout, BufRead, BufReader, BufWriter, ErrorKind, Write}; // from the previous step

// fn first_name() {
//     println!("Rohit");
// }

// fn last_name() {
//     println!("Jain");
// }

// fn sub(a: i32, b: i32) -> i32 {
//     a - b
// }

// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// struct GroceryItem {
//     stock: i32,
//     price: f64,
// }

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();
    say(&message, width, &mut writer).unwrap();

    // println!("What is your name?");
    // let mut name = String::new();
    // let greetings = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");

    // println!("Hello, {}! {}", name.trim_end(), greetings);
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");
    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age: &str = "24";
    // let mut age: u32 = age.trim().parse().expect("Age wasn't' assigned a number");
    // age = age + 1;
    // println!("I'm {} and i want ${}",age,ONE_MIL);

    // Date Types
    // Unsigned Integer : u8, u16, u32, u64, u128, usize
    // Signed Integer: i8, i16, i32, i64, i128, isize

    // println!("Max u32: {}", u32::MAX);
    // println!("Max u64: {}", u64::MAX);
    // println!("Max usize: {}", usize::MAX);
    // println!("Max u128: {}", u128::MAX);
    // println!("Max f32: {}", f32::MAX);
    // println!("Max f64: {}", f64::MAX);

    // let is_true = true;
    // let my_grade = 'A';
    // let my_name = "John";

    // let num_1: f32 = 1.1111111111111;
    // println!("num_1: {}", num_1 + 0.1111111111111);
    // let num_2: f64 = 1.1111111111111;
    // println!("num_3: {}", num_2 + 0.1111111111111);
    // let mut num_3: u32 = 5;
    // let num_4: u32 = 4;

    // println!("5 + 4: {}", num_3 + num_4);
    // println!("5 - 4: {}", num_3 - num_4);
    // println!("5 * 4: {}", num_3 * num_4);
    // println!("5 / 4: {}", num_3 / num_4);
    // println!("5 % 4: {}", num_3 % num_4);

    // num_3 += 1;
    // println!("5 + 1: {}", num_3);
    // num_3 -= 1;
    // println!("5 - 1: {}", num_3);

    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random Number: {}", random_num);

    // let age: i32 = 8;

    // if (age >= 1) && (age <= 18) {
    //     println!("You are a child");
    // } else if (age == 21) || (age >= 65) {
    //     println!("You are a senior");
    // } else {
    //     println!("You are an adult");
    // }

    // let mut my_age = 47;
    // let can_vote = if my_age >= 18 { true } else { false };
    // println!("Can Vote: {}", can_vote);

    // let age2: i32 = 28;
    // match age2 {
    //     1..=18 => println!("You are a child"),
    //     21 | 50 => println!("You are a senior"),
    //     65..=i32::MAX => println!("You are a senior"),
    //     _ => println!("You are an adult"),
    // }

    // let my_age: i32 = 18;
    // let voting_age: i32 = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("You are not old enough to vote"),
    //     Ordering::Greater => println!("You are old enough to vote"),
    //     Ordering::Equal => println!("You are old enough to vote"),
    // }

    // let arr_1 = [1, 2, 3, 4, 5];
    // println!("Array Length: {}", arr_1.len());
    // println!("1st: {}", arr_1[0]);
    // println!("Array: {:?}", arr_1);
    // println!("Array: {:#?}", arr_1);

    // let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut index = 0;

    // loop {
    //     if arr_2[index] % 2 == 0 {
    //         index += 1;
    //         continue;
    //     } else {
    //         println!("Odd: {}", arr_2[index]);
    //     }
    //     if arr_2[index] == 5 {
    //         println!("Found 5");
    //         break;
    //     }

    //     println!("Index: {}", index);
    //     index += 1;
    // }

    // loop {
    //     println!("Guess the number!");
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     match guess.cmp(&random_num) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    // while index < arr_2.len() {
    //     println!("Index: {}", arr_2[index]);
    //     index += 1;
    // }

    // for element in arr_2.iter() {
    //     println!("Element: {}", element);
    // }

    // let my_tuple: (i32, String, u8) = (500, "Rohit".to_string(), 1);
    // let (x, y, z) = my_tuple;
    // println!("The value of y is: {}", y);
    // println!("The value of z is: {}", z);
    // println!("The value of x is: {}", x);

    // let mut st1 = String::new();
    // st1.push('a');
    // st1.push_str("bc");
    // println!("st1: {}", st1);

    // for word in st1.split_whitespace() {
    //     println!("Word: {}", word);
    // }

    // let st2 = st1.replace("a", "Another");
    // println!("st2: {}", st2);

    // let str3 = String::from("x  t b h k k a m c");
    // let mut v1: Vec<char> = str3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("Char: {}", char);
    // }

    // let str4: &str = "Random String";
    // let mut str5: String = str4.to_string();
    // println!("str5: {}", str5);
    // let byte_arr1: &[u8] = str5.as_bytes();
    // let st6: &str = &str5[0..6];
    // println!("st6 length: {}", st6.len());
    // str5.clear();
    // let st6 = String::from("Hello");
    // let str7 = String::from("World");
    // let st8 = st6 + &str7;
    // for char in st8.bytes() {
    //     println!("Char: {}", char);
    // }
    // let int_u8: u8 = 5;
    // let int2_u8: u8 = 6;
    // let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // enum Direction {
    //     Up,
    //     Down,
    //     Left,
    //     Right,
    // }

    // impl Direction {
    //     fn get_direction(&self) {
    //         match self {
    //             Direction::Up => println!("Up"),
    //             Direction::Down => println!("Down"),
    //             Direction::Left => println!("Left"),
    //             Direction::Right => println!("Right"),
    //         }
    //     }

    //     fn get_direction2(&self) -> &str {
    //         match self {
    //             Direction::Up => "Up",
    //             Direction::Down => "Down",
    //             Direction::Left => "Left",
    //             Direction::Right => "Right",
    //         }
    //     }
    // }

    // println!("Direction: {}", Direction::Up.get_direction2());

    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2: Vec<i32> = vec![1, 2, 3, 4, 5];
    // vec2.push(6);
    // vec2.push(7);

    // println!("vec2: {:?}", vec2);
    // println!("1st: {}", vec2[0]);
    // let second: &i32 = &vec2[1];
    // match vec2.get(2) {
    //     Some(second) => println!("3rd: {}", second),
    //     None => println!("No 3rd element"),
    // }

    // for i in &mut vec2 {
    //     *i += 50;
    // }

    // for i in &vec2 {
    //     println!("i: {}", i);
    // }

    // println!("vec2: {:?}", vec2);
    // println!("Vec Length: {}", vec2.len());
    // println!("Pop Value: {:?}", vec2.pop());

    // say_hello();

    // let num_list: Vec<i32> = vec![1, 2, 3, 4, 5];
    // println!("Sum: {}", sum_list(&num_list));

    // Display a message to the user
    // let sum = 2 + 2;
    // let value = 10 - 5;
    // let result = sub(sum, value);
    // println!("Hello, world! {} {} {:?}", sum, value, result);

    // let age = 15;
    // if age >= 21 {
    //     println!("ok to purchase");
    // } else {
    //     println!("not ok to purchase");
    // }

    // let my_bool = true;
    // if my_bool {
    //     println!("hello");
    // } else {
    //     println!("good bye");
    // }

    // let n = 5;
    // if n > 5 {
    //     println!("n is greater than 5");
    // } else if n > 0 {
    //     println!("n is greater than 0");
    // } else {
    //     println!("n is less than 0");
    // }

    // let name = "Rohit Jain";

    // match name {
    //     "Rohit Jain" => println!("hello"),
    //     "Mohit" => println!("good bye"),
    //     _ => println!("default"),
    // }

    // let x = true;

    // match x {
    //     true => println!("true"),
    //     false => println!("false"),
    // }

    // let mut i = 3;
    // let x = 3;

    // loop {
    //     println!("{} {}", i, x);
    //     i -= 1;
    //     if i == 0 {
    //         break;
    //     }
    // }
    // println!("done");

    // let mut i = 1;

    // while i <= 3 {
    //     println!("{}", i);
    //     i += 1;
    // }
    // which_way(Direction::Up);
    // which_way(Direction::Down);
    // which_way(Direction::Right);
    // which_way(Direction::Left);

    // let cereal = GroceryItem {
    //     stock: 10,
    //     price: 3.50,
    // };
    // println!("Stock: {} Price: {}", cereal.stock, cereal.price);
    // print_drink(Drink {
    //     flavor: Flavor::Sparkling,
    //     fluid_oz: 12.0,
    // });
    // print_drink(Drink {
    //     flavor: Flavor::Sweet,
    //     fluid_oz: 12.0,
    // });
    // print_drink(Drink {
    //     flavor: Flavor::Fruity,
    //     fluid_oz: 12.0,
    // });

    // let coord = (2, 3);
    // println!("x: {:?} y: {:?}", coord.0, coord.1);

    // let (x, y) = (5, 6);
    // println!("x: {:?} y: {:?}", x, y);

    // let user_info = ("Emma", 30);
    // println!("Name: {:?} Age: {:?}", user_info.0, user_info.1);

    // let favorites = ("red", 14, "TX", "pizza", "TV Show", "home");
    // let state = favorites.2;
    // println!("State: {:?}", state);
    // println!("Favorite color: {:?}", favorites.0);

    // display_light1(dull);

    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t, "Hello, ").unwrap();
    t.fg(term::color::RED).unwrap();
    writeln!(t, "world!").unwrap();
    t.reset().unwrap();

    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
    display_light(&dull);
    display_light1(dull);
}

// fn sum_list(list: &[i32]) -> i32 {
//     let mut sum = 0;
//     for i in list {
//         sum += i;
//     }
//     return sum;
// }

// fn get_sum(x: i32, y: i32) -> (i32, i32) {
//     return (x + y, x - y);
// }

// fn say_hello() {
//     // println!("Hello");
//     println!("Sum: {}", get_sum(5, 6).0);
//     println!("Sum: {}", get_sum(5, 6).1);
// }

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}
fn display_light1(light: Light) {
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}

enum Light {
    Bright,
    Dull,
}

// fn which_way(go: Direction) {
//     match go {
//         Direction::Up => println!("Up"),
//         Direction::Down => println!("Down"),
//         _ => println!("Left or Right"),
//     }
// }

// enum Flavor {
//     Sparkling,
//     Sweet,
//     Fruity,
// }
// struct Drink {
//     flavor: Flavor,
//     fluid_oz: f64,
// }

// fn print_drink(drink: Drink) {
//     match drink.flavor {
//         Flavor::Sparkling => println!("Flavor: Sparkling, {:?} oz", drink.fluid_oz),
//         Flavor::Sweet => println!("Flavor: Sweet, {:?} oz", drink.fluid_oz),
//         Flavor::Fruity => println!("Flavor: Fruity, {:?} oz", drink.fluid_oz),
//     }
// }
