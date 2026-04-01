use std::env; // To read command line arguments
use chrono::Local;
use colored::*; // For colored terminal output


use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Note{
    timestamp: String,
    category: String,
    content: String,
}





fn main() {
    // 1. Collect what the user typed into a list (Vector)
    let args: Vec<String> = env::args().collect();


    // 2. Check if they actually gave us a command
    if args.len() < 2 {
        println!("{}", "Usage: cargo run -- [add|list|search|delete|clear|export] \"your note here\"".red().bold());
        return;
    }



    let command = &args[1];


    // 3. Decide what to do based on the command
    match command.as_str() {
    "add" => add_note(&args),
    "list" => list_notes(),
    "clear" => clear_notes(),
    "search" => search_notes(&args),
    "delete" => delete_note(&args),
    "export" => export_notes(),
        _ => println!("Unknown command: '{}'. Use add, list, search, delete, clear, or export.", command),
    }
}



    fn add_note(args: &[String]) {
        println!("🕵️ DEBUG ARGS: {:?}", args);
    // 1. Find the category (anything starting with "--")
    let category = args.iter()
        .find(|a| a.starts_with("--"))
        .map(|c| c.trim_start_matches("--"))
        .unwrap_or("general");

    // 2. Grab EVERY word that isn't a flag and join them with spaces!
    let content = args.iter()
        .skip(2)
        .filter(|a| !a.starts_with("-"))
        .cloned()
        .collect::<Vec<String>>()
        .join(" ");

    // 3. Make sure they actually typed something
    if content.trim().is_empty() {
        println!("{}", "❌ Error: What is the note content?".red());
        return;
    }

    // 4. Create the note
    let new_note = Note {
        timestamp: Local::now().format("%d-%m-%Y %H:%M:%S").to_string(),
        category: category.to_uppercase(),
        content,
    };

    // 5. Read the existing notes
    let mut notes: Vec<Note> = match std::fs::read_to_string("notes.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    };

    // 6. Add the new note and save
    notes.push(new_note);
    let json = serde_json::to_string_pretty(&notes).expect("Failed to serialize");
    std::fs::write("notes.json", json).expect("Failed to write file");

    println!("✅ Note added to JSON database!");
}
        
    fn list_notes() {
            
            // Read the whole file and print it
            let content = std::fs::read_to_string("notes.json")
                .unwrap_or_else(|_| "[]".to_string());
            
            let notes: Vec<Note> = match serde_json::from_str(&content) {
                Ok(parsed_notes) => parsed_notes,
                Err(_) => {
                    println!("{}", "No notes found.".red());
                    return;
                }
            };
            
            if notes.is_empty() {
                println!("{}", "Your journal is empty. Add a note first!".yellow());
                return;
                }
            
                println!("\n--- {} ---", "Your Digital Journal".bold().cyan());
            


        for (i, note) in notes.iter().enumerate() {
        println!(
            "{}. [{}] [{}] {}",
            (i + 1).to_string().yellow(),
            note.timestamp.bright_black(),
            note.category.green().bold(), // Accessing the field directly
            note.content.white()
            
        );
    }
}
        
    
    fn delete_note(args: &[String]) {
    if args.len() < 3 {
        println!("{}", "Usage: cargo run -- delete [number]".yellow());
        return;
    }

    // 1. Parse the string "2" into the number 2
    let target_index: usize = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "❌ Error: Please provide a valid number!".red());
            return;
        }
    };

    // 2. Read the current notes
    let content = std::fs::read_to_string("notes.json").unwrap_or_default();
    let mut notes: Vec<Note> = serde_json::from_str(&content).unwrap_or_default();

    // 3. Safety Check: Is the number within the list?
    if target_index == 0 || target_index > notes.len() {
        println!("❌ Error: Note #{} does not exist. (Total notes: {})", target_index, notes.len());
        return;
    }

    // 4. Remove the item (Human 1 = Computer 0)
    if target_index <= notes.len() {
        let removed = notes.remove(target_index - 1);


    // 5. Save back to file with clean newlines
    let json = serde_json::to_string_pretty(&notes).expect("Failed to serialize");
        std::fs::write("notes.json", json).expect("Failed to write file");
        
        println!("🗑️  Note: \"{}\" deleted successfully.", removed.content.red());
    } else {
        println!("❌ Error: Note #{} does not exist.", target_index);
    }
}



    fn clear_notes() {
            match std::fs::remove_file("notes.json") {
                Ok(_) => println!("✅ All notes cleared!"),
                Err(_) => println!("clearance failed"),
            }
        }


    fn search_notes(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: cargo run -- search [word]");
        return;
    }
    let target = &args[2];

    // Read the file. If it doesn't exist, stop here.
    let data = std::fs::read_to_string("notes.json").unwrap_or_else(|_| "[]".to_string());
let notes: Vec<Note> = serde_json::from_str(&data).unwrap_or_default();
    
    println!("Searching for: '{}'...", target);   
    // Loop through each line in the file
for note in notes {
    if note.content.to_lowercase().contains(&target.to_lowercase()) || 
    note.category.to_lowercase().contains(&target.to_lowercase()) {
    println!("🎯 Found: [{}] [{}] {}", 
        note.timestamp.bright_black(), 
        note.category.green(), 
        note.content
    );
}
}
    }
    

fn export_notes() {
    let content = std::fs::read_to_string("notes.json").unwrap_or_default();
    let notes: Vec<Note> = serde_json::from_str(&content).unwrap_or_default();
    if notes.is_empty(){
        println!("No notes found to export.");
        return;
    }
    let mut html = String::from("<!DOCTYPE html><html><head><meta charset='utf-8'>");
    html.push_str("<title>Expert Potato Notes</title>");
    html.push_str("<style>body { font-family: sans-serif; padding: 50px; background: #fafafa; } .card { background: white; padding: 20px; border-radius: 10px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); max-width: 600px; margin: auto; } h1 { color: #2c3e50; border-bottom: 2px solid #3498db; } ul { list-style: none; padding: 0; } li { padding: 10px 0; border-bottom: 1px solid #eee; } .date { font-weight: bold; color: #3498db; margin-right: 10px; }</style>");
    html.push_str("</head><body><div class='card'><h1>📝 My Notes</h1><ul>");

    
    let mut count = 0;
    // Just ONE loop through the notes
    for note in notes {
        count += 1;
        html.push_str(&format!(
            "<li><span class='date'>[{}]</span> <span class='cat'>[{}]</span> {}</li>", 
            note.timestamp, note.category, note.content
        ));
    }

    html.push_str("</ul></div></body></html>");

    std::fs::write("notes.html", &html).expect("Failed to write HTML file");
    println!("✅ Exported {} notes to notes.html", count);
}
    

   

