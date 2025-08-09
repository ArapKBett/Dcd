# Solana USDC Indexer

A Rust application that indexes USDC transfers for a specific Solana wallet address. This indexer backfills the last 24 hours of USDC transactions and provides detailed transfer information.

## Features

- 🔍 **Wallet-Specific Indexing**: Index all USDC transfers for a specific wallet
- ⏰ **Configurable Time Range**: Backfill transfers for the last N hours (default: 24)
- 📊 **Detailed Transfer Info**: Shows amount, direction, timestamp, and counterparty addresses
- 🎯 **USDC Focus**: Specifically designed for USDC (EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v) transfers
- 📝 **Multiple Output Formats**: Pretty-printed summary or JSON output
- 🚀 **Production Ready**: Optimized for deployment on Render.com

## Usage

### Command Line Options

```bash
# Index specific wallet with default settings (24 hours)
./indexer --wallet=7cMEhpt9y3inBNVv8fNnuaEbx7hKHZnLvR1KWKKxuDDU

# Index with custom time range
./indexer --wallet=7cMEhpt9y3inBNVv8fNnuaEbx7hKHZnLvR1KWKKxuDDU --hours=48

# Output as JSON
./indexer --wallet=7cMEhpt9y3inBNVv8fNnuaEbx7hKHZnLvR1KWKKxuDDU --output=json

# Show help
./indexer --help
```

### Example Output

```
🚀 Starting Solana USDC Indexer
📍 Wallet: 7cMEhpt9y3inBNVv8fNnuaEbx7hKHZnLvR1KWKKxuDDU
⏰ Backfilling last 24 hours

📊 USDC Transfer Summary
═══════════════════════════════════════════════════════════════
Found 5 USDC transfers

🕒 2024-01-15 14:30:25 UTC | 📥 RECEIVED | $100.000000 USDC
   💳 Transaction: 5KJp...abc123
   📨 From: 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM

🕒 2024-01-15 12:15:42 UTC | 📤 SENT | $50.500000 USDC
   💳 Transaction: 3Mrt...def456
   📤 To: 4quzHbvGHBEeM4dMAyb4tKjFHNhNdKhgMKTdwTrUmFFr

═══════════════════════════════════════════════════════════════
💰 Total Received: $150.000000 USDC
💸 Total Sent: $75.500000 USDC
📈 Net Change: $74.500000 USDC
═══════════════════════════════════════════════════════════════
```

## Deployment on Render.com

### Option 1: Web Service (Always Running)
1. Connect your GitHub repository to Render
2. Use the `render.yaml` configuration (web service section)
3. The service will run continuously and can be triggered via HTTP

### Option 2: Cron Job (Scheduled)
1. Use the cron job configuration in `render.yaml`
2. Runs automatically every hour
3. More cost-effective for periodic indexing

### Environment Variables

The application uses these environment variables (optional):
- `RUST_LOG`: Set to `info` for detailed logging
- `PORT`: Port for web service (default: 8080)

## Local Development

### Prerequisites
- Rust 1.75 or later
- OpenSSL development libraries

### Setup

```bash
# Clone the repository
git clone <your-repo>
cd solana-usdc-indexer

# Install dependencies
cargo build

# Run the indexer
cargo run -- --wallet=7cMEhpt9y3inBNVv8fNnuaEbx7hKHZnLvR1KWKKxuDDU

# Run with custom parameters
cargo run -- --wallet=YOUR_WALLET_ADDRESS --hours=12 --output=json
```

### Testing with Different Wallets

The indexer works with any Solana wallet address. Some example wallets you can test with:
- `7cMEhpt9y3inBNVv8fNnuaEbx7hKHZnLvR1KWKKxuDDU` (provided example)
- Replace with any valid Solana wallet address

## Architecture

### Components

1. **Main Application** (`src/main.rs`)
   - CLI argument parsing
   - Output formatting
   - Orchestrates the indexing process

2. **Indexer** (`src/indexer.rs`)
   - Connects to Solana RPC
   - Fetches transaction signatures for a wallet
   - Retrieves full transaction details
   - Filters by time range

3. **Parser** (`src/parser.rs`)
   - Analyzes transaction data
   - Identifies USDC transfers
   - Determines transfer direction and amounts
   - Extracts counterparty addresses

4. **Types** (`src/types.rs`)
   - Data structures for Solana RPC responses
   - Transfer data models
   - Constants (USDC mint address, etc.)

### Technical Details

- **RPC Endpoint**: Uses Solana mainnet-beta RPC
- **USDC Mint**: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
- **Rate Limiting**: Includes delays between requests to be respectful to RPC
- **Error Handling**: Robust error handling with detailed logging
- **Batch Processing**: Processes transactions in configurable batches

## Performance Considerations

- **RPC Limits**: Uses public Solana RPC with rate limiting
- **Memory Usage**: Processes transactions in batches to manage memory
- **Network Efficiency**: Filters transactions by time before detailed parsing
- **Caching**: Avoids processing duplicate transactions

## Limitations

- **RPC Dependency**: Relies on Solana RPC availability
- **Rate Limits**: Subject to public RPC rate limits
- **Historical Data**: Limited by RPC historical data retention
- **Token Account Detection**: Uses heuristics for owner detection

## Troubleshooting

### Common Issues

1. **RPC Timeouts**: Increase timeout or use different RPC endpoint
2. **Rate Limiting**: Add delays between requests or use paid RPC
3. **Memory Issues**: Reduce batch size or time range
4. **Missing Transfers**: Check if wallet has associated token accounts

### Debugging

Enable verbose logging:
```bash
RUST_LOG=debug ./indexer --wallet=YOUR_WALLET
```

## Future Enhancements

- [ ] Support for other SPL tokens
- [ ] Real-time indexing with WebSocket subscriptions  
- [ ] Database storage for historical data
- [ ] Web API interface
- [ ] GraphQL endpoint
- [ ] Multiple wallet support
- [ ] Transaction categorization and tagging

## License

MIT License - see LICENSE file for details.
