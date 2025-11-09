// programs/sonar-clmm-dex/src/lib.rs
use anchor_lang::prelude::*;
pub mod instructions;

use instructions::*;

declare_id!("FBDjF5uQ7Gyvz54757oLhirZJp34BhVGzzRyxvziSKru");

#[program]
pub mod sonar_clmm_dex {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        tick_spacing: u16,
        initial_sqrt_price: u128,
    ) -> Result<()> {
        instructions::initialize_pool::handler(ctx, tick_spacing, initial_sqrt_price)
    }
}