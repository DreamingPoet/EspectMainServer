use bytes::{BytesMut, Bytes, BufMut};
use futures::SinkExt;
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
        // stream: FramedStreamSender,
        // 返回值类型
        // resp: Responder<Option<ConnectionType>>,
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

pub async fn handle_unknow(conn_type: &mut ConnectionType, buf: &mut BytesMut, sender: &mut FramedStreamSender<'_> ) -> Result<(), EspectError>{
    println!("handle_unknow: msg:len = {}, content = {:?}", buf.len(), buf);
    if let Some(rpc_data) = RPCData::from(buf) {
        match rpc_data.MsgType {
            RPCMessageType::SetConnectionType => {
                let data_str: SetConnectionTypeReq  = serde_json::from_slice(&rpc_data.Data)?;
                println!("data_str.connectionType = {:?}", &data_str.connType);
                *conn_type = data_str.connType;


                // 1 准备返回的数据
                let s = String::from("{}");
                let bytes = s.into_bytes();

                // 2 开始组装数据
                let mut buf = BytesMut::new();
                buf.put_u16(rpc_data.MagicNum);
                buf.put_u32(rpc_data.ReqID);
                buf.put_u16(rpc_data.MsgType.to_u16());
                buf.put_slice(&bytes);

                println!("send data{:?}, len = {}", &buf, &buf.len());
                let _ = sender.send(Bytes::from(buf)).await;
            },
            _ => {},
        };
    }

    Ok(())
}
