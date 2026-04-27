use crossterm::event::{KeyCode, KeyEvent};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::app::{App, Screen, Theme};
use crate::ui::ConfirmModal;

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if let Some(modal) = &mut self.confirm_modal {
            match key.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                    modal.toggle_selection();
                }
                KeyCode::Enter => {
                    let should_quit = modal.is_confirm_selected();
                    self.confirm_modal = None;
                    return should_quit;
                }
                KeyCode::Esc | KeyCode::Char('n') => {
                    self.confirm_modal = None;
                }
                _ => {}
            }
            return false;
        }

        if key.code == KeyCode::F(1) || key.code == KeyCode::Char('?') {
            self.show_help_popup = !self.show_help_popup;
            return false;
        }

        if self.show_help_popup {
            self.show_help_popup = false;
            return false;
        }

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
            KeyCode::Char('q') => {
                self.confirm_modal = Some(ConfirmModal::new(
                    "⚠️  Confirm Exit",
                    "Are you sure you want to exit?\n\nUse ←→ to select, Enter to confirm"
                ));
                return false;
            }
            KeyCode::Enter => {
                match self.selected {
                    0 => {
                        self.screen = Screen::IssueLicense;
                        self.setup_form_for_screen();
                        self.status_message = "Issue License - Fill the form".to_string();
                    }
                    1 => {
                        self.screen = Screen::ExtendLicense;
                        self.setup_form_for_screen();
                        self.status_message = "Extend License - Fill the form".to_string();
                    }
                    2 => {
                        self.screen = Screen::ValidateLicense;
                        self.setup_form_for_screen();
                        self.status_message = "Validate License - Fill the form".to_string();
                    }
                    3 => {
                        self.screen = Screen::RevokeLicense;
                        self.setup_form_for_screen();
                        self.status_message = "Revoke License - Fill the form".to_string();
                    }
                    4 => {
                        self.screen = Screen::ListLicenses;
                        self.setup_form_for_screen();
                        self.status_message = "List Licenses - Fill the form".to_string();
                    }
                    5 => {
                        self.screen = Screen::Settings;
                        self.status_message = "Settings - Select an option".to_string();
                        self.selected = 0;
                    }
                    6 => {
                        self.confirm_modal = Some(ConfirmModal::new(
                            "⚠️  Confirm Exit",
                            "Are you sure you want to exit?\n\nUse ←→ to select, Enter to confirm"
                        ));
                    }
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
                self.form_fields.clear();
                self.form_index = 0;
                self.status_message = "Returned to main menu".to_string();
            }
            KeyCode::Up => {
                if self.form_index > 0 {
                    self.form_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.form_index < self.form_fields.len().saturating_sub(1) {
                    self.form_index += 1;
                }
            }
            KeyCode::Left => {
                if self.form_index < self.form_fields.len() {
                    self.form_fields[self.form_index].cycle_prev();
                }
            }
            KeyCode::Right => {
                if self.form_index < self.form_fields.len() {
                    self.form_fields[self.form_index].cycle_next();
                }
            }
            KeyCode::Char(c) => {
                if self.form_index < self.form_fields.len() {
                    let field = &self.form_fields[self.form_index];
                    if field.options.is_empty() {
                        self.form_fields[self.form_index].value.push(c);
                    }
                }
            }
            KeyCode::Backspace => {
                if self.form_index < self.form_fields.len() {
                    let field = &self.form_fields[self.form_index];
                    if field.options.is_empty() {
                        self.form_fields[self.form_index].value.pop();
                    }
                }
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

        for field in &self.form_fields {
            if field.required && field.value.trim().is_empty() {
                self.status_message = format!("❌ '{}' is required", field.label);
                return;
            }
        }

        let client = self.sdk_client.as_ref().unwrap();

        match self.screen {
            Screen::IssueLicense => {
                if self.form_fields.len() < 3 {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str = &self.form_fields[0].value;
                let plan_name = &self.form_fields[1].value;
                let duration_str = &self.form_fields[2].value;
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                use crate::app::LicensePlan;
                let product_id = match LicensePlan::find_by_name(plan_name) {
                    Some(plan) => plan.id,
                    None => {
                        self.status_message = "❌ Invalid plan selected".to_string();
                        return;
                    }
                };
                
                let days: i64 = duration_str
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30);

                self.status_message = "Issuing license...".to_string();
                
                match client.op_issue_license(&owner.to_string(), &product_id, days) {
                    Ok(sig) => {
                        self.status_message = format!("✅ License issued!\nSignature: {}", sig);
                        self.form_fields.clear();
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                    }
                }
            }
            Screen::ExtendLicense => {
                if self.form_fields.len() < 2 {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str = &self.form_fields[0].value;
                let duration_str = &self.form_fields[1].value;
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                let days: i64 = duration_str
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(15);

                self.status_message = "Extending license...".to_string();
                
                match client.op_extend_license(&owner.to_string(), days) {
                    Ok(sig) => {
                        self.status_message = format!("✅ License extended!\nSignature: {}", sig);
                        self.form_fields.clear();
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                    }
                }
            }
            Screen::ValidateLicense => {
                if self.form_fields.len() < 2 {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str = &self.form_fields[0].value;
                let plan_name = &self.form_fields[1].value;
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                use crate::app::LicensePlan;
                let product_id = match LicensePlan::find_by_name(plan_name) {
                    Some(plan) => plan.id,
                    None => {
                        self.status_message = "❌ Invalid plan selected".to_string();
                        return;
                    }
                };
                
                let (license_pda, bump) = client.derive_license_pda(&owner);
                self.status_message = format!(
                    "✅ Ready to validate!\nPDA: {}\nBump: {}\nProduct: {}",
                    license_pda, bump, product_id
                );
                self.form_fields.clear();
            }
            Screen::RevokeLicense => {
                if self.form_fields.is_empty() {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str = &self.form_fields[0].value;
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        return;
                    }
                };

                self.status_message = "Revoking license...".to_string();
                
                match client.op_revoke_license(&owner.to_string()) {
                    Ok(sig) => {
                        self.status_message = format!("✅ License revoked!\nSignature: {}", sig);
                        self.form_fields.clear();
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                    }
                }
            }
            Screen::ListLicenses => {
                if self.form_fields.is_empty() {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str = &self.form_fields[0].value;
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
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
                        self.form_fields.clear();
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                    }
                }
            }
            _ => {}
        }
    }
}
