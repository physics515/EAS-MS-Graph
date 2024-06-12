use rocket::http::Status;
use serde::{Deserialize, Serialize};

/* {
	"@odata.context": "https://graph.microsoft.com/v1.0/$metadata#users/$entity",
	"businessPhones": [
		"713.679.8990 x307"
	],
	"displayName": "Justin Icenhour",
	"givenName": "Justin",
	"jobTitle": "Technical Support",
	"mail": "justin@eggersmannusa.com",
	"mobilePhone": "336.682.5892",
	"officeLocation": "Houston, TX",
	"preferredLanguage": null,
	"surname": "Icenhour",
	"userPrincipalName": "justin@eggersmannusa.com",
	"id": "c258b80f-26fb-435d-8ad5-95f588c953e5"
} */
#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
	#[serde(rename = "@odata.context")]
	pub odata_context: Option<String>,
	#[serde(rename = "businessPhones")]
	pub business_phones: Option<Vec<String>>,
	#[serde(rename = "displayName")]
	pub display_name: Option<String>,
	#[serde(rename = "givenName")]
	pub given_name: Option<String>,
	#[serde(rename = "jobTitle")]
	pub job_title: Option<String>,
	#[serde(rename = "mail")]
	pub mail: Option<String>,
	#[serde(rename = "mobilePhone")]
	pub mobile_phone: Option<String>,
	#[serde(rename = "officeLocation")]
	pub office_location: Option<String>,
	#[serde(rename = "preferredLanguage")]
	pub preferred_language: Option<String>,
	#[serde(rename = "surname")]
	pub surname: Option<String>,
	#[serde(rename = "userPrincipalName")]
	pub user_principal_name: Option<String>,
	#[serde(rename = "id")]
	pub id: String,
}

pub struct MeResponse {
	pub me: Option<Me>,
	pub error: Option<String>,
	pub status: Status,
}

impl<'r> rocket::response::Responder<'r, 'static> for MeResponse {
	fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
		if let Some(me) = self.me {
			let json = serde_json::to_string(&me).unwrap();
			rocket::response::Response::build_from(json.respond_to(req).unwrap()).status(self.status).ok()
		} else {
			let json = serde_json::to_string(&self.error).unwrap();
			rocket::response::Response::build_from(json.respond_to(req).unwrap()).status(self.status).ok()
		}
	}
}
