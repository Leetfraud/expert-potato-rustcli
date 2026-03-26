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
        
    fn list_notes() {
            // Read the whole file and print it
            let content = std::fs::read_to_string("notes.txt")
                .unwrap_or_else(|_| String::from("No notes found yet."));
            
            println!("\n--- {} ---", "Your Digital Journal".bold().cyan());


        for (i, line) in content.lines().enumerate() {
        if let Some(pos) = line.find(']') {
            let (date, note) = line.split_at(pos + 1);
            // Print the number in yellow, date in grey, note in white
            println!("{}. {} {}", (i + 1).to_string().yellow(), date.bright_black(), note.white());
        } else {
            println!("{}. {}", (i + 1).to_string().yellow(), line);
        }
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
    let content = std::fs::read_to_string("notes.txt").unwrap_or_default();
    let mut lines: Vec<&str> = content.lines().collect();

    // 3. Safety Check: Is the number within our list?
    if target_index == 0 || target_index > lines.len() {
        println!("❌ Error: Note #{} does not exist. (Total notes: {})", target_index, lines.len());
        return;
    }

    // 4. Remove the item (Human 1 = Computer 0)
    lines.remove(target_index - 1);

    // 5. Save back to file with clean newlines
    let mut file = std::fs::File::create("notes.txt").expect("Failed to open file");
    for line in lines {
        writeln!(file, "{}", line).expect("Failed to write to file");
    }

    println!("🗑️  Note #{} deleted successfully.", target_index);
}




    fn clear_notes() {
            match std::fs::remove_file("notes.txt") {
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
    }
    }

fn export_notes() {
    let content = std::fs::read_to_string("notes.txt").unwrap_or_default();
    if content.is_empty(){
        println!("No notes found to export.");
        return;
    }
    let mut html = String::from("<!DOCTYPE html><html><head><meta charset='utf-8'>");
    html.push_str("<title>Expert Potato Notes</title>");
    html.push_str("<style>body { font-family: sans-serif; padding: 50px; background: #fafafa; } .card { background: white; padding: 20px; border-radius: 10px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); max-width: 600px; margin: auto; } h1 { color: #2c3e50; border-bottom: 2px solid #3498db; } ul { list-style: none; padding: 0; } li { padding: 10px 0; border-bottom: 1px solid #eee; } .date { font-weight: bold; color: #3498db; margin-right: 10px; }</style>");
    html.push_str("</head><body><div class='card'><h1>📝 My Notes</h1><ul>");

    let mut count = 0;
    for line in content.lines() {
        if line.trim().is_empty() { continue; } // Skip empty lines
        
        count += 1;
        if let Some(pos) = line.find(']') {
            let (date, text) = line.split_at(pos + 1);
            html.push_str(&format!("<li><span class='date'>{}</span>{}</li>", date, text));
        } else {
            html.push_str(&format!("<li>{}</li>", line));
        }
    }

    html.push_str("</ul></div></body></html>");

    // CRITICAL: Make sure this write happens!
    std::fs::write("notes.html", &html).expect("Failed to write HTML file");
    
    println!("✅ Exported {} notes to notes.html", count);
    }

