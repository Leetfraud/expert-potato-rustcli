use std::env; // To read command line arguments
use std::fs::OpenOptions; // To open files with specific permissions
use std::io::Write; // To write data to files
use chrono::Local;
use colored::*; // For colored terminal output

fn main() {
    // 1. Collect what the user typed into a list (Vector)
    let args: Vec<String> = env::args().collect();

    // 2. Check if they actually gave us a command
    if args.len() < 2 {
        println!("Usage: cargo run -- [add|list] \"your note\"");
        return;
    }

    let command = &args[1];

    // 3. Decide what to do based on the command
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Error: What is the note content?");
                return;
            }
            let note = &args[2];

            let timestamp = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
            
            // Open (or create) notes.txt and append the new note
            let file_result = OpenOptions::new()
                .append(true)
                .create(true)
                .open("notes.txt");
            let mut file = match file_result {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to open notes.txt: {}", e);
                    return;
                }
            };
    

                        

            writeln!(file, "[{}] {}", timestamp, note).expect("Failed to write note");
            println!("✅ Note saved!");
        }
        "list" => {
            // Read the whole file and print it
            let content = std::fs::read_to_string("notes.txt")
                .unwrap_or_else(|_| String::from("No notes found yet."));
            
            println!("\n--- {} ---", "Your Digital Journal".bold().cyan());


            for line in content.lines() {
        // Find the end of the timestamp ']'
        if let Some(pos) = line.find(']') {
            let (date, note) = line.split_at(pos + 1);
            // Print date in bright black (grey) and the note in white
            println!("{} {}", date.bright_black(), note.white());
        } else {
            println!("{}", line);
        }
    }
        }

        "clear" => {
            match std::fs::remove_file("notes.txt") {
                Ok(_) => println!("✅ All notes cleared!"),
                Err(_) => println!("clearance failed"),
            }
        }

    "search" => {
    if args.len() < 3 {
        println!("Usage: cargo run -- search [word]");
        return;
    }
    let target = &args[2];

    // Read the file. If it doesn't exist, stop here.
    let content_file = std::fs::read_to_string("notes.txt");
    let content = match content_file {
        Ok(c) => c,
        Err(_) => {
            println!("No notes found to search through.");
            return;
        }
    };
        

    println!("Searching for: '{}'...", target);

    // Loop through each line in the file
    for line in content.lines() {
        if line.to_lowercase().contains(&target.to_lowercase()) {
            println!("🎯 Found: {}", line);
        }
    }}
    "delete" => {
        if args.len() < 3 {
        println!("Usage: cargo run -- delete [number]");
        return;
    }

    // 1. Convert the input string to a number (starting from 1 for humans)
    let target_index: usize = args[2].parse().expect("Invalid number");

    // 2. Read the file
    let content = std::fs::read_to_string("notes.txt").expect("Could not read file");
    let mut lines: Vec<&str> = content.lines().collect();

    // 3. Check if the index is valid
    if target_index == 0 || target_index > lines.len() {
        println!("❌ Error: Note #{} does not exist.", target_index);
        return;
    }

    // 4. Remove the item (we subtract 1 because computers start at 0)
    lines.remove(target_index - 1);

    // 5. Join the remaining lines back together and save
    let new_content = lines.join("\n");
    std::fs::write("notes.txt", new_content).expect("Failed to save changes");

    println!("🗑️ Deleted note #{}", target_index);
    }

        
        _ => println!("Unknown command. Try 'add' or 'list' or 'clear'."),
    }}


