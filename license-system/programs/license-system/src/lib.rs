use anchor_lang::prelude::*;

declare_id!("46gJKBXChEaV3K4juvfcmAvNEXpCPd51yCDK6Cf4HoeX");

#[program]
pub mod license_system {
    use super::*;

    pub fn issue_license(
        ctx: Context<IssueLicense>,
        owner: Pubkey,
        product_id: String,
        duration_days: i64,
    ) -> Result<()> {
        let license = &mut ctx.accounts.license;
        license.owner = owner;
        license.product_id = product_id;
        license.expires_at = Clock::get()?.unix_timestamp + (duration_days * 24 * 60 * 60);
        license.is_revoked = false;

        msg!("License issued for {} days", duration_days);
        Ok(())
    }

    pub fn extend_license(
        ctx: Context<ExtendLicense>,
        additional_days: i64,
    ) -> Result<()> {
        let license = &mut ctx.accounts.license;
        
        if license.is_revoked {
            return Err(ErrorCode::LicenseRevoked.into());
        }

        let now = Clock::get()?.unix_timestamp;
        let grace_period = 7 * 24 * 60 * 60;
        if license.expires_at < now + grace_period {
            return Err(ErrorCode::LicenseExpired.into());
        }

        license.expires_at += additional_days * 24 * 60 * 60;
        
        msg!("License extended by {} days", additional_days);
        Ok(())
    }

    pub fn revoke_license(ctx: Context<RevokeLicense>) -> Result<()> {
        let license = &mut ctx.accounts.license;
        
        if license.is_revoked {
            return Err(ErrorCode::LicenseAlreadyRevoked.into());
        }

        license.is_revoked = true;
        
        msg!("License revoked");
        Ok(())
    }

    pub fn validate_license(
        ctx: Context<ValidateLicense>,
        product_id: String,
    ) -> Result<bool> {
        let license = &ctx.accounts.license;
        let now = Clock::get()?.unix_timestamp;

        let is_valid = !license.is_revoked 
            && license.expires_at > now 
            && license.product_id == product_id;

        if is_valid {
            msg!("License VALID");
        } else {
            msg!("License INVALID");
        }

        Ok(is_valid)
    }
}

#[account]
#[derive(InitSpace)]
pub struct License {
    pub owner: Pubkey,
    #[max_len(64)]
    pub product_id: String,
    pub expires_at: i64,
    pub is_revoked: bool,
}

#[derive(Accounts)]
pub struct IssueLicense<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + License::INIT_SPACE,
        seeds = [b"license", owner.key().as_ref()],
        bump,
        constraint = owner.key() == authority.key() @ ErrorCode::Unauthorized
    )]
    pub license: Account<'info, License>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExtendLicense<'info> {
    #[account(
        mut,
        seeds = [b"license", license.owner.as_ref()],
        bump,
        constraint = authority.key() == license.owner @ ErrorCode::Unauthorized
    )]
    pub license: Account<'info, License>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeLicense<'info> {
    #[account(
        mut,
        seeds = [b"license", license.owner.as_ref()],
        bump,
        constraint = authority.key() == license.owner @ ErrorCode::Unauthorized
    )]
    pub license: Account<'info, License>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ValidateLicense<'info> {
    pub license: Account<'info, License>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("License has been revoked")]
    LicenseRevoked,

    #[msg("License has already been revoked")]
    LicenseAlreadyRevoked,

    #[msg("License has expired and cannot be extended")]
    LicenseExpired,

    #[msg("Unauthorized to perform this action")]
    Unauthorized,
}