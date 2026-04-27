use crossterm::event::{KeyCode, KeyEvent};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::app::{App, Screen, Theme};
use crate::ui::{Modal, ModalType};

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if let Some(modal) = &mut self.modal {
            match &modal.modal_type {
                ModalType::Notification { .. } => {
                    self.modal = None;
                    self.status_message = "Ready".to_string();
                    self.screen = Screen::Main;
                    self.selected = 0;
                    return false;
                }
                ModalType::Confirm { .. } => {
                    match key.code {
                        KeyCode::Left | KeyCode::Right => {
                            modal.toggle_confirm_selection();
                        }
                        KeyCode::Enter => {
                            let should_quit = modal.is_confirm_selected();
                            self.modal = None;
                            return should_quit;
                        }
                        KeyCode::Esc => {
                            self.modal = None;
                        }
                        _ => {}
                    }
                    return false;
                }
                ModalType::Password { .. } => {
                    match key.code {
                        KeyCode::Char(c) => {
                            modal.push_password_char(c);
                        }
                        KeyCode::Backspace => {
                            modal.pop_password_char();
                        }
                        KeyCode::Enter => {
                            let password = modal.get_password().unwrap_or_default();
                            self.modal = None;
                            
                            if password == "dc-ok" {
                                if let Some(network) = self.pending_network.take() {
                                    self.network = network.clone();
                                    self.status_message = format!("✅ Network changed to: {}", network);
                                    self.modal = Some(Modal::success(
                                        "Network Changed",
                                        &format!("Successfully switched to {}", network)
                                    ));
                                }
                                self.screen = Screen::Settings;
                                self.selected = 2;
                            } else {
                                self.modal = Some(Modal::error(
                                    "Invalid Password",
                                    "Incorrect password.\n\nMainnet access denied."
                                ));
                                self.pending_network = None;
                            }
                        }
                        KeyCode::Esc => {
                            self.modal = None;
                            self.pending_network = None;
                            self.status_message = "Network change cancelled".to_string();
                        }
                        _ => {}
                    }
                    return false;
                }
                ModalType::Progress { .. } => {
                    return false;
                }
            }
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
            Screen::ViewAllLicenses | Screen::ViewRevokedHistory => {
                match key.code {
                    KeyCode::Esc => {
                        self.screen = Screen::Main;
                        self.selected = 0;
                        self.status_message = "Returned to main menu".to_string();
                    }
                    _ => {}
                }
                false
            }
            _ => self.handle_input_screen(key),
        }
    }

    fn handle_main_menu(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => {
                self.modal = Some(Modal::confirm(
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
                        self.modal = Some(Modal::confirm(
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
                    let new_network = networks[self.selected].to_string();
                    
                    if new_network == "mainnet" {
                        self.pending_network = Some(new_network);
                        self.modal = Some(Modal::password(
                            "🔐 Mainnet Password Required",
                            "Enter password to switch to mainnet"
                        ));
                    } else {
                        self.pending_network = Some(new_network.clone());
                        self.modal = Some(Modal::progress(
                            "🌐 Switching Network",
                            &format!("Connecting to {}...", new_network),
                            0
                        ));
                    }
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
            self.status_message = "❌ SDK not initialized".to_string();
            self.modal = Some(Modal::error(
                "SDK Error",
                "SDK not initialized.\n\nMake sure ANCHOR_WALLET is set."
            ));
            return;
        }

        for field in &self.form_fields {
            if field.required && field.value.trim().is_empty() {
                self.status_message = format!("❌ '{}' is required", field.label);
                self.modal = Some(Modal::error(
                    "Validation Error",
                    &format!("Field '{}' is required.\n\nPlease fill all required fields.", field.label)
                ));
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
                
                let owner_str_raw = &self.form_fields[0].value;
                let owner_str = self.get_full_pubkey(owner_str_raw).unwrap_or_else(|| owner_str_raw.clone());
                let plan_name = &self.form_fields[1].value;
                let duration_str = &self.form_fields[2].value;
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        self.modal = Some(Modal::error(
                            "Invalid Pubkey",
                            "The owner pubkey is invalid.\n\nMake sure it's a valid Solana pubkey (43-44 characters).\n\nExample:\n3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c"
                        ));
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
                        self.status_message = format!("✅ License issued! Signature: {}", sig);
                        self.modal = Some(Modal::success(
                            "License Issued",
                            &format!("License created successfully!\n\nSignature:\n{}\n\nProduct: {}\nDuration: {} days", sig, product_id, days)
                        ));
                        self.form_fields.clear();
                        self.input.clear();
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                        self.modal = Some(Modal::error(
                            "Transaction Failed",
                            &format!("Failed to issue license:\n\n{}", e)
                        ));
                        self.input.clear();
                    }
                }
            }
            Screen::ExtendLicense => {
                if self.form_fields.len() < 2 {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str_raw = &self.form_fields[0].value;
                let owner_str = self.get_full_pubkey(owner_str_raw).unwrap_or_else(|| owner_str_raw.clone());
                let duration_str = &self.form_fields[1].value;
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        self.modal = Some(Modal::error(
                            "Invalid Pubkey",
                            "The owner pubkey is invalid.\n\nMake sure it's a valid Solana pubkey (43-44 characters).\n\nExample:\n3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c"
                        ));
                        return;
                    }
                };
                
                match client.get_license(&owner.to_string()) {
                    Ok(license) => {
                        if license.is_revoked {
                            self.status_message = "❌ License is revoked".to_string();
                            self.modal = Some(Modal::error(
                                "Cannot Extend Revoked License",
                                &format!(
                                    "This license has been permanently revoked.\n\n\
                                    Owner: {}\n\
                                    Product: {}\n\
                                    Revoked: Yes\n\n\
                                    You cannot extend a revoked license.",
                                    license.owner, license.product_id
                                )
                            ));
                            return;
                        }
                    }
                    Err(_) => {
                        self.status_message = "❌ License not found".to_string();
                        self.modal = Some(Modal::error(
                            "License Not Found",
                            "No license found for this owner.\n\nCreate a license first."
                        ));
                        return;
                    }
                }
                
                let days: i64 = duration_str
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(15);

                self.status_message = "Extending license...".to_string();
                
                match client.op_extend_license(&owner.to_string(), days) {
                    Ok(sig) => {
                        self.status_message = format!("✅ License extended! Signature: {}", sig);
                        self.modal = Some(Modal::success(
                            "License Extended",
                            &format!("License extended successfully!\n\nSignature:\n{}\n\nAdditional days: {}", sig, days)
                        ));
                        self.form_fields.clear();
                        self.input.clear();
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                        self.modal = Some(Modal::error(
                            "Transaction Failed",
                            &format!("Failed to extend license:\n\n{}", e)
                        ));
                        self.input.clear();
                    }
                }
            }
            Screen::ValidateLicense => {
                if self.form_fields.len() < 2 {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str_raw = &self.form_fields[0].value;
                let owner_str = self.get_full_pubkey(owner_str_raw).unwrap_or_else(|| owner_str_raw.clone());
                let plan_name = &self.form_fields[1].value;
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        self.modal = Some(Modal::error(
                            "Invalid Pubkey",
                            "The owner pubkey is invalid.\n\nMake sure it's a valid Solana pubkey (43-44 characters).\n\nExample:\n3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c"
                        ));
                        return;
                    }
                };
                
                use crate::app::LicensePlan;
                let product_id = match LicensePlan::find_by_name(plan_name) {
                    Some(plan) => plan.id,
                    None => {
                        self.status_message = "❌ Invalid plan selected".to_string();
                        self.modal = Some(Modal::error(
                            "Invalid Plan",
                            "The selected plan is invalid."
                        ));
                        return;
                    }
                };
                
                self.status_message = "Validating license...".to_string();
                
                match client.get_license(&owner.to_string()) {
                    Ok(license) => {
                        let is_valid = !license.is_revoked 
                            && license.expires_at > chrono::Utc::now().timestamp()
                            && license.product_id == product_id;
                        
                        if is_valid {
                            self.status_message = "✅ License is valid!".to_string();
                            self.modal = Some(Modal::success(
                                "License Valid",
                                &format!(
                                    "License is VALID ✅\n\nOwner: {}\nProduct: {}\nExpires: {}\nStatus: Active",
                                    license.owner, license.product_id, license.expires_at
                                )
                            ));
                        } else {
                            let reason = if license.is_revoked {
                                "License has been revoked"
                            } else if license.expires_at <= chrono::Utc::now().timestamp() {
                                "License has expired"
                            } else {
                                "Product ID does not match"
                            };
                            
                            self.status_message = "❌ License is invalid!".to_string();
                            self.modal = Some(Modal::error(
                                "License Invalid",
                                &format!(
                                    "License is INVALID ❌\n\nReason: {}\n\nOwner: {}\nProduct: {}\nExpires: {}\nRevoked: {}",
                                    reason, license.owner, license.product_id, license.expires_at, license.is_revoked
                                )
                            ));
                        }
                        self.form_fields.clear();
                        self.input.clear();
                    }
                    Err(e) => {
                        self.status_message = "❌ License not found".to_string();
                        self.modal = Some(Modal::error(
                            "License Not Found",
                            &format!("No license found for this owner.\n\n{}", e)
                        ));
                        self.input.clear();
                    }
                }
            }
            Screen::RevokeLicense => {
                if self.form_fields.is_empty() {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str_raw = &self.form_fields[0].value;
                let owner_str = self.get_full_pubkey(owner_str_raw).unwrap_or_else(|| owner_str_raw.clone());
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        self.modal = Some(Modal::error(
                            "Invalid Pubkey",
                            "The owner pubkey is invalid.\n\nMake sure it's a valid Solana pubkey (43-44 characters).\n\nExample:\n3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c"
                        ));
                        return;
                    }
                };

                self.status_message = "Revoking license...".to_string();
                
                match client.op_revoke_license(&owner.to_string()) {
                    Ok(sig) => {
                        self.status_message = format!("✅ License revoked! Signature: {}", sig);
                        self.modal = Some(Modal::success(
                            "License Revoked",
                            &format!("License revoked successfully!\n\nSignature:\n{}\n\n⚠️  This action is permanent!", sig)
                        ));
                        self.form_fields.clear();
                        self.input.clear();
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                        self.modal = Some(Modal::error(
                            "Transaction Failed",
                            &format!("Failed to revoke license:\n\n{}", e)
                        ));
                        self.input.clear();
                    }
                }
            }
            Screen::ListLicenses => {
                if self.form_fields.is_empty() {
                    self.status_message = "❌ Form not initialized".to_string();
                    return;
                }
                
                let owner_str_raw = &self.form_fields[0].value;
                let owner_str = self.get_full_pubkey(owner_str_raw).unwrap_or_else(|| owner_str_raw.clone());
                
                let owner = match Pubkey::from_str(owner_str.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "❌ Invalid owner pubkey".to_string();
                        self.modal = Some(Modal::error(
                            "Invalid Pubkey",
                            "The owner pubkey is invalid.\n\nMake sure it's a valid Solana pubkey (43-44 characters).\n\nExample:\n3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c"
                        ));
                        return;
                    }
                };
                
                self.status_message = "Fetching license...".to_string();
                
                match client.get_license(&owner.to_string()) {
                    Ok(license) => {
                        use license_sdk::pda::derive_license_pda;
                        let (pda, bump) = derive_license_pda(&owner);
                        self.status_message = format!("✅ License found!");
                        self.modal = Some(Modal::info(
                            "License Details",
                            &format!(
                                "PDA: {}\nBump: {}\n\nOwner: {}\nProduct: {}\nExpires: {}\nRevoked: {}",
                                pda, bump, license.owner, license.product_id, license.expires_at, license.is_revoked
                            )
                        ));
                        self.form_fields.clear();
                        self.input.clear();
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Error: {}", e);
                        self.modal = Some(Modal::error(
                            "License Not Found",
                            &format!("Failed to fetch license:\n\n{}", e)
                        ));
                        self.input.clear();
                    }
                }
            }
            _ => {}
        }
    }
}
