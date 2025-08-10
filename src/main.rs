use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use clap::Parser;
use serde::{Deserialize, Serialize};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcTransactionConfig, RpcAccountInfoConfig},
    rpc_request::RpcRequest,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signature,
};
use spl_token::instruction::TokenInstruction;
use std::{collections::HashSet, str::FromStr};

mod types;
mod parser;
mod indexer;

use types::*;
use indexer::SolanaIndexer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Wallet address to index
    #[arg(short, long, default_value = "7cMEhpt9y3inBNVv8fNnuaEbx7hKHZnLvR1KWKKxuDDU")]
    wallet: String,

    /// Hours to backfill (default: 24)
    #[arg(short, long, default_value = "24")]
    hours: u64,

    /// Output format (json or pretty)
    #[arg(short, long, default_value = "pretty")]
    output: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🚀 Starting Solana USDC Indexer");
    println!("📍 Wallet: {}", args.wallet);
    println!("⏰ Backfilling last {} hours", args.hours);
    
    let wallet_pubkey = Pubkey::from_str(&args.wallet)
        .map_err(|e| anyhow!("Invalid wallet address: {}", e))?;
    
    let indexer = SolanaIndexer::new()?;
    let transfers = indexer.get_usdc_transfers(wallet_pubkey, args.hours).await?;
    
    match args.output.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&transfers)?);
        }
        _ => {
            print_transfers_pretty(&transfers);
        }
    }
    
    Ok(())
}

fn print_transfers_pretty(transfers: &[UsdcTransfer]) {
    println!("\n📊 USDC Transfer Summary");
    println!("═══════════════════════════════════════════════════════════════");
    println!("Found {} USDC transfers", transfers.len());
    println!();
    
    if transfers.is_empty() {
        println!("No USDC transfers found in the specified time period.");
        return;
    }
    
    let mut total_sent = 0.0;
    let mut total_received = 0.0;
    
    for transfer in transfers {
        let direction = if transfer.is_incoming { "📥 RECEIVED" } else { "📤 SENT" };
        let amount_formatted = format!("{:.6}", transfer.amount);
        
        println!("🕒 {} | {} | ${} USDC", 
                 transfer.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                 direction,
                 amount_formatted);
        println!("   💳 Transaction: {}", transfer.signature);
        
        if transfer.is_incoming {
            println!("   📨 From: {}", transfer.from_address);
            total_received += transfer.amount;
        } else {
            println!("   📤 To: {}", transfer.to_address);
            total_sent += transfer.amount;
        }
        println!();
    }
    
    println!("═══════════════════════════════════════════════════════════════");
    println!("💰 Total Received: ${:.6} USDC", total_received);
    println!("💸 Total Sent: ${:.6} USDC", total_sent);
    println!("📈 Net Change: ${:.6} USDC", total_received - total_sent);
    println!("═══════════════════════════════════════════════════════════════");
}