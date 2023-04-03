use serde::{Deserialize, Serialize};

///
/// Graph API team object.
///
/* {
	"@odata.context": "https://graph.microsoft.com/v1.0/$metadata#teams/$entity",
	"id": "197b10b2-2113-4009-96e8-ef4fdbf3d4c8",
	"createdDateTime": "2021-08-17T20:51:57.767Z",
	"displayName": "New York",
	"description": "New York Showroom Team Site",
	"internalId": "19:ubg6jkK4Z3tKz_ywc0NWg3eS3-3qxPrjAmoz8kmaY3o1@thread.tacv2",
	"classification": null,
	"specialization": "none",
	"visibility": "private",
	"webUrl": "https://teams.microsoft.com/l/team/19%3aubg6jkK4Z3tKz_ywc0NWg3eS3-3qxPrjAmoz8kmaY3o1%40thread.tacv2/conversations?groupId=197b10b2-2113-4009-96e8-ef4fdbf3d4c8&tenantId=2ce577f8-c6c2-43b5-ad55-848315910e1a",
	"isArchived": false,
	"isMembershipLimitedToOwners": false,
	"discoverySettings": {
		"showInTeamsSearchAndSuggestions": false
	},
	"memberSettings": {
		"allowCreateUpdateChannels": true,
		"allowCreatePrivateChannels": true,
		"allowDeleteChannels": true,
		"allowAddRemoveApps": true,
		"allowCreateUpdateRemoveTabs": true,
		"allowCreateUpdateRemoveConnectors": true
	},
	"guestSettings": {
		"allowCreateUpdateChannels": false,
		"allowDeleteChannels": false
	},
	"messagingSettings": {
		"allowUserEditMessages": true,
		"allowUserDeleteMessages": true,
		"allowOwnerDeleteMessages": true,
		"allowTeamMentions": true,
		"allowChannelMentions": true
	},
	"funSettings": {
		"allowGiphy": true,
		"giphyContentRating": "moderate",
		"allowStickersAndMemes": true,
		"allowCustomMemes": true
	},
	"summary": {
		"ownersCount": 2,
		"membersCount": 13,
		"guestsCount": 0
	}
} */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
	#[serde(rename = "@odata.context")]
	pub odata_context: Option<String>,

	#[serde(rename = "id")]
	pub id: String,

	#[serde(rename = "createdDateTime")]
	pub created_date_time: Option<String>,

	#[serde(rename = "displayName")]
	pub display_name: Option<String>,

	#[serde(rename = "description")]
	pub description: Option<String>,

	#[serde(rename = "internalId")]
	pub internal_id: Option<String>,

	#[serde(rename = "classification")]
	pub classification: Option<String>,

	#[serde(rename = "specialization")]
	pub specialization: Option<String>,

	#[serde(rename = "visibility")]
	pub visibility: Option<String>,

	#[serde(rename = "webUrl")]
	pub web_url: Option<String>,

	#[serde(rename = "isArchived")]
	pub is_archived: Option<bool>,

	#[serde(rename = "isMembershipLimitedToOwners")]
	pub is_membership_limited_to_owners: Option<bool>,

	#[serde(rename = "discoverySettings")]
	pub discovery_settings: Option<DiscoverySettings>,

	#[serde(rename = "memberSettings")]
	pub member_settings: Option<MemberSettings>,

	#[serde(rename = "guestSettings")]
	pub guest_settings: Option<GuestSettings>,

	#[serde(rename = "messagingSettings")]
	pub messaging_settings: Option<MessagingSettings>,

	#[serde(rename = "funSettings")]
	pub fun_settings: Option<FunSettings>,

	#[serde(rename = "summary")]
	pub summary: Option<Summary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverySettings {
	#[serde(rename = "showInTeamsSearchAndSuggestions")]
	pub show_in_teams_search_and_suggestions: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberSettings {
	#[serde(rename = "allowCreateUpdateChannels")]
	pub allow_create_update_channels: Option<bool>,

	#[serde(rename = "allowCreatePrivateChannels")]
	pub allow_create_private_channels: Option<bool>,

	#[serde(rename = "allowDeleteChannels")]
	pub allow_delete_channels: Option<bool>,

	#[serde(rename = "allowAddRemoveApps")]
	pub allow_add_remove_apps: Option<bool>,

	#[serde(rename = "allowCreateUpdateRemoveTabs")]
	pub allow_create_update_remove_tabs: Option<bool>,

	#[serde(rename = "allowCreateUpdateRemoveConnectors")]
	pub allow_create_update_remove_connectors: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestSettings {
	#[serde(rename = "allowCreateUpdateChannels")]
	pub allow_create_update_channels: Option<bool>,

	#[serde(rename = "allowDeleteChannels")]
	pub allow_delete_channels: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagingSettings {
	#[serde(rename = "allowUserEditMessages")]
	pub allow_user_edit_messages: Option<bool>,

	#[serde(rename = "allowUserDeleteMessages")]
	pub allow_user_delete_messages: Option<bool>,

	#[serde(rename = "allowOwnerDeleteMessages")]
	pub allow_owner_delete_messages: Option<bool>,

	#[serde(rename = "allowTeamMentions")]
	pub allow_team_mentions: Option<bool>,

	#[serde(rename = "allowChannelMentions")]
	pub allow_channel_mentions: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunSettings {
	#[serde(rename = "allowGiphy")]
	pub allow_giphy: Option<bool>,

	#[serde(rename = "giphyContentRating")]
	pub giphy_content_rating: Option<String>,

	#[serde(rename = "allowStickersAndMemes")]
	pub allow_stickers_and_memes: Option<bool>,

	#[serde(rename = "allowCustomMemes")]
	pub allow_custom_memes: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
	#[serde(rename = "ownersCount")]
	pub owners_count: Option<i32>,

	#[serde(rename = "membersCount")]
	pub members_count: Option<i32>,

	#[serde(rename = "guestsCount")]
	pub guests_count: Option<i32>,
}

/*
{
  "displayName": "My Contoso Tab",
  "teamsApp@odata.bind" : "https://graph.microsoft.com/beta/appCatalogs/teamsApps/06805b9e-77e3-4b93-ac81-525eb87513b8",
  "configuration": {
	"entityId": "2DCA2E6C7A10415CAF6B8AB6661B3154",
	"contentUrl": "https://www.contoso.com/Orders/2DCA2E6C7A10415CAF6B8AB6661B3154/tabView",
	"websiteUrl": "https://www.contoso.com/Orders/2DCA2E6C7A10415CAF6B8AB6661B3154",
	"removeUrl": "https://www.contoso.com/Orders/2DCA2E6C7A10415CAF6B8AB6661B3154/uninstallTab"
  }
}
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsTab {
	#[serde(rename = "displayName")]
	pub display_name: Option<String>,
	#[serde(rename = "teamsApp@odata.bind")]
	pub teams_app_data_bind: Option<String>,
	#[serde(rename = "configuration")]
	pub configuration: Option<TeamsTabConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsTabConfiguration {
	#[serde(rename = "entityId")]
	pub entity_id: Option<String>,
	#[serde(rename = "contentUrl")]
	pub content_url: Option<String>,
	#[serde(rename = "websiteUrl")]
	pub website_url: Option<String>,
	#[serde(rename = "removeUrl")]
	pub remove_url: Option<String>,
}
