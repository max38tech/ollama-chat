// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

// This is the command that will be called from the Svelte frontend
#[tauri::command]
async fn invoke_ollama(app_handle: AppHandle, prompt: String) -> Result<(), String> {
    let client = Client::new();

    // Send a request to the local Ollama API
    let res = client
        .post("http://127.0.0.1:11434/api/generate")
        .json(&OllamaRequest {
            model: "llama3", // Change this to your preferred model
            prompt: &prompt,
            stream: true, // We want to stream the response
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Ollama API Error: {}", res.status()));
    }

    let mut stream = res.bytes_stream();

    // Process the stream of responses from Ollama
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;
        let line = String::from_utf8(chunk.to_vec()).map_err(|e| e.to_string())?;

        // Ollama streams JSON objects separated by newlines
        for part in line.split('\n').filter(|s| !s.is_empty()) {
            match serde_json::from_str::<OllamaResponse>(part) {
                Ok(ollama_response) => {
                    // Emit an event to the frontend with the new chunk of text
                    app_handle.emit_all("ollama-response", ollama_response).unwrap();
                }
                Err(e) => eprintln!("Failed to parse JSON: {}, raw: '{}'", e, part),
            }
        }
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![invoke_ollama])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}