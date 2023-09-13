use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Drive {
	#[serde(rename = "createdDateTime")]
	pub created_date_time: Option<String>,
	pub description: Option<String>,
	pub id: String,
	#[serde(rename = "lastModifiedDateTime")]
	pub last_modified_date_time: Option<String>,
	pub name: Option<String>,
	#[serde(rename = "webUrl")]
	pub web_url: Option<String>,
	#[serde(rename = "driveType")]
	pub drive_type: Option<String>,
	#[serde(rename = "createdBy")]
	pub created_by: Option<DriveUser>,
	#[serde(rename = "lastModifiedBy")]
	pub last_modified_by: Option<DriveUser>,
	pub owner: Option<DriveUser>,
	pub quota: Option<DriveQuota>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DriveUser {
	pub user: Option<DriveUserUser>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DriveUserUser {
	pub email: Option<String>,
	pub id: Option<String>,
	#[serde(rename = "displayName")]
	pub display_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DriveQuota {
	pub deleted: Option<i64>,
	pub remaining: Option<i64>,
	pub state: Option<String>,
	pub total: Option<i64>,
	pub used: Option<i64>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DriveCollection {
	#[serde(rename = "@odata.context")]
	pub odata_context: Option<String>,
	pub value: Vec<Drive>,
}
