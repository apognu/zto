extern crate serde;
extern crate serde_json;

use command::common;
use std::process;
use clap::{ArgMatches, SubCommand};
use util;
use util::HttpMethod::Get;
use reqwest::StatusCode;

#[derive(Debug, Serialize, Deserialize)]
struct ApiPeerPath {
  address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiPeer {
  address: String,
  role: String,
  latency: u32,
  version: String,
  paths: Vec<ApiPeerPath>,
}

pub struct Peer {}

impl common::Command for Peer {
  fn handle(&self, args: ArgMatches) {
    if let None = args.subcommand {
      eprintln!("ERROR: at least one command must be given");
      process::exit(1);
    }

    match args.subcommand.unwrap() {
      box SubCommand { name, .. } => {
        match name.as_ref() {
          "list" => self.list(),
          _ => {
            eprintln!("ERROR: unknown command `{}`", name);
            process::exit(1);
          }
        }
      }
    }
  }
}

impl Peer {
  fn list(&self) {
    if let Ok((status, json)) = util::request(Get, "/peer", None) {
      match status {
        StatusCode::Ok => {}
        StatusCode::NotFound => {
          println!("ERROR: {}, provided network does not exist", status);
          process::exit(1);
        }
        _ => {
          println!("ERROR: {}", status);
          process::exit(1);
        }
      }

      if let Ok(peers) = serde_json::from_str::<Vec<ApiPeer>>(&*json) {
        let mut data: Vec<Vec<String>> = vec![vec!["PEER ID".to_string(), "ADDRESS".to_string(), "ROLE".to_string(), "LATENCY".to_string(), "VERSION".to_string()]];

        for peer in peers {
          let mut version = String::from("-");
          let mut address = String::from("-");
          let (ztid, role, latency) = (
            peer.address,
            peer.role,
            peer.latency,
          );

          if let Some(path) = peer.paths.into_iter().nth(0) {
            address = path.address;
          }

          if role != "LEAF" {
            version = peer.version;
          }

          data.push(vec![ztid.clone(), address, role.to_string(), latency.to_string(), version.to_string()]);
        }

        return util::print_table(data);
      }
    }

    eprintln!("ERROR: could not retrieve data.");
    process::exit(1);
  }
}