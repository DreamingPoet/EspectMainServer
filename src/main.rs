mod config;
pub use config::*;

use bytes::{BufMut, Bytes, BytesMut};
use entity::{ConnectionType, DataOperation};

mod data_models;
mod entity;

mod errors;
pub use errors::*;

use std::error::Error;
use std::time::Duration;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_conf = ServerConfig::load("conf/config.toml")?;
    let listen_addr = server_conf.listen_address.addr;

    let listener = TcpListener::bind(&listen_addr).await?;
    println!("Listening on {} ......", listen_addr);

    // 创建一个访问和操作共享数据的通道
    let (tx, rx) = mpsc::channel(32);

    // let tx1 = tx.clone();

    // 初始化数据channel
    tokio::spawn(handle_data_channel(rx));

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Client: {:?} connected", addr);
        let tmp_tx = tx.clone();
        tokio::spawn(async move {
            // 使用Frame的LengthDelimitedCodec进行编解码操作
            let codec = LengthDelimitedCodec::builder()
                .max_frame_length(1024)
                .length_adjustment(LENGTH_ADJUSTMENT)
                .length_field_length(LENGTH_FIELD_LENGTH)
                .num_skip(0)
                .new_codec();
            let mut stream = Framed::new(stream, codec);

            handle_connection(tmp_tx, &mut stream).await;

            println!("Client {:?} disconnected", addr);
        });
    }

    // let _ = handle_data_channel.await.unwrap();
}


async fn handle_connection(
    tx: Sender<DataOperation>,
    mut stream: &mut Framed<TcpStream, LengthDelimitedCodec>,
) {
    let (mut sender, mut receiver) = stream.split();

    // 该连接的数据====
    let mut connect_type = ConnectionType::Unknown;
    // 该连接的数据====

    // let a  = sender.clone();

    while let Some(Ok(mut buf)) = receiver.next().await {
        // 区分是UEServer 还是 Player，或者是还是第一次连接
        match connect_type {
            ConnectionType::UEServer => if let Ok(_) = handle_ue_server(&mut buf).await {},
            ConnectionType::Player => if let Ok(_) = handle_player(&mut buf, &mut sender).await {},
            ConnectionType::Unknown => {
                if let Ok(_) = handle_unknow(&mut connect_type, &mut buf,  &mut sender).await {}
            }
        }
    }
}
