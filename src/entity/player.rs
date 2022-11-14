use std::{sync::Arc, time::Duration};

use bytes::BytesMut;
use futures::SinkExt;
use std::io;
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;

use crate::{
    data_models::{RPCData, RPCMessageType},
    FramedStream, FramedStreamSender, Rx,
};

use super::{DataManager, Peer};
use std::error::Error;

pub struct Player {
    pub peer: Peer,
    account: String,
}

impl Player {
    /// Create a new instance of `Peer`.
    pub async fn new(state: Arc<Mutex<DataManager>>, peer: Peer) -> io::Result<Player> {
        // Get the client socket address
        let addr = peer.stream.get_ref().peer_addr()?;

        // Add an entry for this `Peer` in the shared state map.
        state.lock().await.players.insert(addr, peer.tx.clone());

        Ok(Player {
            peer,
            account: "".to_string(),
        })
    }


pub async fn handle_player(&mut self, buf: &mut BytesMut) -> Result<(), Box<dyn Error>> {
    println!(
        "handle_player: msg:len = {}, content = {:?}",
        buf.len(),
        buf
    );

    // peer.stream.send(item);

    if let Some(rpc_data) = RPCData::from(buf) {
        match rpc_data.MsgType {
            RPCMessageType::HeartBeat => {}
            RPCMessageType::Login => {
                let res = tokio::spawn(async move{
                    let mut a = 0;
                    loop {
                        sleep(Duration::from_millis(1000)).await;
                        println!("handle_login ... ... {}", a);
                        a += 1;
                        if a == 10 {
                            return a;
                        }
                    }
                });

     
            }
            RPCMessageType::CreateRoom => {
                // let _ = handle_create_room(&rpc_data, &mut stream).await;
                tokio::spawn(async move{

                        println!("handle_create_room ... ... ")

                });
            }
            RPCMessageType::SetPlayerInfo => {
                // let _ = handle_create_room(&rpc_data, &mut stream).await;
            }
            _ => {}
        }

        // tokio::select! {
        //     _ = handle_create_room(&rpc_data) => {
        //         println!("handle_create_room() completed first")
        //     }

        //     _ = handle_login(&rpc_data) => {
        //         println!("handle_login() completed first")
        //     }
        // }
    }

    //     tokio::join!(create_room, login);

    //     // match rpc_data.MsgType {
    //     //     RPCMessageType::HeartBeat => {},
    //     //     RPCMessageType::Login => {},
    //     //     RPCMessageType::CreateRoom => {
    //     //         // let _ = handle_create_room(&rpc_data, &mut stream).await;
    //     //     },
    //     //     RPCMessageType::SetPlayerInfo => {
    //     //         // let _ = handle_create_room(&rpc_data, &mut stream).await;
    //     //     },
    //     //     _ => todo!(),
    //     // }
    // }

    Ok(())
}

async fn handle_login(rpc_data: &RPCData) -> Result<(), Box<dyn Error>> {
    if rpc_data.MsgType == RPCMessageType::Login {
        loop {
            sleep(Duration::from_millis(1000)).await;
            println!("handle_login ... ... ")
        }
    }

    Ok(())
}

async fn handle_create_room(rpc_data: &RPCData) -> Result<(), Box<dyn Error>> {
    if rpc_data.MsgType == RPCMessageType::CreateRoom {
        loop {
            sleep(Duration::from_millis(1000)).await;
            println!("handle_create_room ... ... ")
        }
    }

    // // 1 准备返回的数据
    // let data = CreateRoomResp {
    //     RoomHost: "127.0.0.1:7777".to_string(),
    // };

    // let data_str = serde_json::to_vec(&data)?;

    // // 2 开始组装数据
    // let mut buf = BytesMut::new();
    // buf.put_u16(rpc_data.MagicNum);
    // buf.put_u32(rpc_data.ReqID);
    // buf.put_u16(rpc_data.MsgType.to_u16());
    // buf.put_slice(&data_str);

    // println!("send data{:?}, len = {}", &buf, &buf.len());

    // stream.send(Bytes::from(buf)).await.unwrap();

    Ok(())
}

}