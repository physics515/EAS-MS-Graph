use serde::{Deserialize, Serialize};

/* {
		"createdDateTime": "2018-02-09T22:30:55Z",
		"displayName": "Andrea Villamarin",
		"id": "eggersmannusa-my.sharepoint.com,9026785b-c745-476e-abd6-089a9758f0a5,e166d7b9-76ee-4a74-bf20-70464bbb36a9",
		"name": "Andrea Villamarin",
		"root": {

		},
		"siteCollection": {
				"hostname": "eggersmannusa-my.sharepoint.com"
		},
		"webUrl": "https://eggersmannusa-my.sharepoint.com/personal/andrea_villamarin_eggersmannusa_com"
}, */

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Site {
	#[serde(rename = "createdDateTime")]
	pub created_date_time: Option<String>,
	#[serde(rename = "displayName")]
	pub display_name: Option<String>,
	pub id: String,
	pub name: Option<String>,
	pub root: Option<SiteRoot>,
	#[serde(rename = "siteCollection")]
	pub site_collection: Option<SiteCollect>,
	#[serde(rename = "webUrl")]
	pub web_url: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SiteRoot {
	pub id: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SiteCollect {
	pub hostname: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SiteCollection {
	#[serde(rename = "@odata.context")]
	pub odata_context: Option<String>,
	pub value: Vec<Site>,
}
