use bytes::BytesMut;
use std::{sync::Arc, time::Duration, io};
use crate::{EspectError, data_models::{RPCMessageType, RPCData}, FramedStream, Rx};

use super::{Player, Peer, DataManager};
use tokio::sync::{mpsc, Mutex};
pub struct UEServer {
    pub peer: Peer,

    // 服务器房间端口
    room_host: String,

    // 课件ID
    lesson_id: String,

    // 房间创建者ID
    creater_id: String,

    // creater: Player,
    
}

impl UEServer {
    /// Create a new instance of `Peer`.
    pub async fn new(state: Arc<Mutex<DataManager>>, peer: Peer) -> io::Result<UEServer> {
        // Get the client socket address
        let addr = peer.stream.get_ref().peer_addr()?;

        // Add an entry for this `Peer` in the shared state map.
        state.lock().await.ue_servers.insert(addr, peer.tx.clone());

        Ok(UEServer {
            peer,
            room_host: "".to_string(),
            lesson_id: "".to_string(),
            creater_id: "".to_string(),
            // creater: ,
            
        })
    }



pub async fn handle_ue_server(&mut self, buf: &mut BytesMut)  -> Result<(), EspectError> {
    println!("handle_ue_server: msg:len = {}, content = {:?}", buf.len(), buf);
    if let Some(rpc_data) = RPCData::from(buf) {
        match rpc_data.MsgType {
            RPCMessageType::HeartBeat => {},
            RPCMessageType::Login => {},
            RPCMessageType::CreateRoom => {
                // let _ = handle_create_room(&rpc_data, &mut stream).await;
            },
            RPCMessageType::SetUEServerInfo => {
                // let _ = handle_create_room(&rpc_data, &mut stream).await;
            },
            _ => todo!(),
        }
    }

    Ok(())
}

}