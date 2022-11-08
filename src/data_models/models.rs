use serde::{Serialize, Deserialize};

// 请求的参数
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReq
{
	pub name: String,
	pub pwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomReq
{
	pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomResp
{
	pub RoomHost: String,
}
