use license_sdk::LicenseClient;
use crate::app::{Theme, FormField};

#[derive(Debug, Clone)]
pub enum Screen {
    Main,
    IssueLicense,
    ExtendLicense,
    ValidateLicense,
    RevokeLicense,
    ListLicenses,
    Settings,
    SettingsTheme,
    SettingsNetwork,
}

pub struct App {
    pub screen: Screen,
    pub selected: usize,
    pub input: String,
    pub status_message: String,
    pub sdk_client: Option<LicenseClient>,
    pub theme: Theme,
    pub form_fields: Vec<FormField>,
    pub form_index: usize,
    pub network: String,
    pub settings_items: Vec<String>,
    pub show_help_popup: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Main,
            selected: 0,
            input: String::new(),
            status_message: String::from("Welcome to License System TUI"),
            sdk_client: None,
            theme: Theme::default(),
            form_fields: Vec::new(),
            form_index: 0,
            network: "localnet".to_string(),
            settings_items: vec![
                "Theme".to_string(),
                "Network".to_string(),
                "Back".to_string(),
            ],
            show_help_popup: false,
        }
    }

    pub fn menu_items(&self) -> Vec<&str> {
        match self.screen {
            Screen::Settings | Screen::SettingsTheme | Screen::SettingsNetwork => {
                vec!["← Back", "Theme", "Network"]
            }
            _ => vec![
                "Issue License",
                "Extend License",
                "Validate License",
                "Revoke License",
                "List Licenses",
                "Settings",
                "Exit",
            ],
        }
    }

    pub fn settings_menu_items(&self) -> Vec<&str> {
        vec!["← Back", "Theme", "Network"]
    }

    pub fn theme_options(&self) -> Vec<&str> {
        vec!["Dc Studio", "Dark", "Light", "Dracula", "Nord", "Gruvbox"]
    }

    pub fn network_options(&self) -> Vec<&str> {
        vec!["Localnet", "Devnet", "Mainnet"]
    }

    pub fn content_selected(&self) -> usize {
        match self.screen {
            Screen::SettingsTheme | Screen::SettingsNetwork => self.selected,
            _ => 0,
        }
    }

    pub fn init_sdk(&mut self, keypair_path: &str) -> anyhow::Result<()> {
        let keypair = solana_sdk::signature::read_keypair_file(keypair_path)
            .map_err(|e| anyhow::anyhow!("Failed to load keypair: {}", e))?;
        
        let client = LicenseClient::new_localnet(keypair);
        self.sdk_client = Some(client);
        self.status_message = "Connected to Solana localnet".to_string();
        Ok(())
    }
}
