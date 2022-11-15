use bytes::{Buf, BytesMut};

pub const LENGTH_FIELD_LENGTH: usize = 4;
pub const LENGTH_ADJUSTMENT: isize = 4;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum RPCMessageType {
    #[default]
    Unknown,
    HeartBeat,
    Login,
    CreateRoom,
    SetConnectionType,
    SetPlayerInfo,
    SetUEServerInfo,
}

impl RPCMessageType {
    pub fn to_u16(&self) -> u16 {
        match self {
            RPCMessageType::Unknown => return 0x00,
            RPCMessageType::HeartBeat => return 0x01,
            RPCMessageType::Login => return 0x02,
            RPCMessageType::CreateRoom => return 0x03,
            RPCMessageType::SetConnectionType => return 0x04,
            RPCMessageType::SetPlayerInfo => return 0x05,
            RPCMessageType::SetUEServerInfo => return 0x06,
        };
    }
}

impl From<u16> for RPCMessageType {
    fn from(orig: u16) -> RPCMessageType {
        match orig {
            0x01 => return RPCMessageType::HeartBeat,
            0x02 => return RPCMessageType::Login,
            0x03 => return RPCMessageType::CreateRoom,
            0x04 => return RPCMessageType::SetConnectionType,
            0x05 => return RPCMessageType::SetPlayerInfo,
            0x06 => return RPCMessageType::SetUEServerInfo,
            _ => return RPCMessageType::Unknown,
        };
    }
}

#[derive(Debug, Default)]
pub struct RPCData {
    Len: u32,                    // 消息的长度
    pub MagicNum: u16,           //魔数,用于快速确定 是不是一个我们需要的包 0x0E0C = 3596
    pub ReqID: u32,              // 本地请求序号
    pub MsgType: RPCMessageType, // 消息类型
    pub Data: Vec<u8>,           // Json Bytes
}

impl RPCData {
    // 读取信息，需要修改BytesMut
    pub fn from(src: &mut BytesMut) -> Option<Self> {
        if src.len() < LENGTH_FIELD_LENGTH {
            return None;
        }

        let mut res = RPCData::default();

        res.Len = src.get_u32();
        res.MagicNum = src.get_u16();
        res.ReqID = src.get_u32();
        res.MsgType = RPCMessageType::from(src.get_u16());
        res.Data = src.get(0..src.len())?.iter().cloned().collect();
        Some(res)
    }
}

pub struct RPCDataLite {                  // 消息的长度
    pub MagicNum: u16,           //魔数,用于快速确定 是不是一个我们需要的包 0x0E0C = 3596
    pub ReqID: u32,              // 本地请求序号
    pub MsgType: RPCMessageType, // 消息类型
}

impl From<RPCData> for RPCDataLite {
    fn from(src: RPCData) -> Self {
        RPCDataLite {
            MagicNum: src.MagicNum,
            ReqID: src.ReqID,
            MsgType: src.MsgType,
        }
    }
}
