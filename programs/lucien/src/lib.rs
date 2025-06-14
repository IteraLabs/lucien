#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

/// To initialize state accounts
use crate::state::{
    model_params::ModelParameters,
    model_results::ModelResults,
    model_features::ModelFeatures,
    model_experiments::ModelExperiments,
    prices::PriceHistory
};

// Program ID
declare_id!("GWVfXsjLBfbLvHpuwXPR6jYsCPSCcbMXaKBZvNLM1bgJ");

/// To execute instructions
pub use instructions::initialize;

/// OnChain Model
pub mod models;

/// OnChain Instructions
pub mod instructions;

/// OnChain State Trackers
pub mod state;

/// OnChain Data IO
pub mod data;

/// SVM operations related errors
pub mod errors;

#[program]
pub mod lucien {

    use super::*;

    /// Initialize model parameters account
    pub fn initialize_params(ctx: Context<InitializeParams>, weights: [f32; 5], bias: f32,
    ) -> Result<()> {
        instructions::initialize::initialize_params(ctx, weights, bias)
    }
     
    /// Initialize model results account
    pub fn initialize_results(ctx: Context<InitializeResults>) -> Result<()> {
        instructions::initialize::initialize_results(ctx)
    }
    
    /// Initialize model features account
    pub fn initialize_features(ctx: Context<InitializeFeatures>) -> Result<()> {
        instructions::initialize::initialize_features(ctx)
    }
    
    /// Initialize model experiments account
    pub fn initialize_experiments(ctx: Context<InitializeExperiments>) -> Result<()> {
        instructions::initialize::initialize_experiments(ctx)
    }
    
    /// Initialize price history account
    pub fn initialize_price_history(ctx: Context<InitializePriceHistory>) -> Result<()> {
        instructions::initialize::initialize_price_history(ctx)
    }

    /// Fetch latest prices from Pyth oracle and update price history
    pub fn fetch_and_store_price(ctx: Context<FetchPrice>) -> Result<()> {
        instructions::fetch_price::fetch_and_store_price(ctx)
    }

    /// Calculate features from price history
    pub fn calculate_features(ctx: Context<CalculateFeatures>) -> Result<()> {
        instructions::calculate_features::calculate_features(ctx)
    }

    /// Run ML inference using current features and model parameters
    pub fn run_inference(ctx: Context<ModelInference>) -> Result<()> {
        instructions::run_inference::run_inference(ctx)
    }
    
}

#[derive(Accounts)]
pub struct InitializeParams<'info> {

    #[account(
        init,
        payer = authority,
        space = ModelParameters::LEN,
        seeds = [b"model_params", authority.key().as_ref()],
        bump
    )]
    pub model_params: Account<'info, ModelParameters>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct InitializeResults<'info> {

    #[account(
        init,
        payer = authority,
        space = ModelResults::LEN,
        seeds = [b"model_results", authority.key().as_ref()],
        bump
    )]
    pub model_results: Account<'info, ModelResults>,
    
    #[account(seeds = [b"model_params", authority.key().as_ref()], bump)]
    pub model_params: Account<'info, ModelParameters>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct InitializeFeatures<'info> {

    #[account(
        init,
        payer = authority,
        space = ModelFeatures::LEN,
        seeds = [b"model_features", authority.key().as_ref()],
        bump
    )]
    pub model_features: Account<'info, ModelFeatures>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct InitializeExperiments<'info> {

    #[account(
        init,
        payer = authority,
        space = ModelExperiments::LEN,
        seeds = [b"model_experiments", authority.key().as_ref()],
        bump
    )]
    pub model_experiments: Account<'info, ModelExperiments>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct InitializePriceHistory<'info> {

    #[account(
        init,
        payer = authority,
        space = PriceHistory::LEN,
        seeds = [b"price_history", authority.key().as_ref()],
        bump
    )]
    pub price_history: Account<'info, PriceHistory>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct FetchPrice<'info> {

    #[account(
        mut,
        seeds = [b"price_history", authority.key().as_ref()],
        bump
    )]

    pub price_history: Account<'info, PriceHistory>,
    pub price_update: Account<'info, PriceUpdateV2>,
    
    #[account(mut)]
    pub authority: Signer<'info>,

}

#[derive(Accounts)]
pub struct CalculateFeatures<'info> {
    #[account(
        seeds = [b"model_params", authority.key().as_ref()],
        bump
    )]
    pub model_params: Account<'info, ModelParameters>,
    
    #[account(
        mut,
        seeds = [b"model_results", authority.key().as_ref()],
        bump
    )]
    pub model_results: Account<'info, ModelResults>,
    
    #[account(
        mut,
        seeds = [b"model_features", authority.key().as_ref()],
        bump
    )]
    pub model_features: Account<'info, ModelFeatures>,

    #[account(
        seeds = [b"price_history", authority.key().as_ref()],
        bump
    )]
    pub price_history: Account<'info, PriceHistory>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModelInference<'info> {
    #[account(
        seeds = [b"model_params", authority.key().as_ref()],
        bump
    )]
    pub model_params: Account<'info, ModelParameters>,
    
    #[account(
        mut,
        seeds = [b"model_results", authority.key().as_ref()],
        bump
    )]
    pub model_results: Account<'info, ModelResults>,
    
    #[account(
        seeds = [b"model_features", authority.key().as_ref()],
        bump
    )]
    
    pub model_features: Account<'info, ModelFeatures>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

