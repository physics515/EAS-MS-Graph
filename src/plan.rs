#![allow(renamed_and_removed_lints)]

use std::{collections::HashMap, str::FromStr};

use rocket::FromForm;
use serde::{Deserialize, Serialize};

/*
{
	"@odata.context": "https://graph.microsoft.com/v1.0/$metadata#planner/plans/$entity",
	"@odata.etag": "W/\"JzEtUGxhbiAgQEBAQEBAQEBAQEBAQEBARCc=\"",
	"createdDateTime": "2023-01-31T22:42:34.5740015Z",
	"owner": "01b95734-9bf9-4015-b065-4d300ddcd1fb",
	"title": "Automation Tasks",
	"id": "-GIcbyTe1U-HMRIICi4gRmUAHMV2",
	"createdBy": {
		"user": {
			"displayName": null,
			"id": "c258b80f-26fb-435d-8ad5-95f588c953e5"
		},
		"application": {
			"displayName": null,
			"id": "1fec8e78-bce4-4aaf-ab1b-5451cc387264"
		}
	},
	"container": {
		"containerId": "01b95734-9bf9-4015-b065-4d300ddcd1fb",
		"type": "group",
		"url": "https://graph.microsoft.com/v1.0/groups/01b95734-9bf9-4015-b065-4d300ddcd1fb"
	}
}
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
	#[serde(rename = "@odata.context")]
	pub odata_context: Option<String>,
	#[serde(rename = "@odata.etag")]
	pub odata_etag: Option<String>,
	#[serde(rename = "createdDateTime")]
	pub created_date_time: Option<String>,
	pub owner: Option<String>,
	pub title: Option<String>,
	pub id: String,
	#[serde(rename = "createdBy")]
	pub created_by: Option<CreatedBy>,
	pub container: Option<Container>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedBy {
	pub user: Option<User>,
	pub application: Option<Application>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
	#[serde(rename = "displayName")]
	pub display_name: Option<String>,
	pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
	#[serde(rename = "displayName")]
	pub display_name: Option<String>,
	pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
	#[serde(rename = "containerId")]
	pub container_id: String,
	#[serde(rename = "type")]
	pub type_: Option<String>,
	pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCollection {
	#[serde(rename = "@odata.context")]
	pub odata_context: Option<String>,
	#[serde(rename = "@odata.nextLink")]
	pub odata_next_link: Option<String>,
	#[serde(rename = "@odata.count")]
	pub odata_count: Option<i64>,
	pub value: Option<Vec<Plan>>,
}

/*
{
  "creationSource": {"@odata.type": "#microsoft.graph.plannerBucketCreation"},
  "id": "String (identifier)",
  "name": "String",
  "orderHint": "String",
  "planId": "String"
}
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
	#[serde(rename = "creationSource")]
	creation_source: Option<CreationSource>,
	id: Option<String>,
	name: Option<String>,
	#[serde(rename = "orderHint")]
	order_hint: Option<String>,
	#[serde(rename = "planId")]
	plan_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreationSource {
	#[serde(rename = "@odata.type")]
	odata_type: Option<String>,
}

#[derive(FromForm, Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanForm {
	pub plan_name: String,
	pub plan_template: String,
}

impl CreatePlanForm {
	#[must_use]
	pub fn to_create_plan(&self) -> CreatePlan {
		CreatePlan {
			plan_name: self.plan_name.clone(),
			plan_template: match PlanTemplateType::from_str(&self.plan_template) {
				Ok(t) => t,
				Err(()) => PlanTemplateType::Default,
			},
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlan {
	pub plan_name: String,
	pub plan_template: PlanTemplateType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlanTemplateType {
	Default,
	Project,
}

impl FromStr for PlanTemplateType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"project" => Ok(Self::Project),
			_ => Ok(Self::Default),
		}
	}
}

pub struct PlanTemplateSpec {
	pub buckets: HashMap<String, Vec<String>>,
}

impl PlanTemplateType {
	#[must_use]
	pub fn to_spec(&self) -> PlanTemplateSpec {
		match self {
			Self::Default => PlanTemplateSpec { buckets: HashMap::new() },
			Self::Project => {
				let mut buckets = HashMap::new();
				buckets.insert("Tech Work".to_owned(), Vec::new());
				buckets.insert("Design Work".to_owned(), Vec::new());
				buckets.insert("Install Work".to_owned(), Vec::new());
				PlanTemplateSpec { buckets }
			}
		}
	}
}
