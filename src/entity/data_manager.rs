use bytes::{BytesMut, Bytes, BufMut};
use futures::SinkExt;
use serde::{Serialize, Deserialize};
use std::io;
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use std::error::Error;
use tokio::sync::{mpsc::Receiver, oneshot};

use crate::{Tx, Rx, FramedStream};
use crate::{
    data_models::{RPCData, RPCMessageType, SetConnectionTypeReq},
    FramedStreamSender, EspectError,
};

use super::{Player, UEServer};
use tokio::sync::{mpsc, Mutex};

// All Shared Datas
pub struct DataManager {

    pub players: HashMap<SocketAddr, Tx>,
    pub ue_servers: HashMap<SocketAddr, Tx>,

}


impl DataManager {
    /// Create a new, empty, instance of `Shared`.
    pub fn new() -> Self {
        DataManager {
            players: HashMap::new(),
            ue_servers: HashMap::new(),
        }
    }
    
}

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

// ====================================== Peer Start ======================

pub struct Peer {
    pub stream: FramedStream,
    pub rx: Rx,
    pub tx: Tx,
}

impl Peer {
    /// Create a new instance of `Peer`.
    pub fn new(
        stream: FramedStream,
    ) -> io::Result<Peer> {
        // Get the client socket address
        let addr = stream.get_ref().peer_addr()?;
        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded_channel();
        Ok(Peer { stream, rx , tx})
    }
}

// ====================================== Peer End ======================