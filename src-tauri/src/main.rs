// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ fs::File, io::Read, process::Command };
use native_dialog::FileDialog;
use tauri::{
    AboutMetadata,
    CustomMenuItem,
    Menu,
    MenuItem,
    Submenu,
    api::process::restart,
    Env,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn open_awa_file() -> Option<String> {
//     // Gets the file path
//     let file_path = match
//         FileDialog::new()
//             .set_location("~") // Makes the default location 'User'
//             .add_filter("awa File Type", &["awa"]) // Accepts only 'awa' file types
//             .show_open_single_file()
//     {
//         // Opens a single file
//         Ok(Some(path)) => path,
//         _ => {
//             println!("No file selected.");
//             return None;
//         }
//     };

//     // Read the awa file
//     match File::open(&file_path) {
//         Ok(mut awa_file) => {
//             let mut contents = String::new();
//             if let Err(err) = awa_file.read_to_string(&mut contents) {
//                 println!("Failed to awa read file: {}", err);
//                 return None;
//             }
//             Some(contents)
//         }
//         Err(err) => {
//             println!("Failed to open file: {}", err);
//             None
//         }
//     }
// }

// #[tauri::command]
// fn send_awa_file(awa_file_code: String) -> String {
//     event::emit_all("file_content", awa_file_code) // Use the imported `event` module
//         .expect("Failed to send file content to frontend");
// }

#[tauri::command]
fn get_code(code: String) {
    println!("{}", code);
}

#[tauri::command]
fn run() {}

#[tauri::command]
fn stop() {
}

fn app_menu() -> Menu {
    //~ File Options
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let file = Submenu::new("File", Menu::new().add_item(open).add_item(quit).add_item(close));
    let menu = Menu::new().add_submenu(file);
    menu
}
fn main() {
    let app_menu = app_menu();
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            // send_awa_file,
            // open_awa_file,
            get_code,
            ])
        .menu(app_menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => std::process::exit(0),
                "open" => {
                    // if let Some(result) = open_awa_file() {
                    //     println!("{}", result);
                        // send_awa_file(result);
                        event.window().emit("open_file", "").expect("msg");
                    // }
                }
                "close" => println!("close"),
                _ => println!("ok"),
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
