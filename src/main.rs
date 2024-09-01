use anyhow::Context;
use bittorrent_starter_rust::torrent::{Keys, Torrent};
use clap::{Parser, Subcommand};
// Available if you need it!
use serde_bencode;
use std::path::PathBuf;
use bittorrent_starter_rust::tracker::{TrackerRequest, TrackerResponse};

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Decode {
        value: String,
    },
    Info {
        torrent: PathBuf,
    },
    Peers {
        torrent: PathBuf,
    },
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.commands {
        Commands::Decode { value: _value } => {
            unimplemented!("serde_bencode -> serde_json::Value is borked")
        }
        Commands::Info { torrent } => {
            let dot_torrent = std::fs::read(torrent).context("read torrent file")?;
            let t: Torrent =
                serde_bencode::from_bytes(&dot_torrent).context("parse torrent file")?;
            println!("Tracker URL: {}", t.announce);
            let length = if let Keys::SingleFile { length } = t.info.keys {
                length
            } else {
                todo!();
            };
            println!("Length: {length}");
            let info_hash = t.info_hash();
            println!("Info Hash: {}", hex::encode(&info_hash));
            println!("Piece Length: {}", t.info.plength);
            println!("Piece Hashes:");
            for hash in t.info.pieces.0 {
                println!("{}", hex::encode(&hash));
            }
        }
        Commands::Peers { torrent } => {
            let dot_torrent = std::fs::read(torrent).context("read torrent file")?;
            let t: Torrent =
                serde_bencode::from_bytes(&dot_torrent).context("parse torrent file")?;
            let length = if let Keys::SingleFile { length } = t.info.keys {
                length
            } else {
                todo!();
            };
            let info_hash = t.info_hash();
            let request = TrackerRequest {
                peer_id: String::from("00112233445566778899"),
                port: 6881,
                uploaded: 0,
                downloaded: 0,
                left: length,
                compact: 1,
            };
            let url_params =
                serde_urlencoded::to_string(&request).context("url-encode tracker parameters")?;
            let tracker_url = format!(
                "{}?{}&info_hash={}",
                t.announce,
                url_params,
                &urlencode(&info_hash)
            );
            let response = reqwest::get(tracker_url).await.context("query tracker")?;
            let response = response.bytes().await.context("fetch tracker response")?;

            let response:TrackerResponse = serde_bencode::from_bytes(&response).context("parse tracker response")?;
            for peer in &response.peers.0{
                println!("{}:{}", peer.ip(), peer.port());
            }
        }
    }
    Ok(())
}

fn urlencode(t: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(3 * t.len());
    for &byte in t {
        encoded.push('%');
        encoded.push_str(&hex::encode(&[byte]));
    }
    encoded
}

