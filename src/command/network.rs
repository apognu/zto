extern crate serde;
extern crate serde_json;

use std::process;
use command::common;
use util;
use util::HttpMethod::{Get, Post, Delete};
use clap::{ArgMatches, SubCommand};
use reqwest::StatusCode;

#[derive(Debug, Serialize, Deserialize)]
struct ApiNetwork {
  nwid: String,
  name: String,
  #[serde(rename = "type")] kind: String,
  status: String,
  mac: String,
  #[serde(rename = "portDeviceName")] device: String,
  #[serde(rename = "assignedAddresses")] addresses: Vec<String>,
}

pub struct Network {}

impl common::Command for Network {
  fn handle(&self, args: ArgMatches) {
    if let None = args.subcommand {
      eprintln!("ERROR: at least one command must be given");
      process::exit(1);
    }

    match args.subcommand.unwrap() {
      box SubCommand { name, matches } => {
        match name.as_ref() {
          "list" => self.list(),
          "join" => {
            let network_id = matches.value_of("NETWORK_ID").unwrap();
            self.join(network_id.to_string());
          }
          "leave" => {
            let network_id = matches.value_of("NETWORK_ID").unwrap();
            self.leave(network_id.to_string());
          }
          _ => {
            eprintln!("ERROR: unknown command `{}`", name);
            process::exit(1);
          }
        }
      }
    }
  }
}

impl Network {
  fn list(&self) {
    if let Ok((status, json)) = util::request(Get, "/network", None) {
      if status != StatusCode::Ok {
        println!("ERROR: {}", status);
        process::exit(1);
      }

      if let Ok(networks) = serde_json::from_str::<Vec<ApiNetwork>>(&*json) {
        let mut data: Vec<Vec<String>> = vec![vec!["NETWORK ID".to_string(), "NAME".to_string(), "TYPE".to_string(), "STATUS".to_string(), "MAC".to_string(), "DEVICE".to_string(), "ADDRESSES".to_string()]];

        for network in networks {
          let addresses = network.addresses.join("\n");
          let (nwid, name, kind, status, mac, device) = (
            network.nwid,
            network.name,
            network.kind,
            network.status,
            network.mac,
            network.device,
          );

          data.push(vec![nwid, name, kind, status, mac, device, addresses]);
        }

        return util::print_table(data);
      }
    }

    eprintln!("ERROR: could not retrieve data.");
    process::exit(1);
  }

  fn join(&self, network_id: String) {
    if let Ok((status, _)) = util::request(Post, format!("/network/{}", network_id).as_ref(), None) {
      if status != StatusCode::Ok {
        println!("ERROR: {}", status);
        process::exit(1);
      }

      return println!("OK: joined network `{}`", network_id);
    }

    eprintln!("ERROR: could not retrieve data.");
    process::exit(1);
  }

  fn leave(&self, network_id: String) {
    if let Ok((status, _)) = util::request(Delete, format!("/network/{}", network_id).as_ref(), None) {
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

      return println!("OK: left network `{}`", network_id);
    }

    eprintln!("ERROR: could not retrieve data.");
    process::exit(1);
  }
}
