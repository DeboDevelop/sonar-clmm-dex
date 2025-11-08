use anchor_lang::prelude::*;

declare_id!("FBDjF5uQ7Gyvz54757oLhirZJp34BhVGzzRyxvziSKru");

#[program]
pub mod sonar_clmm_dex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
