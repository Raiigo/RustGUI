#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
use std::{fs::read, io::Read};

use app::TestApp;
use async_std;
use surf::http::auth;
use std::net::TcpStream;
use std::iter::Map;
use serde::*;
use serde_json::Value;
use websocket::{Message, client::*, header};
use websocket::header::*;
use native_tls::{TlsConnector, TlsConnectorBuilder};
use std::thread;
use std::time::*;

#[derive(Deserialize, Serialize)]
pub struct SessionStartLimit {
    total: i32,
    remaining: i32,
    reset_after: i32,
    max_concurrency: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Gateway {
    url: String,
    shards: i32,
    session_start_limit: SessionStartLimit,
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[async_std::main]
async fn main() -> surf::Result<()> {
    // let app = app::TestApp::default();
    // eframe::run_native(Box::new(app));

    let mut gateway = surf::get("https://discord.com/api/v9/gateway/bot").header("Authorization", "Bot token_artifact").await?;
    let body_string = gateway.body_string().await?;
    println!("{}", body_string);
    let gateway_struct: Value = serde_json::from_str(&body_string)?;

    let mut url = String::new();
    url.push_str(&gateway_struct["url"].to_string());
    url = url.replace("\"", "");
    url.push_str("/?v=8&encoding=json");

    println!("{}", url);

    let mut headers = Headers::new();
    headers.set(Authorization("Bot token_artifact".to_owned()));
    headers.set(UserAgent("myBotThing (http://some.url, v0.1)".to_owned()));

    let mut client = ClientBuilder::new(&url)?
    .custom_headers(&headers)
    .connect_secure(Some(TlsConnector::new().unwrap()))?;

    let text_message = match client.recv_message()? {
        websocket::OwnedMessage::Text(text) => text,
        websocket::OwnedMessage::Binary(bin) => String::new(),
        websocket::OwnedMessage::Close(close) => String::new(),
        websocket::OwnedMessage::Ping(ping) => String::new(),
        websocket::OwnedMessage::Pong(pong) => String::new(),
    };

    dbg!(&text_message);

    let op_code_10: Value = serde_json::from_str(&text_message)?;
    println!("{}", op_code_10["d"]["heartbeat_interval"]);
    let hearthbeat_interval: i32 = op_code_10["d"]["heartbeat_interval"].to_string().parse::<i32>()?;

    let result_op_2 = client.send_message(&Message::text(r#"{
        "op": 2,
        "d": {
          "token": "token_artifact",
          "intents": 513,
          "properties": {
            "$os": "linux",
            "$browser": "rust_lib",
            "$device": "rust_lib"
          },
          "presence": {
            "activities": [{
              "name": "être un bot de qualité",
              "type": 0
            }],
            "status": "online",
            "since": 91879201,
            "afk": false
          }
        }
      }"#)).unwrap();

    let child = thread::spawn(move || -> surf::Result<()> {
        loop {
            println!("test");
            let result = &client.send_message(&Message::text("{\"op\": 1,\"d\": 251}")).unwrap(); // Heartbeat
            dbg!(&client.recv_message().unwrap());
            thread::sleep(Duration::from_millis(hearthbeat_interval.to_string().parse::<u64>().unwrap()));
            println!("caca");
        }
        Ok(())
    });

    println!("---------------------------------------------------------------");

    // let mut stream = TcpStream::connect(gateway_struct["url"].to_string())?;

    let response = surf::post("https://discord.com/api/v9/channels/713120571822571560/messages")
    .content_type(surf::http::mime::JSON)
    .header("Authorization", "Bot token_artifact")
    .header("User-Agent", "myBotThing (http://some.url, v0.1)")
    .body("{\"content\": \"Salut !\"}")
    .recv_string()
    .await?;

    println!("{}", response);

    let mut res = surf::get("https://discord.com/api/v9/users/@me").header("Authorization", "Bot token_artifact").await?;
    println!("{}", res.body_string().await?);
    return child.join().unwrap();
    
}