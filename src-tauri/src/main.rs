#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::Manager;
use window_vibrancy::{apply_blur, apply_mica};

mod analysis;
mod code_design;
mod col_geometric_props;
mod column;
mod frontend_functions;
mod load_factoring;
mod material_props;
mod ref_steel;
mod types;

// use directories::ProjectDirs;
use bincode::serialize_into;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct UserDataProject {
    name: String,
    author: String,
    company: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Project {
    proj_description: UserDataProject,
    // selected_resteel: Vec<ReSteel>,
}

impl Project {
    fn create_tmp_project() -> Project {
        Project {
            proj_description: UserDataProject {
                name: TMP_PROY_NAME.parse().unwrap(),
                author: TMP_PROY_AUTHOR.parse().unwrap(),
                company: TMP_PROY_COMPANY.parse().unwrap(),
            },
        }
    }
}
fn main() {
    let context = tauri::generate_context!();
    let window = tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            #[cfg(target_os = "windows")]
            apply_mica(&win)
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
