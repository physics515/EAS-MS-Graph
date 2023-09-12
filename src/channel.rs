#![allow(renamed_and_removed_lints)]

use serde::{Deserialize, Serialize};
use rocket::{response::Responder, FromForm};
use crate::Team;
use crate::plan::CreatePlanForm;

///
/// Graph API channel object.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
	pub id: Option<String>,
	pub email: Option<String>,
	pub description: Option<String>,

	#[serde(rename = "createdDateTime")]
	pub created_date_time: Option<String>,

	#[serde(rename = "displayName")]
	pub display_name: Option<String>,

	#[serde(rename = "isFavoriteByDefault")]
	pub is_favorite_by_default: Option<bool>,

	#[serde(rename = "tenantId")]
	pub tenant_id: Option<String>,

	#[serde(rename = "webUrl")]
	pub web_url: Option<String>,

	#[serde(rename = "membershipType")]
	pub membership_type: Option<String>,
}

///
/// Graph API channel member object.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMember {
	pub roles: Vec<String>,

	#[serde(rename(serialize = "@odata.type"))]
	pub odata_type: String,

	#[serde(rename(serialize = "user@odata.bind"))]
	pub user_odata_bind: String,
}

///
/// The body for creating a channel.
/// This struct should be serialized to JSON before sending to the Graph API.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelBody {
	#[serde(rename(serialize = "displayName"))]
	pub display_name: String,
	pub description: String,
	#[serde(rename(serialize = "membershipType"))]
	pub membership_type: String,
	pub members: Vec<ChannelMember>,
}

///
/// Channel Collection
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCollection {
	#[serde(rename = "@odata.context")]
	pub odata_context: String,

	#[serde(rename(serialize = "@odata.count"))]
	pub odata_count: Option<i32>,

	pub value: Vec<Channel>,
}




///
/// The input form for creating a shared channel
///
#[derive(FromForm, Debug, Clone, Serialize, Deserialize)]
pub struct CreateSharedChannelForm {
	pub team_name: String,
	pub channel_display_name: String,
	pub channel_description: String,
	pub owner_id: String,
	pub member_id: String,
	pub plan: Option<CreatePlanForm>,
}



///
/// The response from creating a shared channel
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateSharedChannelResponse {
	pub channel: Option<Channel>,
	pub team: Option<Team>,
	pub general_channel: Option<Channel>,
	pub error: Option<String>,
	pub status: String,
}

impl<'r> Responder<'r, 'static> for CreateSharedChannelResponse {
	fn respond_to(self, _req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
		rocket::serde::json::Json(self).respond_to(_req)
	}
}
