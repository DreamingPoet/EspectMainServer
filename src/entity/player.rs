use std::{sync::Arc, time::Duration};

use bytes::{BufMut, Bytes, BytesMut};
use futures::SinkExt;
use std::io;
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;

use crate::data_models::{CreateRoomResp, RPCDataLite, SetPlayerInfoReq};
use crate::Tx;
use crate::{
    data_models::{RPCData, RPCMessageType},
    FramedStream, FramedStreamSender, Rx,
};

use super::{DataManager, Peer};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Player {
    tx: Tx,
    account: String,
    data_manager: Arc<Mutex<DataManager>>,
}

impl Player {
    /// Create a new instance of `Peer`.
    pub async fn new(state: Arc<Mutex<DataManager>>, peer: &Peer, tx: Tx) -> io::Result<Player> {
        // Get the client socket address
        let addr = peer.stream.get_ref().peer_addr()?;

        let player = Player {
            tx: tx,
            account: "".to_string(),
            data_manager: state.clone(),
        };

        state.lock().await.players.insert(addr, player.clone());
        Ok(player)
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
                RPCMessageType::Login => {}
                RPCMessageType::CreateRoom => {
                    tokio::spawn(Self::handle_create_room(
                        self.data_manager.clone(),
                        self.account.clone(),
                        rpc_data.into(),
                        self.tx.clone(),
                    ));
                }
                RPCMessageType::SetPlayerInfo => {
                    let data: SetPlayerInfoReq = serde_json::from_slice(&rpc_data.Data)?;
                    self.account = data.account;
                }
                _ => {}
            }
        }

        Ok(())
    }

    // async fn handle_login(rpc_data: &RPCData) -> Result<(), Box<dyn Error>> {
    //     if rpc_data.MsgType == RPCMessageType::Login {
    //         loop {
    //             sleep(Duration::from_millis(1000)).await;
    //             println!("handle_login ... ... ")
    //         }
    //     }

    //     Ok(())
    // }

    async fn handle_create_room(
        data_manager: Arc<Mutex<DataManager>>,
        account: String,
        rpc_data: RPCDataLite,
        tx: Tx,
    ) {
        if rpc_data.MsgType == RPCMessageType::CreateRoom {
            let mut a = 0;
            let mut room_host = "".to_string();
            loop {
                sleep(Duration::from_millis(500)).await;
                println!("handle_create_room ... ... ");

                let mut state = data_manager.lock().await;
                room_host = state.find_server_by_creater_account(&account);

                a += 1;
                if a > 50 || room_host != "".to_string() {
                    // 1 准备返回的数据
                    let data = CreateRoomResp {
                        RoomHost: room_host,
                    };

                    if let Ok(data_str) = serde_json::to_vec(&data) {
                        // 2 开始组装数据
                        let mut buf = BytesMut::new();
                        buf.put_u16(rpc_data.MagicNum);
                        buf.put_u32(rpc_data.ReqID);
                        buf.put_u16(rpc_data.MsgType.to_u16());
                        buf.put_slice(&data_str);
                        println!("send data{:?}, len = {}", &buf, &buf.len());
                        tx.send(buf).unwrap();
                    }
                    return;
                }
            }
        }
    }
}
