use serde::{Serialize, Deserialize};

use crate::entity::ConnectionType;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct SetConnectionTypeReq
{
	pub connType: ConnectionType,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SetUEServerInfoReq
{
	pub creater_account: String,
	pub room_port: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SetPlayerInfoReq
{
	pub account: String,
}