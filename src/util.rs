use std::env;
use std::io::Read;
use reqwest::{self, StatusCode};
use prettytable::Table;
use prettytable::row::Row;
use prettytable::format::FormatBuilder;
use std::fs::File;

pub enum HttpMethod {
  Get,
  Post,
  Delete,
}

fn get_url(url: &str) -> String {
  let base_url = env::var("BASE_URL").unwrap_or("http://127.0.0.1:9993".to_string());
  let mut token = String::new();

  if let Ok(ref mut file) = File::open("/var/lib/zerotier-one/authtoken.secret") {
    file.read_to_string(&mut token).unwrap();
  } else {
    if let Ok(t) = env::var("TOKEN") {
      token = t
    }
  }

  format!("{}{}?auth={}", base_url, url, token)
}

fn get(url: String) -> reqwest::RequestBuilder {
  reqwest::Client::new().get(&*url)
}

fn post(url: String) -> reqwest::RequestBuilder {
  reqwest::Client::new().post(&*url)
}

fn delete(url: String) -> reqwest::RequestBuilder {
  reqwest::Client::new().delete(&*url)
}

pub fn request(method: HttpMethod, url: &str, body: Option<String>) -> Result<(StatusCode, String), Option<StatusCode>> {
  let mut request = match method {
    HttpMethod::Get => get(get_url(url)),
    HttpMethod::Post => post(get_url(url)),
    HttpMethod::Delete => delete(get_url(url)),
  };

  if let Some(body) = body {
    request.body(body);
  }

  if let Ok(mut response) = request.send() {
    let mut body = String::new();
    if let Ok(_) = response.read_to_string(&mut body) {
      return Ok((response.status(), body));
    }

    return Err(Some(response.status()));
  }
  
  Err(None)
}

pub fn print_table(rows: Vec<Vec<String>>) {
  let format = FormatBuilder::new().column_separator(' ').padding(0, 1).build();
  let mut table = Table::new();
  table.set_format(format);

  for row in rows.iter() {
    table.add_row(Row::from(row));
  }

  table.printstd();
}
