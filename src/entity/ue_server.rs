use bytes::BytesMut;

use crate::EspectError;

use super::Player;

pub struct UEServer {
    // 服务器房间端口
    room_host: String,

    // 课件ID
    lesson_id: String,

    // 房间创建者ID
    creater_id: String,

    creater: Player,
}


pub async fn handle_ue_server(buf: &mut BytesMut)  -> Result<(), EspectError> {
    
    Ok(())
}