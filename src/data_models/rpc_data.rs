use bytes::{Buf, BytesMut};

pub const LENGTH_FIELD_LENGTH: usize = 4;
pub const LENGTH_ADJUSTMENT: isize = 8;

#[derive(Debug, Default)]
pub enum RPCMessageType {
    HeartBeat,
    Login,
    CreateRoom,
    #[default]
    Unknown,
}
impl RPCMessageType {
    pub fn to_u16(&self) -> u16 {
        match self {
            RPCMessageType::HeartBeat => return 0x00,
            RPCMessageType::Login => return 0x01,
            RPCMessageType::CreateRoom => return 0x02,
            RPCMessageType::Unknown => return 0x03,
        };
    }
}
impl From<u16> for RPCMessageType {
    fn from(orig: u16) -> RPCMessageType {
        match orig {
            0x00 => return RPCMessageType::HeartBeat,
            0x01 => return RPCMessageType::Login,
            0x02 => return RPCMessageType::CreateRoom,
            _ => return RPCMessageType::Unknown,
        };
    }
}

#[derive(Debug, Default)]
pub struct RPCData {
    Len: u32,          // 消息的长度
    pub MagicNum: u16, //魔数,用于快速确定 是不是一个我们需要的包 0x0E0C = 3596
    pub ReqID: u32,    // 本地请求序号
    pub MsgType: RPCMessageType, // 消息类型
    pub Data: Vec<u8>, // Json Bytes
}

impl RPCData {
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
