use bytes::BytesMut;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::{mpsc::Receiver, oneshot};

use crate::{
    data_models::{RPCData, RPCMessageType, SetConnectionTypeReq},
    FramedStreamSender, EspectError,
};

use super::{Player, UEServer};

struct DataManager {}

// 连接上来的客户端的类型
#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionType {
    UEServer,
    Player,
    Unknown,
}

impl ConnectionType {
    pub fn to_u8(&self) -> u8 {
        match self {
            ConnectionType::UEServer => return 0x0,
            ConnectionType::Player => return 0x1,
            ConnectionType::Unknown => return 0x2,
        };
    }
}

impl From<u8> for ConnectionType {
    fn from(orig: u8) -> ConnectionType {
        match orig {
            0x0 => return ConnectionType::UEServer,
            0x1 => return ConnectionType::Player,
            _ => return ConnectionType::Unknown,
        };
    }
}

//  对共享数据（ClientMap）的所有的操作
pub enum DataOperation {
    // 添加一个UEServer
    add_ue_server {
        key: usize,
        // client: SplitSink<WebSocket, Message>,
    },

    // 添加一个UEServer
    add_player {
        key: usize,
        // client: SplitSink<WebSocket, Message>,
    },
    // 检查连接是什么类型
    check_type {
        // 传参
        stream: FramedStreamSender,
        // 返回值类型
        resp: Responder<Option<ConnectionType>>,
    },
}


// 对客户端操作的响应
type Responder<T> = oneshot::Sender<T>;

// 处理数据
pub async fn handle_data_channel(mut rx: Receiver<DataOperation>) -> Option<()> {
    // 数据
    println!("into data handle");
    let ue_servers: HashMap<String, UEServer>;
    let players: HashMap<String, Player>;

    loop {}
}

pub async fn handle_unknow(conn_type: &mut ConnectionType, buf: &mut BytesMut) -> Result<(), EspectError>{
    println!("Receive a msg:len = {}, content = {:?}", buf.len(), buf);
    if let Some(rpc_data) = RPCData::from(buf) {
        match rpc_data.MsgType {
            RPCMessageType::SetConnectionType => {
                let a = SetConnectionTypeReq{ connType: ConnectionType::UEServer };
                let data_str  = serde_json::to_string(&a)?;

                println!("data_str.connectionType = {:?}", data_str);

                let data_str: SetConnectionTypeReq  = serde_json::from_slice(&rpc_data.Data)?;

                println!("data_str.connectionType = {:?}", data_str.connType);

            },
            _ => {},
        };
    }

    Ok(())
}
