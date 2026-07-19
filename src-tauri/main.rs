// src-tauri/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{self, Manager};
use serde::Deserialize;
use std::path::PathBuf;
use image::DynamicImage;
use potrace::{Bitmap, Potrace, TurnPolicy};

#[derive(Deserialize)]
struct VectorizeParams {
    path: String,
}

#[tauri::command]
fn vectorize_image(params: VectorizeParams) -> Result<String, String> {
    // Load the image from the given path
    let img = image::open(&params.path).map_err(|e| e.to_string())?;
    // Convert to grayscale
    let gray = img.to_luma8();
    // Create bitmap for potrace
    let bitmap = Bitmap::from_luma_image(&gray);
    // Trace to SVG
    let path = Potrace::new()
        .turn_policy(TurnPolicy::Minority)
        .threshold(128)
        .trace(&bitmap)
        .map_err(|e| e.to_string())?;
    // Generate SVG string
    let svg = path.to_svg_string(1.0);
    Ok(svg)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![vectorize_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
