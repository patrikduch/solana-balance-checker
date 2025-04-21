use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::Keypair,
    signer::Signer,
};
use std::env;
use std::process::Command;
use anyhow::{anyhow, Result};
use dotenv::dotenv;

fn main() -> Result<()> {
    // Load environment variables from .env file if present
    dotenv().ok();
    
    // Get Solana RPC URL
    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    
    // Connect to Solana network
    let rpc_client = RpcClient::new_with_commitment(
        rpc_url.clone(),
        CommitmentConfig::confirmed(),
    );
    
    // Check for wallet private key
    let private_key = match env::var("WALLET_PRIVATE_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Err(anyhow!("WALLET_PRIVATE_KEY not found in environment variables"));
        }
    };
    
    // Create wallet from private key
    let wallet_bytes = bs58::decode(private_key).into_vec()?;
    let wallet = Keypair::from_bytes(&wallet_bytes)?;
    let wallet_pubkey = wallet.pubkey();
    
    // Get SOL balance in lamports
    let balance_lamports = rpc_client.get_balance(&wallet_pubkey)?;
    let sol_balance = balance_lamports as f64 / 1_000_000_000.0; // Convert lamports to SOL
    
    println!("Connected to Solana network: {}", rpc_url);
    println!("Wallet address: {}", wallet_pubkey);
    println!("SOL balance: {} SOL ({} lamports)", sol_balance, balance_lamports);
    
    // Get real SOL price by using a command line curl request
    println!("Fetching current SOL price...");
    let sol_price = get_sol_price()?;
    
    // Calculate USD value
    let usd_value = sol_balance * sol_price;
    println!("Current SOL price: ${:.2} USD", sol_price);
    println!("Wallet value: ${:.2} USD", usd_value);
            
    // Calculate how much SOL is $1 worth
    let sol_for_one_dollar = 1.0 / sol_price;
    println!("\n$1 USD = {:.8} SOL", sol_for_one_dollar);
    println!("$1 USD = {} lamports", (sol_for_one_dollar * 1_000_000_000.0) as u64);
    
    Ok(())
}

// Get real-time SOL price in USD using curl command
fn get_sol_price() -> Result<f64> {
    // Use curl to get the price from CoinGecko API
    let output = Command::new("curl")
        .args(&["https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd"])
        .output()?;
    
    if !output.status.success() {
        return Err(anyhow!("Failed to execute curl command"));
    }
    
    // Parse the JSON response
    let response = String::from_utf8(output.stdout)?;
    
    // Extract the price using string manipulation (basic JSON parsing)
    // The response is in the format: {"solana":{"usd":X.XX}}
    let price_start = response.find("\"usd\":").ok_or_else(|| anyhow!("Could not find price in response"))?;
    let price_substr = &response[price_start + 6..];
    let price_end = price_substr.find("}").ok_or_else(|| anyhow!("Could not find end of price in response"))?;
    let price_str = &price_substr[..price_end];
    
    // Convert to f64
    let price = price_str.parse::<f64>()?;
    Ok(price)
}