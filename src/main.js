// src/main.js
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { writeTextFile, readTextFile } from "@tauri-apps/api/fs";

const convertBtn = document.getElementById("convertBtn");
const outputDiv = document.getElementById("output");

convertBtn.addEventListener("click", async () => {
  try {
    // Open a file dialog to select an image file
    const selected = await open({ multiple: false, filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "bmp", "gif"] }] });
    if (!selected) {
      outputDiv.textContent = "No file selected.";
      return;
    }
    // Call the Rust backend
    const svg = await invoke("vectorize_image", { path: selected });
    // Show SVG content (escaped)
    outputDiv.textContent = svg;
    // Optionally write to a file on disk
    // await writeTextFile("output.svg", svg);
  } catch (e) {
    console.error(e);
    outputDiv.textContent = "Error: " + e;
  }
});
