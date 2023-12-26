// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod verify;
use std::env;

use commands::{create_task, LoginUser,delete_task};
use verify::verify;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        let mut login_user = LoginUser {
            username: args[1].clone(),
            password: args[2].clone(),
            login_type: args[3].parse().unwrap(),
        };
        verify(&mut login_user).await.unwrap();
        return;
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_task,delete_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
