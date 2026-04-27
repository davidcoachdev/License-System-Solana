use crossterm::event::{KeyCode, KeyEvent};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::app::{App, Screen, Theme};

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.screen {
            Screen::Main => self.handle_main_menu(key),
            Screen::Settings => self.handle_settings_menu(key),
            Screen::SettingsTheme => self.handle_theme_selection(key),
            Screen::SettingsNetwork => self.handle_network_selection(key),
            _ => self.handle_input_screen(key),
        }
    }

    fn handle_main_menu(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => return true,
            KeyCode::Enter => {
                match self.selected {
                    0 => {
                        self.screen = Screen::IssueLicense;
                        self.status_message = "Issue License - Enter: owner_pubkey,product_id,days".to_string();
                    }
                    1 => {
                        self.screen = Screen::ExtendLicense;
                        self.status_message = "Extend License - Enter: owner_pubkey,additional_days".to_string();
                    }
                    2 => {
                        self.screen = Screen::ValidateLicense;
                        self.status_message = "Validate License - Enter: owner_pubkey,product_id".to_string();
                    }
                    3 => {
                        self.screen = Screen::RevokeLicense;
                        self.status_message = "Revoke License - Enter: owner_pubkey".to_string();
                    }
                    4 => {
                        self.screen = Screen::ListLicenses;
                        self.status_message = "List Licenses - Enter: owner_pubkey".to_string();
                    }
                    5 => {
                        self.screen = Screen::Settings;
                        self.status_message = "Settings - Select an option".to_string();
                        self.selected = 0;
                    }
                    6 => return true,
                    _ => {}
                }
            }
            KeyCode::Down => {
                self.selected = (self.selected + 1) % self.menu_items().len();
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                } else {
                    self.selected = self.menu_items().len() - 1;
                }
            }
            _ => {}
        }
        false
    }

    fn handle_settings_menu(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.screen = Screen::Main;
                self.selected = 0;
                self.status_message = "Returned to main menu".to_string();
            }
            KeyCode::Enter => {
                match self.selected {
                    0 => {
                        self.screen = Screen::Main;
                        self.selected = 0;
                        self.status_message = "Returned to main menu".to_string();
                    }
                    1 => {
                        self.screen = Screen::SettingsTheme;
                        self.selected = 0;
                        self.status_message = "Select a theme from the right panel".to_string();
                    }
                    2 => {
                        self.screen = Screen::SettingsNetwork;
                        self.selected = 0;
                        self.status_message = "Select a network from the right panel".to_string();
                    }
                    _ => {}
                }
            }
            KeyCode::Down => {
                self.selected = (self.selected + 1) % 3;
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                } else {
                    self.selected = 2;
                }
            }
            _ => {}
        }
        false
    }

    fn handle_theme_selection(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.screen = Screen::Settings;
                self.selected = 1;
                self.status_message = "Settings - Select an option".to_string();
            }
            KeyCode::Enter => {
                let themes = vec!["dcdev", "dark", "light", "dracula", "nord", "gruvbox"];
                if self.selected < themes.len() {
                    let theme_name = themes[self.selected];
                    self.theme = Theme::by_name(theme_name);
                    let display_name = if theme_name == "dcdev" { "Dc Studio" } else { theme_name };
                    self.status_message = format!("✅ Theme changed to: {}", display_name);
                    self.screen = Screen::Settings;
                    self.selected = 1;
                }
            }
            KeyCode::Down => {
                self.selected = (self.selected + 1) % 6;
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                } else {
                    self.selected = 5;
                }
            }
            _ => {}
        }
        false
    }

    fn handle_network_selection(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.screen = Screen::Settings;
                self.selected = 2;
                self.status_message = "Settings - Select an option".to_string();
            }
            KeyCode::Enter => {
                let networks = vec!["localnet", "devnet", "mainnet"];
                if self.selected < networks.len() {
                    self.network = networks[self.selected].to_string();
                    self.status_message = format!("✅ Network changed to: {}", self.network);
                    self.screen = Screen::Settings;
                    self.selected = 2;
                }
            }
            KeyCode::Down => {
                self.selected = (self.selected + 1) % 3;
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                } else {
                    self.selected = 2;
                }
            }
            _ => {}
        }
        false
    }

    fn handle_input_screen(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.screen = Screen::Main;
                self.selected = 0;
                self.input.clear();
                self.status_message = "Returned to main menu".to_string();
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.execute_action();
            }
            _ => {}
        }
        false
    }

    pub fn execute_action(&mut self) {
        if self.sdk_client.is_none() {
            self.status_message = "Error: SDK not initialized".to_string();
            return;
        }

        let client = self.sdk_client.as_ref().unwrap();

        match self.screen {
            Screen::IssueLicense => {
                let parts: Vec<&str> = self.input.split(',').collect();
                if parts.len() != 3 {
                    self.status_message = "Format: owner_pubkey,product_id,days".to_string();
                    return;
                }
                
                let owner = match Pubkey::from_str(parts[0].trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                let product_id = parts[1].trim().to_string();
                let days: i64 = match parts[2].trim().parse() {
                    Ok(d) => d,
                    Err(_) => {
                        self.status_message = "Invalid days number".to_string();
                        return;
                    }
                };

                self.status_message = "Issuing license...".to_string();
                
                match client.op_issue_license(&owner.to_string(), &product_id, days) {
                    Ok(sig) => {
                        self.status_message = format!("✅ License issued!\nSignature: {}", sig);
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                    }
                }
            }
            Screen::ExtendLicense => {
                let parts: Vec<&str> = self.input.split(',').collect();
                if parts.len() != 2 {
                    self.status_message = "Format: owner_pubkey,additional_days".to_string();
                    return;
                }
                
                let owner = match Pubkey::from_str(parts[0].trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                let days: i64 = match parts[1].trim().parse() {
                    Ok(d) => d,
                    Err(_) => {
                        self.status_message = "Invalid days number".to_string();
                        return;
                    }
                };

                self.status_message = "Extending license...".to_string();
                
                match client.op_extend_license(&owner.to_string(), days) {
                    Ok(sig) => {
                        self.status_message = format!("✅ License extended!\nSignature: {}", sig);
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                    }
                }
            }
            Screen::ValidateLicense => {
                let parts: Vec<&str> = self.input.split(',').collect();
                if parts.len() != 2 {
                    self.status_message = "Format: owner_pubkey,product_id".to_string();
                    return;
                }
                
                let owner = match Pubkey::from_str(parts[0].trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                let product_id = parts[1].trim();
                let (license_pda, bump) = client.derive_license_pda(&owner);
                self.status_message = format!(
                    "✅ Ready to validate!\nPDA: {}\nBump: {}",
                    license_pda, bump
                );
            }
            Screen::RevokeLicense => {
                let owner = match Pubkey::from_str(self.input.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };

                self.status_message = "Revoking license...".to_string();
                
                match client.op_revoke_license(&owner.to_string()) {
                    Ok(sig) => {
                        self.status_message = format!("✅ License revoked!\nSignature: {}", sig);
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                    }
                }
            }
            Screen::ListLicenses => {
                let owner = match Pubkey::from_str(self.input.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                self.status_message = "Fetching license...".to_string();
                
                match client.get_license(&owner.to_string()) {
                    Ok(license) => {
                        use license_sdk::pda::derive_license_pda;
                        let (pda, bump) = derive_license_pda(&owner);
                        self.status_message = format!(
                            "✅ License Found!\nPDA: {}\nBump: {}\nOwner: {}\nProduct: {}\nExpires: {}\nRevoked: {}",
                            pda, bump, license.owner, license.product_id, license.expires_at, license.is_revoked
                        );
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                    }
                }
            }
            _ => {}
        }
        
        self.input.clear();
    }
}
