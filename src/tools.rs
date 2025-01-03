use std::process::{Command, Output};
use std::io::{self, Read, Write};
use std::fs::{self, File, create_dir_all};
use std::path::{Path, PathBuf};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use zip::ZipWriter;
use zip::write::FileOptions;

pub fn exec(command: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .with_context(|| format!("Failed to execute command: {}", command))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        Err(anyhow::anyhow!("Command failed: {}, Error: {}", command, error_message))
    }
}

pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

pub async fn fetch_url(url: &str) -> Result<String> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to fetch URL: {}", url))?;

    if response.status().is_success() {
        let body = response.text().await
            .with_context(|| format!("Failed to read response body from URL: {}", url))?;
        Ok(body)
    } else {
        Err(anyhow::anyhow!("Failed to fetch URL: {}, Status: {}", url, response.status()))
    }
}

pub fn xor_encrypt_decrypt(data: &mut Vec<u8>, key: &str) -> Result<Vec<u8>> {
    let key_bytes = key.as_bytes();
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key_bytes[i % key_bytes.len()];
    }
    Ok(data.to_vec())
}

pub fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn initialize_global_tmp_path() -> Result<String> {
    let tmp_dir = get_temporary_directory();
    let random_string = generate_random_string(25);
    let temp_path = format!("{}{}", tmp_dir, random_string);
    create_temporary_folder(&temp_path)?;
    Ok(temp_path)
}

pub fn get_temporary_directory() -> String {
    std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp/".to_string())
}

pub fn create_temporary_folder(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path))?;
    }
    Ok(())
}

pub fn copy_file_to_directory(source_path: &str, destination_directory: &str) -> Result<()> {
    create_dir_all(destination_directory)
        .with_context(|| format!("Failed to create directory: {}", destination_directory))?;

    let source_file = Path::new(source_path);
    let file_name = source_file.file_name()
        .ok_or(anyhow::anyhow!("Failed to get file name from path: {}", source_path))?
        .to_str()
        .ok_or(anyhow::anyhow!("Failed to convert file name to string"))?;

    let destination_path = Path::new(destination_directory).join(file_name);

    fs::copy(source_path, destination_path)
        .with_context(|| format!("Failed to copy file from {} to {}", source_path, destination_path.display()))?;

    Ok(())
}

pub fn get_home_directory() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
}

pub fn create_directory(path: &str) -> Result<()> {
    create_dir_all(path)
        .with_context(|| format!("Failed to create directory: {}", path))?;
    Ok(())
}

pub fn copy_directory_with_files(src: &str, dest: &str) -> Result<()> {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);

    if !src_path.exists() {
        return Err(anyhow::anyhow!("Source directory does not exist: {}", src));
    }

    if !dest_path.exists() {
        create_dir_all(dest_path)
            .with_context(|| format!("Failed to create destination directory: {}", dest))?;
    }

    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let src_entry_path = entry.path();
        let dest_entry_path = dest_path.join(entry.file_name());

        if src_entry_path.is_dir() {
            copy_directory_with_files(src_entry_path.to_str().unwrap(), dest_entry_path.to_str().unwrap())?;
        } else {
            fs::copy(&src_entry_path, &dest_entry_path)
                .with_context(|| format!("Failed to copy file from {} to {}", src_entry_path.display(), dest_entry_path.display()))?;
        }
    }
    Ok(())
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn compress_folder(folder_path: &str) -> Result<()> {
    let file = File::create(format!("{}.zip", folder_path))?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let path = Path::new(folder_path);
    if path.is_dir() {
        compress_directory(&mut zip, path, path, options)?;
    } else if path.is_file() {
         let file = File::open(path)?;
         let mut buffer = Vec::new();
         let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("file");
         zip.start_file(file_name, options)?;
         let _ = file.read_to_end(&mut buffer)?;
         zip.write_all(&buffer)?;
    }
    zip.finish()?;
    Ok(())
}

fn compress_directory<W: Write + io::Seek>(
    zip: &mut ZipWriter<W>,
    base_path: &Path,
    path: &Path,
    options: FileOptions,
) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let name = entry_path.strip_prefix(base_path)?.to_str().unwrap();

        if entry_path.is_dir() {
            zip.add_directory(name, options)?;
            compress_directory(zip, base_path, &entry_path, options)?;
        } else {
            let file = File::open(&entry_path)?;
            let mut buffer = Vec::new();
            let _ = file.read_to_end(&mut buffer)?;
            zip.start_file(name, options)?;
            zip.write_all(&buffer)?;
        }
    }
    Ok(())
}

pub fn remove_item_at_path(path: &str) -> Result<()> {
    let path = Path::new(path);
    if path.exists() {
        if path.is_dir() {
            fs::remove_dir_all(path)
                .with_context(|| format!("Failed to remove directory: {}", path.display()))?;
        } else {
            fs::remove_file(path)
                .with_context(|| format!("Failed to remove file: {}", path.display()))?;
        }
    }
    Ok(())
}

pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD.decode(data)
        .with_context(|| format!("Failed to decode base64 string: {}", data))
} 