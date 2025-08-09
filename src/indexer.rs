use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use reqwest::Client;
use serde_json::json;
use std::{collections::HashSet, time::Duration as StdDuration};
use futures::future::join_all;

use crate::types::*;
use crate::parser::TransactionParser;

pub struct SolanaIndexer {
    http_client: Client,
    rpc_url: String,
}

impl SolanaIndexer {
    pub fn new() -> Result<Self> {
        // Use Solana mainnet RPC endpoint
        let rpc_url = "https://api.mainnet-beta.solana.com".to_string();

        let http_client = Client::builder()
            .timeout(StdDuration::from_secs(30))
            .build()?;

        Ok(Self {
            http_client,
            rpc_url,
        })
    }

    pub async fn get_usdc_transfers(
        &self,
        wallet: &str,
        hours_back: u64,
    ) -> Result<Vec<UsdcTransfer>> {
        let cutoff_time = Utc::now() - Duration::hours(hours_back as i64);
        let mut all_transfers = Vec::new();
        let mut processed_signatures = HashSet::new();

        println!("🔍 Fetching transaction signatures for wallet...");

        // Get all signatures for the wallet
        let signatures = self.get_signatures_for_address(wallet).await?;

        println!("📝 Found {} recent signatures", signatures.len());

        // Filter signatures by time
        let recent_signatures: Vec<_> = signatures
            .into_iter()
            .filter(|sig| {
                if let Some(block_time) = sig.block_time {
                    let tx_time = chrono::Utc
                        .timestamp_opt(block_time, 0)
                        .single()
                        .unwrap_or(Utc::now());
                    tx_time >= cutoff_time
                } else {
                    true // Include transactions without block time
                }
            })
            .collect();

        println!(
            "⏰ {} signatures within {} hour window",
            recent_signatures.len(),
            hours_back
        );

        // Process transactions in batches
        for chunk in recent_signatures.chunks(10) {
            let batch_futures: Vec<_> = chunk
                .iter()
                .filter(|sig| !processed_signatures.contains(&sig.signature))
                .map(|sig| {
                    processed_signatures.insert(sig.signature.clone());
                    self.get_transaction(&sig.signature)
                })
                .collect();

            let batch_results = join_all(batch_futures).await;

            for result in batch_results {
                match result {
                    Ok(Some(transaction)) => {
                        match TransactionParser::parse_usdc_transfers(&transaction, wallet) {
                            Ok(mut transfers) => {
                                all_transfers.append(&mut transfers);
                            }
                            Err(e) => {
                                eprintln!("⚠️ Error parsing transaction: {}", e);
                            }
                        }
                    }
                    Ok(None) => {
                        // Transaction not found or null
                    }
                    Err(e) => {
                        eprintln!("⚠️ Error fetching transaction: {}", e);
                    }
                }
            }

            // Small delay between batches to be respectful to RPC
            tokio::time::sleep(StdDuration::from_millis(100)).await;
        }

        // Sort transfers by timestamp (newest first)
        all_transfers.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        println!("✅ Found {} USDC transfers", all_transfers.len());

        Ok(all_transfers)
    }

    async fn get_signatures_for_address(
        &self,
        address: &str,
    ) -> Result<Vec<GetSignaturesForAddressResponse>> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSignaturesForAddress",
            "params": [
                address,
                {
                    "limit": 1000,
                    "commitment": "confirmed"
                }
            ]
        });

        let response = self
            .http_client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;

        let rpc_response: RpcResponse<Vec<GetSignaturesForAddressResponse>> =
            response.json().await?;
        Ok(rpc_response.result)
    }

    async fn get_transaction(&self, signature: &str) -> Result<Option<TransactionResponse>> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransaction",
            "params": [
                signature,
                {
                    "encoding": "json",
                    "commitment": "confirmed",
                    "maxSupportedTransactionVersion": 0
                }
            ]
        });

        let response = self
            .http_client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;

        let rpc_response: RpcResponse<Option<TransactionResponse>> = response.json().await?;
        Ok(rpc_response.result)
    }
}

impl SolanaIndexer {
    // Alternative implementation without futures dependency
    pub async fn get_usdc_transfers_sequential(
        &self,
        wallet: &str,
        hours_back: u64,
    ) -> Result<Vec<UsdcTransfer>> {
        let cutoff_time = Utc::now() - Duration::hours(hours_back as i64);
        let mut all_transfers = Vec::new();
        let mut processed_signatures = HashSet::new();

        println!("🔍 Fetching transaction signatures for wallet...");

        // Get all signatures for the wallet
        let signatures = self.get_signatures_for_address(wallet).await?;

        println!("📝 Found {} recent signatures", signatures.len());

        // Filter signatures by time
        let recent_signatures: Vec<_> = signatures
            .into_iter()
            .filter(|sig| {
                if let Some(block_time) = sig.block_time {
                    let tx_time = chrono::Utc
                        .timestamp_opt(block_time, 0)
                        .single()
                        .unwrap_or(Utc::now());
                    tx_time >= cutoff_time
                } else {
                    true // Include transactions without block time
                }
            })
            .collect();

        println!(
            "⏰ {} signatures within {} hour window",
            recent_signatures.len(),
            hours_back
        );

        // Process transactions sequentially
        for (i, sig) in recent_signatures.iter().enumerate() {
            if processed_signatures.contains(&sig.signature) {
                continue;
            }

            processed_signatures.insert(sig.signature.clone());

            if i % 10 == 0 {
                println!(
                    "🔄 Processing transaction {}/{}",
                    i + 1,
                    recent_signatures.len()
                );
            }

            match self.get_transaction(&sig.signature).await {
                Ok(Some(transaction)) => {
                    match TransactionParser::parse_usdc_transfers(&transaction, wallet) {
                        Ok(mut transfers) => {
                            all_transfers.append(&mut transfers);
                        }
                        Err(e) => {
                            eprintln!("⚠️ Error parsing transaction {}: {}", sig.signature, e);
                        }
                    }
                }
                Ok(None) => {
                    // Transaction not found or null
                }
                Err(e) => {
                    eprintln!("⚠️ Error fetching transaction {}: {}", sig.signature, e);
                }
            }

            // Small delay between requests
            tokio::time::sleep(StdDuration::from_millis(50)).await;
        }

        // Sort transfers by timestamp (newest first)
        all_transfers.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        println!("✅ Found {} USDC transfers", all_transfers.len());

        Ok(all_transfers)
    }
}