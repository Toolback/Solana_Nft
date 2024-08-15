use anchor_lang::prelude::*;

declare_id!("RFyHYiMJp57fZWKwDj1VPefavUYyn1kWpbCX9YNm3xB");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}