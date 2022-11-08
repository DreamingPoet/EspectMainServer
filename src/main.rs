mod config;
use bytes::{BufMut, Bytes, BytesMut};
pub use config::*;

mod data_models;

use std::error::Error;
use std::time::Duration;

use crate::{
    data_models::{
        CreateRoomResp, LoginReq, RPCData, RPCMessageType, LENGTH_ADJUSTMENT, LENGTH_FIELD_LENGTH,
    },
    ServerConfig,
};

use futures::{SinkExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_conf = ServerConfig::load("conf/config.toml")?;
    let listen_addr = server_conf.listen_address.addr;

    let listener = TcpListener::bind(&listen_addr).await?;
    println!("Listening on {} ......", listen_addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Client: {:?} connected", addr);

        tokio::spawn(async move {
            // 使用Frame的LengthDelimitedCodec进行编解码操作
            let codec = LengthDelimitedCodec::builder()
                .max_frame_length(1024)
                .length_adjustment(4)
                .length_field_length(LENGTH_FIELD_LENGTH)
                .num_skip(0)
                .new_codec();
            let mut stream = Framed::new(stream, codec);

            while let Some(Ok(mut buf)) = stream.next().await {
                println!("Receive a msg:len = {}, content = {:?}", buf.len(), buf);
                if let Some(rpc_data) = RPCData::from(&mut buf) {
                    // println!("servere get{:?}", rpc_data);
                    // let res = rpc_data;
                    match rpc_data.MsgType {
                        RPCMessageType::HeartBeat => {}
                        RPCMessageType::Login => {}
                        RPCMessageType::CreateRoom => {
                            let _ = handle_create_room(&rpc_data, &mut stream).await;
                        }
                        RPCMessageType::Unknown => {}
                    }
                }
            }
            println!("Client {:?} disconnected", addr);
        });
    }
}

async fn handle_create_room(
    rpc_data: &RPCData,
    stream: &mut Framed<TcpStream, LengthDelimitedCodec>,
) -> Result<(), Box<dyn Error>> {
    // 1 准备返回的数据
    let data = CreateRoomResp {
        RoomHost: "127.0.0.1:7777".to_string(),
    };

    let data_str = serde_json::to_vec(&data)?;

    // 2 开始组装数据
    let mut buf = BytesMut::new();
    buf.put_u16(rpc_data.MagicNum);
    buf.put_u32(rpc_data.ReqID);
    buf.put_u16(rpc_data.MsgType.to_u16());
    buf.put_slice(&data_str);

    println!("send data{:?}, len = {}", &buf, &buf.len());

    stream.send(Bytes::from(buf)).await.unwrap();

    Ok(())
}
