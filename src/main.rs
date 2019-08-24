#![allow(dead_code)]

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

mod module;
use module::another_module;

mod collections;
use collections::{hash_maps, strings, vectors};

mod structs;
use structs::custom_structs;

mod generics;
use generics::{generic_enums, generic_method, generic_structs};

mod traits;
use traits::trait_example;

use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

use std::io::Cursor;

#[derive(Debug)]
enum Address {
    Standard,
    _Modern,
}

#[derive(Debug)]
enum Phone {
    Office(String),
    Personal(String),
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    address: Address,
    phone: (Phone, Phone),
}

fn main() {
    another_module::is_red();
    module::something();
    // Absolute path
    crate::module::another_module::is_red();
    // Relative path
    module::another_module::is_red();

    module::something();

    vectors::list();
    vectors::multiple_types();
    strings::general_string();
    hash_maps::hash();

    let user = custom_structs::create_user(
        String::from("John"),
        String::from("the_password_here"),
        String::from("john@email.ext"),
    );
    println!("{:?}", user);

    let send_message = custom_structs::send_message();
    println!("{:?}", send_message);

    println!("{}", generic_structs::render_colors());
    println!("{}", generic_structs::render_hex_colors());
    println!("{}", generic_structs::render_points(36, 88.889));
    let v: Vec<u8> = vec![1, 2, 251, 189];
    generic_structs::render_other_points("1234".to_string(), v);
    // this one fail Error(usernameAlreadyExist { message: "the username is already taken", type_error: "custom_error", severity: 3 })
    println!("{:?}", generic_enums::check_username(String::from("john")));
    // this one return Ok(true)
    println!(
        "{:?}",
        generic_enums::check_username(String::from("john snow"))
    );

    generic_method::create_point();
    let mixed_rectangle = generic_method::create_rectangle(30, 90);
    println!("mixed rectangle is => {}", mixed_rectangle);

    // Traits
    let article = trait_example::create_article("this is a test".to_string(), "lalala".to_string());
    println!("{}", article);

    trait_example::hello();

    let to_pass = String::from("john snow");
    let sslice = string_slice(&to_pass);
    println!("{:?}", sslice);
    println!("Hello, world!");
    let f = File::open("example.txt");
    match f {
        Ok(file) => println!("file {:?}", file),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("Not Found error");
            }
        }
    }

    println!("error_handle: {:?}", error_handle());

    println!("error_handle_2: {:?}", error_handle_2());

    println!("{:?}", write_file());

    let read_block_32 = read_from_binary_file();
    println!("{:?}", read_block_32);

    let user = create_user();
    println!("new user => {:?}", user);
}

fn error_handle() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("example.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn error_handle_2() -> Result<String, io::Error> {
    fs::read_to_string("Stream_file_12-8_11.8.59")
}

fn read_from_binary_file() -> Vec<u16> {
    // let mut buf = [0; 304];
    // let ss = File::open("Stream_file_12-8_11.8.59")
    //     .unwrap()
    //     .read(&mut buf)
    //     .unwrap();
    // let s = String::from_utf8_lossy(&buf);
    // let split = s.split(":-:-:CDV_STINGER:+:+:INIZIO_XYZ").collect::<Vec<&str>>();
    // let cursor = Cursor::new(&k[50..]).read_u16::<LittleEndian>().unwrap();

    let k = fs::read("Stream_file_12-8_11.8.59").unwrap();
    let mut data = Vec::new();
    let mut n = 48;
    while n < 304 {
        let cursor = Cursor::new(&k[n..]).read_u16::<LittleEndian>().unwrap();
        data.push(cursor);
        n += 2;
    }
    data
}

fn write_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // File::open("example.txt")?.read_to_string(&mut s)?;
    let mut f = File::open("example.txt")?;
    f.read_to_string(&mut s)?;

    if s.len() == 0 {
        fs::write("example.txt", String::from("something new"))?;
        f.read_to_string(&mut s)?;
    }

    Ok(s)
}

fn create_user() -> User {
    User {
        name: String::from("Pippo"),
        age: 19,
        address: Address::Standard,
        phone: (
            Phone::Office(String::from("398 999 12 23")),
            Phone::Personal(String::from("333 444 55 88")),
        ),
    }
}

fn string_slice(s: &String) -> &str {
    &s[..4]
}

// Fixing the largest Function with Trait Bounds
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    &largest
}
