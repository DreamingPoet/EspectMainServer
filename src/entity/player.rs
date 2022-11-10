use bytes::BytesMut;

use crate::{data_models::{RPCData, RPCMessageType}, EspectError, FramedStreamSender};

pub struct Player {
    account:String,
}


pub async fn handle_player(buf: &mut BytesMut, sender: &mut FramedStreamSender<'_>) -> Result<(), EspectError>{

    println!("handle_player: msg:len = {}, content = {:?}", buf.len(), buf);
    if let Some(rpc_data) = RPCData::from(buf) {
        match rpc_data.MsgType {
            RPCMessageType::HeartBeat => {},
            RPCMessageType::Login => {},
            RPCMessageType::CreateRoom => {
                // let _ = handle_create_room(&rpc_data, &mut stream).await;
            },
            RPCMessageType::SetPlayerInfo => {
                // let _ = handle_create_room(&rpc_data, &mut stream).await;
            },
            _ => todo!(),
        }
    }

    Ok(())

}


// async fn handle_create_room(
//     rpc_data: &RPCData,
//     sender: &mut FramedStreamSender<'_>,
// ) -> Result<(), Box<dyn Error>> {
//     // 1 准备返回的数据
//     let data = CreateRoomResp {
//         RoomHost: "127.0.0.1:7777".to_string(),
//     };

//     let data_str = serde_json::to_vec(&data)?;

//     // 2 开始组装数据
//     let mut buf = BytesMut::new();
//     buf.put_u16(rpc_data.MagicNum);
//     buf.put_u32(rpc_data.ReqID);
//     buf.put_u16(rpc_data.MsgType.to_u16());
//     buf.put_slice(&data_str);

//     println!("send data{:?}, len = {}", &buf, &buf.len());

//     stream.send(Bytes::from(buf)).await.unwrap();

//     Ok(())
// }