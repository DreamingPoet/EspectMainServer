use bytes::BytesMut;

use crate::{data_models::{RPCData, RPCMessageType}, EspectError};

pub struct Player {
    account:String,
}


pub async fn handle_player(buf: &mut BytesMut) -> Result<(), EspectError>{

    println!("Receive a msg:len = {}, content = {:?}", buf.len(), buf);
    if let Some(rpc_data) = RPCData::from(buf) {
        // println!("servere get{:?}", rpc_data);
        // let res = rpc_data;
        match rpc_data.MsgType {
            RPCMessageType::HeartBeat => {}
            RPCMessageType::Login => {}
            RPCMessageType::CreateRoom => {
                // let _ = handle_create_room(&rpc_data, &mut stream).await;
            }
            RPCMessageType::Unknown => {}
            RPCMessageType::SetConnectionType => todo!(),
        }
    }

    Ok(())

}