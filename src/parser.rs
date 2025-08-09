use anyhow::{anyhow, Result};
use chrono::{DateTime, TimeZone, Utc};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::types::*;

pub struct TransactionParser;

impl TransactionParser {
    pub fn parse_usdc_transfers(
        tx: &TransactionResponse,
        target_wallet: &Pubkey,
    ) -> Result<Vec<UsdcTransfer>> {
        let mut transfers = Vec::new();
        
        // Skip failed transactions
        if let Some(meta) = &tx.meta {
            if meta.err.is_some() {
                return Ok(transfers);
            }
        } else {
            return Ok(transfers);
        }

        let timestamp = tx.block_time
            .map(|bt| Utc.timestamp_opt(bt, 0).single().unwrap_or(Utc::now()))
            .unwrap_or(Utc::now());

        let signature = tx.transaction.signatures.first()
            .ok_or_else(|| anyhow!("No signature found"))?
            .clone();

        // Parse token balance changes
        if let Some(meta) = &tx.meta {
            transfers.extend(Self::parse_token_balance_changes(
                meta,
                &tx.transaction.message.account_keys,
                target_wallet,
                &signature,
                timestamp,
            )?);
        }

        Ok(transfers)
    }

    fn parse_token_balance_changes(
        meta: &TransactionMeta,
        account_keys: &[String],
        target_wallet: &Pubkey,
        signature: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<Vec<UsdcTransfer>> {
        let mut transfers = Vec::new();

        let pre_balances = meta.pre_token_balances.as_ref().unwrap_or(&vec![]);
        let post_balances = meta.post_token_balances.as_ref().unwrap_or(&vec![]);

        // Find USDC token accounts
        let usdc_mint = Pubkey::from_str(USDC_MINT)?;

        for post_balance in post_balances {
            if post_balance.mint != USDC_MINT {
                continue;
            }

            // Find corresponding pre-balance
            let pre_balance = pre_balances.iter()
                .find(|pb| pb.account_index == post_balance.account_index);

            let pre_amount = pre_balance
                .and_then(|pb| pb.ui_token_amount.ui_amount)
                .unwrap_or(0.0);
            
            let post_amount = post_balance.ui_token_amount.ui_amount.unwrap_or(0.0);
            let amount_change = post_amount - pre_amount;

            if amount_change.abs() < 0.000001 {
                continue; // No significant change
            }

            // Get the owner of the token account
            let account_address = account_keys.get(post_balance.account_index as usize)
                .ok_or_else(|| anyhow!("Account index out of bounds"))?;

            // Try to determine the owner from the token balance or account keys
            let owner = post_balance.owner.as_ref()
                .or_else(|| {
                    // Fallback: try to find the owner in account keys
                    // This is a simplified approach
                    if post_balance.account_index > 0 {
                        account_keys.get((post_balance.account_index - 1) as usize)
                    } else {
                        None
                    }
                });

            if let Some(owner_str) = owner {
                let owner_pubkey = Pubkey::from_str(owner_str).ok();
                
                if let Some(owner_pubkey) = owner_pubkey {
                    if owner_pubkey == *target_wallet {
                        // This is our target wallet's token account
                        if amount_change > 0.0 {
                            // Received tokens - need to find sender
                            let from_address = Self::find_sender_address(
                                meta, account_keys, post_balance.account_index
                            ).unwrap_or_else(|| "Unknown".to_string());

                            transfers.push(UsdcTransfer {
                                signature: signature.to_string(),
                                timestamp,
                                from_address,
                                to_address: target_wallet.to_string(),
                                amount: amount_change,
                                is_incoming: true,
                            });
                        } else if amount_change < 0.0 {
                            // Sent tokens - need to find recipient
                            let to_address = Self::find_recipient_address(
                                meta, account_keys, post_balance.account_index
                            ).unwrap_or_else(|| "Unknown".to_string());

                            transfers.push(UsdcTransfer {
                                signature: signature.to_string(),
                                timestamp,
                                from_address: target_wallet.to_string(),
                                to_address,
                                amount: amount_change.abs(),
                                is_incoming: false,
                            });
                        }
                    }
                }
            }
        }

        Ok(transfers)
    }

    fn find_sender_address(
        meta: &TransactionMeta,
        account_keys: &[String],
        target_account_index: u8,
    ) -> Option<String> {
        // Look through pre-token balances to find who had a decrease
        let pre_balances = meta.pre_token_balances.as_ref()?;
        let post_balances = meta.post_token_balances.as_ref()?;

        for pre_balance in pre_balances {
            if pre_balance.mint != USDC_MINT || pre_balance.account_index == target_account_index {
                continue;
            }

            let post_balance = post_balances.iter()
                .find(|pb| pb.account_index == pre_balance.account_index);

            let pre_amount = pre_balance.ui_token_amount.ui_amount.unwrap_or(0.0);
            let post_amount = post_balance
                .and_then(|pb| pb.ui_token_amount.ui_amount)
                .unwrap_or(0.0);

            if pre_amount > post_amount {
                return pre_balance.owner.clone()
                    .or_else(|| account_keys.get(pre_balance.account_index as usize).cloned());
            }
        }

        None
    }

    fn find_recipient_address(
        meta: &TransactionMeta,
        account_keys: &[String],
        sender_account_index: u8,
    ) -> Option<String> {
        // Look through post-token balances to find who had an increase
        let pre_balances = meta.pre_token_balances.as_ref()?;
        let post_balances = meta.post_token_balances.as_ref()?;

        for post_balance in post_balances {
            if post_balance.mint != USDC_MINT || post_balance.account_index == sender_account_index {
                continue;
            }

            let pre_balance = pre_balances.iter()
                .find(|pb| pb.account_index == post_balance.account_index);

            let pre_amount = pre_balance
                .and_then(|pb| pb.ui_token_amount.ui_amount)
                .unwrap_or(0.0);
            let post_amount = post_balance.ui_token_amount.ui_amount.unwrap_or(0.0);

            if post_amount > pre_amount {
                return post_balance.owner.clone()
                    .or_else(|| account_keys.get(post_balance.account_index as usize).cloned());
            }
        }

        None
    }
}