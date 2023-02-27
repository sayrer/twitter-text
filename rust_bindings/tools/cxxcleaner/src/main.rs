// Copyright 2022 AR Grafica, Inc
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use std::env;
use std::fs::File;
use std::io::{ self, BufRead, Write };
use std::path::Path;

struct CommentState {
    on: bool,
    closing_string: String,
}

impl CommentState {
    fn new() -> CommentState {
        CommentState { 
            on: false,
            closing_string: String::new()
        }
    }

    fn should_comment_out(&mut self, text: &String) -> bool {
        if self.on && text.starts_with(self.closing_string.as_str()) {
            self.on = false;
            return true;
        }

        if text.starts_with("Box<T> Box<T>::in_place(Fields &&...fields) {") 
        || text.starts_with("void Vec<T>::emplace_back(Args &&...args) {") {
            self.on = true;
            self.closing_string = "}".to_string();
            return true;
        }

        if text.starts_with("#ifndef CXXBRIDGE1_IS_COMPLETE") {
            self.on = true;
            self.closing_string = "#endif".to_string();
            return true;          
        }

        if self.on
        || text.starts_with("void panic [[noreturn]]") 
        || text.contains("&&...") {
            return true;
        }

        return false;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut comment_state = CommentState::new();
    if let Ok(lines) = read_lines(args[1].to_string()) {
        for line in lines {
            if let Ok(text) = line {
                if comment_state.should_comment_out(&text) {
                    println!("// {}", text);
                } else {
                    println!("{}", text);
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

