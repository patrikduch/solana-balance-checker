use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer, // Adding Signer trait
};
use std::str::FromStr;
use std::env;
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
    
    // Option 1: Get wallet from private key if provided
    if let Ok(private_key) = env::var("WALLET_PRIVATE_KEY") {
        let wallet_bytes = bs58::decode(private_key).into_vec()?;
        let wallet = Keypair::from_bytes(&wallet_bytes)?;
        let wallet_pubkey = wallet.pubkey(); // Using pubkey() method from Signer trait
        
        // Get balance
        let balance = rpc_client.get_balance(&wallet_pubkey)?;
        let sol_balance = balance as f64 / 1_000_000_000.0; // Convert lamports to SOL
        
        println!("Connected to Solana network: {}", rpc_url);
        println!("Wallet address: {}", wallet_pubkey);
        println!("SOL balance: {} SOL ({} lamports)", sol_balance, balance);
        
        return Ok(());
    }
    
    // Option 2: Use a public key if provided
    if let Ok(public_key) = env::var("WALLET_PUBLIC_KEY") {
        let wallet_pubkey = Pubkey::from_str(&public_key)?;
        
        // Get balance
        let balance = rpc_client.get_balance(&wallet_pubkey)?;
        let sol_balance = balance as f64 / 1_000_000_000.0; // Convert lamports to SOL
        
        println!("Connected to Solana network: {}", rpc_url);
        println!("Wallet address: {}", wallet_pubkey);
        println!("SOL balance: {} SOL ({} lamports)", sol_balance, balance);
        
        return Ok(());
    }
    
    // If no wallet info provided
    return Err(anyhow!("No wallet information provided. Please set either WALLET_PRIVATE_KEY or WALLET_PUBLIC_KEY in your environment or .env file."));
}