use serde::{Deserialize, Serialize};

///
/// Graph API group object.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
	#[serde(rename = "id")]
	pub id: String,

	#[serde(rename = "deletedDateTime")]
	pub deleted_date_time: Option<String>,

	#[serde(rename = "classification")]
	pub classification: Option<String>,

	#[serde(rename = "createdDateTime")]
	pub created_date_time: Option<String>,

	#[serde(rename = "createdByAppId")]
	pub created_by_app_id: Option<String>,

	#[serde(rename = "organizationId")]
	pub organization_id: Option<String>,

	#[serde(rename = "description")]
	pub description: Option<String>,

	#[serde(rename = "displayName")]
	pub display_name: Option<String>,

	#[serde(rename = "expirationDateTime")]
	pub expiration_date_time: Option<String>,

	#[serde(rename = "groupTypes")]
	pub group_types: Option<Vec<String>>,

	#[serde(rename = "infoCatalogs")]
	pub info_catalogs: Option<Vec<String>>,

	#[serde(rename = "isAssignableToRole")]
	pub is_assignable_to_role: Option<bool>,

	#[serde(rename = "isManagementRestricted")]
	pub is_management_restricted: Option<bool>,

	#[serde(rename = "mail")]
	pub mail: Option<String>,

	#[serde(rename = "mailEnabled")]
	pub mail_enabled: Option<bool>,

	#[serde(rename = "mailNickname")]
	pub mail_nickname: Option<String>,

	#[serde(rename = "membershipRule")]
	pub membership_rule: Option<String>,

	#[serde(rename = "membershipRuleProcessingState")]
	pub membership_rule_processing_state: Option<String>,

	#[serde(rename = "onPremisesDomainName")]
	pub on_premises_domain_name: Option<String>,

	#[serde(rename = "onPremisesLastSyncDateTime")]
	pub on_premises_last_sync_date_time: Option<String>,

	#[serde(rename = "onPremisesNetBiosName")]
	pub on_premises_net_bios_name: Option<String>,

	#[serde(rename = "onPremisesSamAccountName")]
	pub on_premises_sam_account_name: Option<String>,

	#[serde(rename = "onPremisesSecurityIdentifier")]
	pub on_premises_security_identifier: Option<String>,

	#[serde(rename = "onPremisesSyncEnabled")]
	pub on_premises_sync_enabled: Option<bool>,

	#[serde(rename = "preferredDataLocation")]
	pub preferred_data_location: Option<String>,

	#[serde(rename = "preferredLanguage")]
	pub preferred_language: Option<String>,

	#[serde(rename = "proxyAddresses")]
	pub proxy_addresses: Option<Vec<String>>,

	#[serde(rename = "renewedDateTime")]
	pub renewed_date_time: Option<String>,

	#[serde(rename = "resourceBehaviorOptions")]
	pub resource_behavior_options: Option<Vec<String>>,

	#[serde(rename = "resourceProvisioningOptions")]
	pub resource_provisioning_options: Option<Vec<String>>,

	#[serde(rename = "securityEnabled")]
	pub security_enabled: Option<bool>,

	#[serde(rename = "securityIdentifier")]
	pub security_identifier: Option<String>,

	#[serde(rename = "theme")]
	pub theme: Option<String>,

	#[serde(rename = "visibility")]
	pub visibility: Option<String>,

	#[serde(rename = "onPremisesProvisioningErrors")]
	pub on_premises_provisioning_errors: Option<Vec<String>>,

	#[serde(rename = "writebackConfiguration")]
	pub writeback_configuration: Option<WritebackConfiguration>,
}

///
/// Graph API group writeback object.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritebackConfiguration {
	#[serde(rename = "isEnabled")]
	pub is_enabled: Option<bool>,

	#[serde(rename = "onPremisesGroupType")]
	pub on_premises_group_type: Option<String>,
}

///
/// Graph API group collection object.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupCollection {
	#[serde(rename = "@odata.context")]
	pub odata_context: String,

	#[serde(rename = "value")]
	pub value: Vec<Group>,
}
