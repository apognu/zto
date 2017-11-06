extern crate serde;
extern crate serde_json;

use command::common;
use std::process;
use clap::{ArgMatches, SubCommand};
use util;
use util::HttpMethod::{Get, Post};
use reqwest::StatusCode;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ApiMember {
  authorized: Option<bool>,
}

pub struct Controller {}

impl common::Command for Controller {
  fn handle(&self, args: ArgMatches) {
    if let None = args.subcommand {
      eprintln!("ERROR: at least one command must be given");
      process::exit(1);
    }

    let network_id = args.value_of("NETWORK_ID").unwrap().to_string();

    match args.subcommand.unwrap() {
      box SubCommand { name, matches } => {
        match name.as_ref() {
          "members" => self.members(network_id),
          "authorize" => self.set_authorization(network_id, matches, true),
          "deauthorize" => self.set_authorization(network_id, matches, false),
          _ => {
            eprintln!("ERROR: unknown command `{}`", name);
            process::exit(1);
          }
        }
      }
    }
  }
}

impl Controller {
  fn members(&self, network_id: String) {
    if let Ok((status, json)) = util::request(Get, format!("/controller/network/{}/member", network_id).as_ref(), None) {
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

      if let Ok(nodes) = serde_json::from_str::<HashMap<String, u32>>(&*json) {
        let mut data: Vec<Vec<String>> = vec![vec!["NETWORK ID".to_string(), "NODE ID".to_string(), "REVISION".to_string()]];

        for (node, revision) in nodes {
          data.push(vec![network_id.clone(), node.to_string(), revision.to_string()]);
        }

        return util::print_table(data);
      }
    }

    eprintln!("ERROR: could not retrieve data.");
    process::exit(1);
  }

  fn set_authorization(&self, network_id: String, args: ArgMatches, authorized: bool) {
    let node_id = args.value_of("NODE_ID").unwrap();
    let node = ApiMember{authorized: Some(authorized)};
    
    if let Ok(node) = serde_json::to_string(&node) {
      if let Ok((status, _)) = util::request(Post, format!("/controller/network/{}/member/{}", network_id, node_id).as_ref(), Some(node)) {
        if status != StatusCode::Ok {
          println!("ERROR: {}", status);
          process::exit(1);
        }

        return println!("OK: node `{}` authorization set on network `{}`", node_id, network_id);
      }
    }

    eprintln!("ERROR: could not retrieve data.");
    process::exit(1);
  }
}