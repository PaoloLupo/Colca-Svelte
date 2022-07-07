#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::analysis::axial_design::axial_calculate_column_area;
use crate::frontend_functions::new_init_column;
use tauri::Manager;
use window_vibrancy::apply_mica;

mod analysis;
mod code_design;
mod col_geometric_props;
mod column;
mod frontend_functions;
mod load_factoring;
mod material_props;
mod ref_steel;
mod types;

// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// struct UserDataProject {
//     name: String,
//     author: String,
//     company: String,
// }
//
// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// struct Project {
//     proj_description: UserDataProject,
//     // selected_resteel: Vec<ReSteel>,
// }
//
// impl Project {
//     fn create_tmp_project() -> Project {
//         Project {
//             proj_description: UserDataProject {
//                 name: TMP_PROJECT_NAME.parse().unwrap(),
//                 author: TMP_PROJECT_AUTHOR.parse().unwrap(),
//                 company: TMP_PROJECT_COMPANY.parse().unwrap(),
//             },
//         }
//     }
// }
fn main() {
    let column = new_init_column(
        "Column".to_string(),
        vec!["Axial".to_string()],
        "ACI".to_string(),
        240.0,
        300.0,
        vec![10.0],
        "21 MPa".to_string(),
        "Grado 60".to_string(),
        vec!["6mm".to_string(), "8mm".to_string(), "1/2".to_string()],
        0.02,
        "rectangulares".to_string(),
    );
    let area_axial = axial_calculate_column_area(&column);

    println!(
        "la columna es{:?} el area calculada es {:?}",
        column, area_axial
    );

    let context = tauri::generate_context!();
    let window = tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            #[cfg(target_os = "windows")]
            apply_mica(&win)
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler!(new_init_column))
        .run(context)
        .expect("error while running tauri application");
}
