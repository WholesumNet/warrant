use risc0_zkvm::Receipt;
use risc0_zkvm::sha::Digest;
use serde_json;
use std::fs;
use hex::FromHex;
use clap::Parser;

// CLI
#[derive(Parser, Debug)]
#[command(name = "Verifier CLI for Wholesum: p2p verifiable computing marketplace.")]
#[command(author = "Wholesum team")]
#[command(version = "0.1")]
#[command(about = "Yet another verifiable compute marketplace.", long_about = None)]
struct Cli {
    #[arg(short, long)]
    image_id: String,

    #[arg(short, long)]
    receipt_file: String,
}

fn main() {
    let cli = Cli::parse();
    
    // factors image_id: af1b4fd024acd5f8756263d3c73e66d816a86ca285bb4addc2a3ee8a14bb87c2
    let image_id = Digest::from_hex(cli.image_id).unwrap();

    let bytes = fs::read(cli.receipt_file).unwrap();
    let s = String::from_utf8_lossy(&bytes);
    let receipt: Receipt = serde_json::from_str(&s).unwrap();       
    
    receipt.verify(image_id)
        .expect("verification failed.");
    println!("successfully verified!");
}