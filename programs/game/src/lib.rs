use anchor_lang::prelude::*;

declare_id!("FLh8ons5qBo9DzDsEmhLqs85cHHPyf9kwokRq1L9CP5N");

#[program]
pub mod game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
