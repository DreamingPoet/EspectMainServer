use bytes::BytesMut;
use std::{sync::Arc, time::Duration, io, net::SocketAddr};
use crate::{EspectError, data_models::{RPCMessageType, RPCData, SetUEServerInfoReq}, FramedStream, Rx, Tx};

use super::{Player, Peer, DataManager};
use tokio::sync::{mpsc, Mutex};

#[derive(Debug, Clone)]
pub struct UEServer {
    pub tx: Tx,
    pub addr: SocketAddr,
    pub data_manager : Arc<Mutex<DataManager>>,

    // 服务器房间端口
    pub room_port: i32,

    // 课件名称
    pub lesson_name: String,

    // 房间创建者账号
    pub creater_account: String,

    // creater: Player,
    
}

impl UEServer {
    /// Create a new instance of `Peer`.
    pub async fn new(state: Arc<Mutex<DataManager>>, peer: & Peer, tx:Tx) -> io::Result<UEServer> {
        // Get the client socket address
        let addr = peer.stream.get_ref().peer_addr()?;
        let ue_server = UEServer {
            tx:tx,
            addr:addr,
            data_manager:state.clone(),
            room_port: 0,
            lesson_name: "".to_string(),
            creater_account: "".to_string(),
            // creater: ,
            
        };

        state.lock().await.ue_servers.insert(addr, ue_server.clone());
        Ok(ue_server)
    }



pub async fn handle_ue_server(&mut self, buf: &mut BytesMut)  -> Result<(), EspectError> {
    println!("handle_ue_server: msg:len = {}, content = {:?}", buf.len(), buf);
    if let Some(rpc_data) = RPCData::from(buf) {
        match rpc_data.MsgType {
            RPCMessageType::HeartBeat => {},
            RPCMessageType::SetUEServerInfo => {
                let data: SetUEServerInfoReq = serde_json::from_slice(&rpc_data.Data)?;
                self.creater_account = data.creater_account;
                self.room_port = data.room_port;

                // 更新数据
                let mut data_manager = self.data_manager.lock().await;
                data_manager.ue_servers.insert(self.addr, self.clone());


            },
            _ => todo!(),
        }
    }

    Ok(())
}

}