use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

// USDC mint address on Solana mainnet
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsdcTransfer {
    pub signature: String,
    pub timestamp: DateTime<Utc>,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub is_incoming: bool,
}

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetSignaturesForAddressResponse {
    pub signature: String,
    pub slot: u64,
    pub err: Option<serde_json::Value>,
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,
    pub confirmation_status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionResponse {
    pub slot: u64,
    pub transaction: TransactionData,
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,
    pub meta: Option<TransactionMeta>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionData {
    pub message: TransactionMessage,
    pub signatures: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionMessage {
    #[serde(rename = "accountKeys")]
    pub account_keys: Vec<String>,
    pub instructions: Vec<TransactionInstruction>,
    #[serde(rename = "recentBlockhash")]
    pub recent_blockhash: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionInstruction {
    pub accounts: Vec<u8>,
    pub data: String,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "stackHeight")]
    pub stack_height: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionMeta {
    pub err: Option<serde_json::Value>,
    pub fee: u64,
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Option<Vec<InnerInstructions>>,
    #[serde(rename = "logMessages")]
    pub log_messages: Option<Vec<String>>,
    #[serde(rename = "postBalances")]
    pub post_balances: Vec<u64>,
    #[serde(rename = "postTokenBalances")]
    pub post_token_balances: Option<Vec<TokenBalance>>,
    #[serde(rename = "preBalances")]
    pub pre_balances: Vec<u64>,
    #[serde(rename = "preTokenBalances")]
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    pub status: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct InnerInstructions {
    pub index: u8,
    pub instructions: Vec<InnerInstruction>,
}

#[derive(Debug, Deserialize)]
pub struct InnerInstruction {
    pub accounts: Vec<u8>,
    pub data: String,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "stackHeight")]
    pub stack_height: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TokenBalance {
    #[serde(rename = "accountIndex")]
    pub account_index: u8,
    pub mint: String,
    #[serde(rename = "uiTokenAmount")]
    pub ui_token_amount: UiTokenAmount,
    pub owner: Option<String>,
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UiTokenAmount {
    pub amount: String,
    pub decimals: u8,
    #[serde(rename = "uiAmount")]
    pub ui_amount: Option<f64>,
    #[serde(rename = "uiAmountString")]
    pub ui_amount_string: String,
}

// SPL Token program ID
pub const SPL_TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
