mod config;
pub use config::*;

use bytes::{BufMut, Bytes, BytesMut};
use entity::{ConnectionType, DataOperation};

mod data_models;
mod entity;

mod errors;
pub use errors::*;
use tokio::sync::Mutex;

use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, net::SocketAddr};

use crate::data_models::SetConnectionTypeReq;
use crate::{
    data_models::{
        CreateRoomResp, LoginReq, RPCData, RPCMessageType, LENGTH_ADJUSTMENT, LENGTH_FIELD_LENGTH,
    },
    entity::*,
    ServerConfig,
};

use futures::{stream::SplitSink, SinkExt, StreamExt};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
};
use tokio::{sync::oneshot, time::sleep};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

type FramedStreamSender<'a> = SplitSink<&'a mut Framed<TcpStream, LengthDelimitedCodec>, Bytes>;

type FramedStream = Framed<TcpStream, LengthDelimitedCodec>;

/// Shorthand for the transmit half of the message channel.
type Tx = mpsc::UnboundedSender<BytesMut>;

/// Shorthand for the receive half of the message channel.
type Rx = mpsc::UnboundedReceiver<BytesMut>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_conf = ServerConfig::load("conf/config.toml")?;
    let listen_addr = server_conf.listen_address.addr;

    let listener = TcpListener::bind(&listen_addr).await?;
    println!("Listening on {} ......", listen_addr);

    // 创建一个访问和操作共享数据的通道
    // let (tx, rx) = mpsc::channel(32);

    // let tx1 = tx.clone();

    // 初始化数据channel
    // tokio::spawn(handle_data_channel(rx));

    // 初始化数据管理器
    let state = Arc::new(Mutex::new(DataManager::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Client: {:?} connected", addr);

        // Clone a handle to the `Shared` state for the new connection.
        let state = Arc::clone(&state);

        // let tmp_tx = tx.clone();
        tokio::spawn(async move {
            // 使用Frame的LengthDelimitedCodec进行编解码操作
            let codec = LengthDelimitedCodec::builder()
                .max_frame_length(1024)
                .length_adjustment(LENGTH_ADJUSTMENT)
                .length_field_length(LENGTH_FIELD_LENGTH)
                .num_skip(0)
                .new_codec();
            let mut stream = Framed::new(stream, codec);

            if let Err(e) = handle_connection(state, stream, addr).await {
                println!("an error occurred; error = {:?}", e);
            }

            println!("Client {:?} disconnected", addr);
        });
    }

    // let _ = handle_data_channel.await.unwrap();
}

async fn handle_connection(
    state: Arc<Mutex<DataManager>>,
    mut stream: FramedStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    // Read the first request from the stream to get the connection type.
    let (rpc_data, connection_type) = match stream.next().await {
        Some(Ok(mut buf)) => {
            let mut temp_rpc_data = RPCData::default();
            let mut temp_connenction_type = ConnectionType::Unknown;
            if let Some(rpc_data) = RPCData::from(&mut buf) {
                match rpc_data.MsgType {
                    RPCMessageType::SetConnectionType => {
                        let data: SetConnectionTypeReq = serde_json::from_slice(&rpc_data.Data)?;
                        println!("data_str.connectionType = {:?}", &data.connType);
                        temp_rpc_data = rpc_data;
                        temp_connenction_type = data.connType;
                    }
                    _ => {}
                };
            }
            (temp_rpc_data, temp_connenction_type)
        }
        // We didn't get a line so we return early here.
        _ => {
            println!(
                "Failed to get ConnectionType from {}. Client disconnected.",
                addr
            );
            return Ok(());
        }
    };

    match connection_type {
        ConnectionType::UEServer => {
            // 响应设置连接类型的请求
            let s = String::from("{}");
            let bytes = s.into_bytes();
            let mut buf = BytesMut::new();
            buf.put_u16(rpc_data.MagicNum);
            buf.put_u32(rpc_data.ReqID);
            buf.put_u16(rpc_data.MsgType.to_u16());
            buf.put_slice(&bytes);
            println!("send data{:?}, len = {}", &buf, &buf.len());
            let _ = stream.send(Bytes::from(buf)).await;

            let peer = Peer::new(stream)?;
            let mut ue_server = UEServer::new(state.clone(), peer).await?;

            loop {
                tokio::select! {

                    // A message was received from a peer. Send it to the current user.
                    Some(msg) = ue_server.peer.rx.recv() => {
                        ue_server.peer.stream.send(Bytes::from(msg)).await?;
                    }

                    result = ue_server.peer.stream.next() => match result {
                        Some(Ok(mut buf)) => {

                            if let Ok(_) = ue_server.handle_ue_server(&mut buf).await {}

                        }
                        // An error occurred.
                        Some(Err(e)) => {
                            println!(
                                "an error occurred while processing messages for ; error = {:?}",
                                e
                            );
                        }
                        // The stream has been exhausted.
                        None => break,
                    },
                }
            }
        }
        ConnectionType::Player => {
            // 响应设置连接类型的请求
            let s = String::from("{}");
            let bytes = s.into_bytes();
            let mut buf = BytesMut::new();
            buf.put_u16(rpc_data.MagicNum);
            buf.put_u32(rpc_data.ReqID);
            buf.put_u16(rpc_data.MsgType.to_u16());
            buf.put_slice(&bytes);
            println!("send data{:?}, len = {}", &buf, &buf.len());
            let _ = stream.send(Bytes::from(buf)).await;

            let peer = Peer::new(stream)?;
            let mut player = Player::new(state.clone(), peer).await?;

            loop {
                tokio::select! {

                    // A message was received from a peer. Send it to the current user.
                    Some(msg) = player.peer.rx.recv() => {
                        player.peer.stream.send(Bytes::from(msg)).await?;
                    }

                    result = player.peer.stream.next() => match result {
                        Some(Ok(mut buf)) => {
                            if let Ok(_) = player.handle_player(&mut buf).await {}
                        }
                        // An error occurred.
                        Some(Err(e)) => {
                            println!(
                                "an error occurred while processing messages for ; error = {:?}",
                                e
                            );
                        }
                        // The stream has been exhausted.
                        None => break,
                    },
                }
            }
        }
        ConnectionType::Unknown => {
            println!(
                "Get ConnectionType Unknown from {}. Client disconnected.",
                addr
            );
            return Ok(());
        }
    }

    Ok(())
}

async fn handle_unknow(
    conn_type: &mut ConnectionType,
    buf: &mut BytesMut,
    peer: &mut Peer,
) -> Result<(), Box<dyn Error>> {
    println!(
        "handle_unknow: msg:len = {}, content = {:?}",
        buf.len(),
        buf
    );
    if let Some(rpc_data) = RPCData::from(buf) {
        match rpc_data.MsgType {
            RPCMessageType::SetConnectionType => {
                let data_str: SetConnectionTypeReq = serde_json::from_slice(&rpc_data.Data)?;
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
                let _ = peer.stream.send(Bytes::from(buf)).await;
            }
            _ => {}
        };
    }

    Ok(())
}
