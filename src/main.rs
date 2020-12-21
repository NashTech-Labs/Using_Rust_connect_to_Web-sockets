use std::env;

use futures::sink::SinkExt;
use futures::stream::StreamExt;
use websocket_lite::{Message, Opcode, Result};
use serde_json::Value;
use substrate_subxt::{ClientBuilder, PairSigner, NodeTemplateRuntime, Client};
use sp_keyring::AccountKeyring;
use substrate_subxt::generic_asset::{CreateCall, AssetOptions, PermissionsV1, Owner};
use substrate_subxt::polkadex::{RegisterNewOrderbookCall, OrderType, SubmitOrder};
use substrate_subxt::sp_runtime::testing::H256;
use substrate_subxt::sp_runtime::sp_std::str::FromStr;

async fn run() -> Result<()> {
    let url = env::args().nth(1).unwrap_or_else(|| "wss://echo.websocket.org".to_owned());
    let builder = websocket_lite::ClientBuilder::new(&url)?;
    let mut ws_stream = builder.async_connect().await?;
    ws_stream.send(Message::close(None)).await;

    loop {
        let msg: Option<Result<Message>> = ws_stream.next().await;

        let msg = if let Some(msg) = msg {
            msg
        } else {
            break;
        };

        let msg = if let Ok(msg) = msg {
            msg
        } else {
            let _ = ws_stream.send(Message::close(None)).await;
            break;
        };

        match msg.opcode() {
            Opcode::Text => {
                println!("{}", msg.as_text().unwrap());
                ws_stream.send(msg).await?
            }
            Opcode::Binary => ws_stream.send(msg).await?,
            Opcode::Ping => ws_stream.send(Message::pong(msg.into_data())).await?,
            Opcode::Close => {
                let _ = ws_stream.send(Message::close(None)).await;
                break;
            }
            Opcode::Pong => {}
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() {
    tokio::spawn(async {
        run().await.unwrap_or_else(|e| {
            eprintln!("{}", e);
        })
    })
        .await
        .unwrap();
}


