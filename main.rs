extern crate rand;
use std::fs;
use std::io::{Write, Read};

fn file_io() {
    {
        // write
        let string = "Hello, file io!";
        let mut f = fs::File::create("test.txt").unwrap(); // open file, you can write to file.
        // "create" open as write only mode.
        f.write_all(string.as_bytes()).unwrap(); // byte-only

        // file is closed here.
    }

    {
        // read
        let mut f = fs::File::open("test.txt").unwrap();
        // "open" open as read only mode
        let mut buf = vec![];

        f.read_to_end(&mut buf).unwrap(); // read here
        println!("{}", std::str::from_utf8(&buf).unwrap()); // to &str
        // println!("{}", String::from_utf8(b).unwrap()); // or String
    }

    // seek
    // f.seek(std::io::SeekFrom::Start(3)) // seek points head + 3 byte.
}

fn main() {
    // if you writes much bytes, you should buffering

    file_io();

    // string add
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
    
    let tuple = rand::random::<u32>();
    println!("{}, {}, {}", rand::random::<u32>()%32, rand::random::<u32>()%32, rand::random::<u32>()%32)
}