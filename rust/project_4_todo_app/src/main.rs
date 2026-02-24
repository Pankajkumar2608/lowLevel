use std::fs;
use std::io::{self, Write};
use std::env;

#[drive(debug)]

struct Task {
    title: String,
    completed: bool,
    create_at: Time,
    completed_at: Time,
}

fn main () {
    let args = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  add <task>");
        println!("  list");
        println!("  done <index>");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "add" => {
            let title = args[2..].join(" ");
            
        }
    }
}