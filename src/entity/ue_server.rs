use bytes::BytesMut;

use crate::{EspectError, data_models::{RPCMessageType, RPCData}, FramedStream, Rx};

use super::Player;

pub struct UEServer {

    // socket stream
    stream: FramedStream,

    /// Receive half of the message channel.
    rx: Rx,

    // 服务器房间端口
    room_host: String,

    // 课件ID
    lesson_id: String,

    // 房间创建者ID
    creater_id: String,

    creater: Player,
    
}


pub async fn handle_ue_server(buf: &mut BytesMut)  -> Result<(), EspectError> {
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