use eggersmann_app_server_auth::User;
use azure_identity::ImdsManagedIdentityCredential;
use azure_security_keyvault::KeyvaultClient;
use rocket::form::Form;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use eggersmann_app_server_auth::MSAccessToken;

pub use me::*;
pub use channel::*;
pub use drive::*;
pub use group::*;
pub use plan::*;
pub use team::*;
pub use site::*;

mod me;
mod channel;
mod drive;
mod group;
mod plan;
mod team;
mod site;

pub struct MSGraph {
	token: MSAccessToken,
}

impl MSGraph {
	/// Create a new MSGraph instance and authenticate with the Microsoft Graph API.
	/// key_vault_name: The name of the Azure Key Vault that contains the secrets. (e.g. "eggappserverkeyvault")
	/// client_id_key: The name of the secret that contains the client ID. (e.g. "ms-auth-client-id")
	/// client_secret_key: The name of the secret that contains the client secret. (e.g. "ms-auth-client-secret")
	/// tenant_id_key: The name of the secret that contains the tenant ID. (e.g. "ms-auth-tenant-id")
	///
	pub async fn new(key_vault_name: &str, client_id_key: &str, client_secret_key: &str, tenant_id_key: &str) -> Result<Self, String> {
		let azure_credentials = ImdsManagedIdentityCredential::default();
		let azure_key_vault_client = KeyvaultClient::new(&format!("https://{key_vault_name}.vault.azure.net"), Arc::new(azure_credentials)).unwrap().secret_client();

		// Get the secrets from the Azure Key Vault.
		let client_id = match azure_key_vault_client.get(client_id_key).await {
			Ok(client_id) => client_id.value,
			Err(e) => return Err(e.to_string()),
		};

		let client_secret = match azure_key_vault_client.get(client_secret_key).await {
			Ok(client_secret) => client_secret.value,
			Err(e) => return Err(e.to_string()),
		};

		let tenant_id = match azure_key_vault_client.get(tenant_id_key).await {
			Ok(tenant_id) => tenant_id.value,
			Err(e) => return Err(e.to_string()),
		};

		let client = reqwest::Client::new();
		let scopes = vec!["https://graph.microsoft.com/.default".to_string()].join(" ").to_string();
		let mut params = HashMap::new();
		params.insert("client_id", client_id.as_str());
		params.insert("client_secret", client_secret.as_str());
		params.insert("grant_type", "client_credentials");
		params.insert("scope", &scopes);

		let res = client.post(format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token")).form(&params).send().await;
		match res {
			Ok(res) => {
				let token = res.json::<MSAccessToken>().await;
				match token {
					Ok(token) => Ok(Self { token }),
					Err(err) => Err(err.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	/// Get the current user.
	pub async fn me(&self, user: User) -> Result<Me, String> {
		let client = reqwest::Client::new();
		let res = client.get("https://graph.microsoft.com/v1.0/me").bearer_auth(&user.token.ms_token.access_token).send().await;

		match res {
			Ok(res) => {
				let json = res.json::<Me>().await;
				match json {
					Ok(json) => Ok(json),
					Err(err) => Err(err.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	/// Get the photo of the current user.
	pub async fn me_photo(&self, user: User) -> Result<Vec<u8>, String> {
		let client = reqwest::Client::new();
		let res = client.get("https://graph.microsoft.com/v1.0/me/photo/$value").bearer_auth(&user.token.ms_token.access_token).send().await;
		match res {
			Ok(res) => {
				let bytes = res.bytes().await;
				match bytes {
					Ok(bytes) => Ok(bytes.to_vec()),
					Err(err) => Err(err.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	/// Create a new shared channel in a team.
	pub async fn automation_teams_create_shared_channel(&self, data: Form<CreateSharedChannelForm>) -> Result<(Team, Channel, Channel), String> {
		//let automation = MSGraphAutomation::new().await;
		let client = reqwest::Client::new();

		let members = vec![ChannelMember {
			odata_type: "#microsoft.graph.aadUserConversationMember".to_string(),
			user_odata_bind: "https://graph.microsoft.com/v1.0/users('".to_string() + &data.owner_id + "')",
			roles: vec!["owner".to_string()],
		}];

		let team = match self.automation_team_by_name(data.team_name.clone()).await {
			Ok(team) => team,
			Err(err) => return Err(format!(" Error getting team {}: {}", data.team_name.clone(), err)),
		};

		let body = json!(CreateChannelBody { display_name: data.channel_display_name.clone(), description: data.channel_description.clone(), membership_type: "shared".to_string(), members });
		let uri = format!("https://graph.microsoft.com/v1.0/teams/{}/channels", team.id.clone());
		println!("uri: {}, Creating channel: {}", uri.clone(), body.clone());
		let res = client.post(uri).json(&body).bearer_auth(&self.token.access_token).send().await;

		match res {
			Ok(_) => {
				//println!("Created channel: {}", res.json::<Value>().await.unwrap().to_string());
				let team = match self.automation_team_by_name(data.team_name.clone()).await {
					Ok(team) => team,
					Err(err) => return Err(format!("Error getting team: {err}")),
				};
				let channel = match self.automation_channel_by_name(team.id.clone(), data.channel_display_name.clone()).await {
					Ok(channel) => channel,
					Err(err) => return Err(format!("Error getting newly created channel: {err}")),
				};
				let general = match self.automation_channel_by_name(team.id.clone(), "General".to_owned()).await {
					Ok(channel) => channel,
					Err(err) => return Err(format!("Error getting general channel: {err}")),
				};

				// add member_id as owner to channel
				let res = self.automation_add_channel_owner(team.clone(), channel.clone(), data.member_id.clone()).await;
				match res {
					Ok(_) => (),
					Err(err) => return Err(format!("Error adding owner to channel: {err}")),
				}

				match &data.plan {
					Some(plan) => {
                                                let plan = plan.to_create_plan();
						let created_plan = self.automation_create_plan(plan.plan_name.clone(), team.display_name.clone().unwrap().clone()).await;
						let created_plan = match created_plan {
							Ok(created_plan) => created_plan,
							Err(err) => return Err(format!("Error creating plan: {err}")),
						};

                                                let spec = plan.plan_template.to_spec();

                                                for (bucket_name, _) in spec.buckets {
                                                        let res = self.automation_add_bucket_to_plan(created_plan.title.clone().unwrap_or(plan.plan_name.clone()).clone(), team.display_name.clone().unwrap().clone(), bucket_name.clone()).await;
                                                        match res {
                                                                Ok(_) => (),
                                                                Err(err) => return Err(format!("Error adding bucket to plan: {err}")),
                                                        }
                                                }

						let res = self.automation_add_plan_tab_to_teams_channel(&format!("{} Tasks", &channel.display_name.clone().unwrap()), team.clone(), channel.clone(), created_plan).await;
						match res {
							Ok(_) => (),
							Err(err) => return Err(format!("Error adding plan tab to channel: {err}")),
						}
						
					}
					None => (),
				}
				Ok((team, channel, general))
			}
			Err(err) => Err(format!("Error creating channel: {err}")),
		}
	}

	/// Add a owner to a channel.
	/// post https://graph.microsoft.com/v1.0/teams/{team-id}/channels/{channel-id}/members
	///
	/// {
	///     "@odata.type": "#microsoft.graph.aadUserConversationMember",
	///     "roles": ["owner"],
	///     "user@odata.bind": "https://graph.microsoft.com/v1.0/users('{member-id}')"
	/// }
	///
	pub async fn automation_add_channel_owner(&self, team: Team, channel: Channel, owner_id: String) -> Result<(), String> {
		let client = reqwest::Client::new();

		let body = json!(ChannelMember {
			odata_type: "#microsoft.graph.aadUserConversationMember".to_string(),
			user_odata_bind: "https://graph.microsoft.com/v1.0/users('".to_string() + &owner_id + "')",
			roles: vec!["owner".to_string()],
		});

		let uri = format!("https://graph.microsoft.com/v1.0/teams/{}/channels/{}/members", team.id.clone(), channel.id.unwrap());
		println!("uri: {}, Adding owner: {}", uri.clone(), body.clone());
		let res = client.post(uri).json(&body).bearer_auth(&self.token.access_token).send().await;

		match res {
			Ok(res) => {
				println!("Added owner: {}", res.json::<Value>().await.unwrap());
				Ok(())
			}
			Err(err) => Err(format!("Error adding owner: {err}")),
		}
	}

	/// Get a channel by name
	pub async fn automation_channel_by_name(&self, team_id: String, channel_name: String) -> Result<Channel, String> {
		let client = reqwest::Client::new();
		let res = client.get(format!("https://graph.microsoft.com/v1.0/teams/{team_id}/channels")).bearer_auth(&self.token.access_token).send().await;
		match res {
			Ok(res) => {
				let body = match res.text().await {
					Ok(body) => {
						println!("Channels Body: {body}");
						body
					}
					Err(err) => return Err(format!("Error Decoding Body: {err}")),
				};
				let json = serde_json::from_str::<ChannelCollection>(&body);
				match json {
					Ok(json) => {
						let channel = json.value.iter().find(|channel| channel.display_name == Some(channel_name.clone()));
						match channel {
							Some(channel) => Ok(channel.clone()),
							None => Err("No matching channel found.".to_string()),
						}
					}
					Err(err) => Err(format!("Error Deserializing Channels JSON: {err}")),
				}
			}
			Err(err) => Err(format!("Error getting Channels: {err}")),
		}
	}

	/// Get all groups that have the Team resourceProvisioningOption
	pub async fn automation_groups_with_teams(&self) -> Result<Vec<Group>, String> {
		let client = reqwest::Client::new();
		let res = client.get("https://graph.microsoft.com/beta/groups?$filter=resourceProvisioningOptions/Any(x:x+eq+'Team')").bearer_auth(&self.token.access_token).send().await;
		match res {
			Ok(res) => {
				let group_collection = res.json::<GroupCollection>().await;
				match group_collection {
					Ok(group_collection) => Ok(group_collection.value),
					Err(err) => Err(err.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	/// Get a team by group id
	pub async fn automation_team_by_group_id(&self, group_id: String) -> Result<Team, String> {
		let client = reqwest::Client::new();
		let res = client.get(format!("https://graph.microsoft.com/v1.0/groups/{group_id}/team")).bearer_auth(&self.token.access_token).send().await;
		match res {
			Ok(res) => {
				let team = res.json::<Team>().await;
				match team {
					Ok(team) => Ok(team),
					Err(err) => Err(err.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	/// Get team by name
	pub async fn automation_team_by_name(&self, team_name: String) -> Result<Team, String> {
		let groups_with_teams = match self.automation_groups_with_teams().await {
			Ok(groups_with_teams) => groups_with_teams,
			Err(err) => return Err(format!("Error getting groups with teams: {err}")),
		};

		let group = match groups_with_teams.iter().find(|group| group.display_name == Some(team_name.clone())) {
			Some(group) => group,
			None => return Err(format!("Could not find group with name {}", team_name.clone())),
		};

		let team = match self.automation_team_by_group_id(group.id.clone()).await {
			Ok(team) => team,
			Err(err) => return Err(format!("Error getting team by id {}: {}", group.id.clone(), err)),
		};

		Ok(team)
	}

	pub async fn sites(&self) -> Result<String, String> {
		let client = reqwest::Client::new();
		let res = client.get("https://graph.microsoft.com/beta/sites").bearer_auth(&self.token.access_token).send().await;
		match res {
			Ok(res) => {
				let json = res.json::<serde_json::Value>().await;
				match json {
					Ok(json) => Ok(json.to_string()),
					Err(err) => Err(err.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	pub async fn get_site_by_name(&self, site_name: String) -> Result<Site, String> {
		let sites = self.sites().await;
		match sites {
			Ok(sites) => {
				let json = serde_json::from_str::<SiteCollection>(&sites);
				match json {
					Ok(json) => {
						let site = json.value.iter().find(|site| site.display_name == Some(site_name.clone()));
						match site {
							Some(site) => Ok(site.clone()),
							None => Err("No matching site found.".to_string()),
						}
					}
					Err(err) => Err(format!("Error Deserializing Sites JSON: {err}")),
				}
			}
			Err(err) => Err(err),
		}
	}

	pub async fn site_pages(&self, site_id: String) -> Result<String, String> {
		let client = reqwest::Client::new();
		let res = client.get(format!("https://graph.microsoft.com/beta/sites/{site_id}/pages")).bearer_auth(&self.token.access_token).send().await;
		match res {
			Ok(res) => {
				let json = res.json::<serde_json::Value>().await;
				match json {
					Ok(json) => Ok(json.to_string()),
					Err(err) => Err(err.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	pub async fn site_drives(&self, site_id: String) -> Result<DriveCollection, String> {
		let client = reqwest::Client::new();
		let res = client.get(format!("https://graph.microsoft.com/beta/sites/{site_id}/drives")).bearer_auth(&self.token.access_token).send().await;
		match res {
			Ok(res) => match res.json::<DriveCollection>().await {
				Ok(collection) => Ok(collection),
				Err(err) => Err(err.to_string()),
			},
			Err(err) => Err(err.to_string()),
		}
	}

	pub async fn get_drive_by_name(&self, site_id: String, drive_name: String) -> Result<Drive, String> {
		let drives = self.site_drives(site_id).await;
		match drives {
			Ok(drives) => {
				let drive = drives.value.iter().find(|drive| drive.name == Some(drive_name.clone()));
				match drive {
					Some(drive) => Ok(drive.clone()),
					None => Err("No matching drive found.".to_string()),
				}
			}
			Err(err) => Err(err),
		}
	}

	pub async fn put_item_in_site_drive(&self, site_name: &str, drive_name: &str, file_name: &str, item: Vec<u8>) -> Result<String, String> {
		let site = match self.get_site_by_name(site_name.to_string()).await {
			Ok(site) => site,
			Err(err) => return Err(err),
		};
		let drive = match self.get_drive_by_name(site.id.clone(), drive_name.to_owned()).await {
			Ok(drive) => drive,
			Err(err) => return Err(err),
		};

		let client = reqwest::Client::new();
		let res = client.put(format!("https://graph.microsoft.com/beta/drives/{}/items/{}:/{}:/content", drive.id, site.id, file_name)).bearer_auth(&self.token.access_token).body(item).send().await;
		match res {
			Ok(res) => {
				let json = res.json::<serde_json::Value>().await;
				match json {
					Ok(json) => Ok(json.to_string()),
					Err(err) => Err(err.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	pub async fn automation_create_plan(&self, plan_name: String, team_name: String) -> Result<Plan, String> {
		let team = match self.automation_team_by_name(team_name).await {
			Ok(team) => team,
			Err(err) => return Err(err),
		};

		let client = reqwest::Client::new();
		let body = json!({
			"container": {
				"url": format!("https://graph.microsoft.com/beta/groups/{}", team.id),
			},
			"title": plan_name,
		});

		println!("{}", body);

		let res = client.post("https://graph.microsoft.com/beta/planner/plans").json(&body).bearer_auth(&self.token.access_token);
		println!("create plan request: {:?}", res);

		match res.send().await {
			Ok(res) => {
				let json = res.json::<serde_json::Value>().await.unwrap();
				println!("create plan response json: {}", json);
				let plan: Plan = match serde_json::from_value(json) {
					Ok(plan) => plan,
					Err(err) => return Err(err.to_string()),
				};
				Ok(plan)
			}
			Err(err) => Err(err.to_string()),
		}
	}

	pub async fn automation_get_plan_by_name(&self, plan_name: String, team_name: String) -> Result<Plan, String> {
		let team = match self.automation_team_by_name(team_name).await {
			Ok(team) => team,
			Err(err) => return Err(err),
		};

		let client = reqwest::Client::new();
		let res = client.get(format!("https://graph.microsoft.com/beta/groups/{}/planner/plans", team.id)).bearer_auth(&self.token.access_token).send().await;
		match res.unwrap().json::<PlanCollection>().await {
			Ok(plans) => {
				let plans = plans.value.expect("No plans found for team.");
				let plan = plans.iter().find(|plan| plan.title == Some(plan_name.clone()));
				match plan {
					Some(plan) => Ok(plan.clone()),
					None => Err("No matching plan found.".to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
		}
	}

	pub async fn automation_add_bucket_to_plan(&self, plan_name: String, team_name: String, bucket_name: String) -> Result<Bucket, String> {
		let plan = match self.automation_get_plan_by_name(plan_name, team_name).await {
			Ok(plan) => plan,
			Err(err) => return Err(err),
		};

		let client = reqwest::Client::new();
		let body = json!({
				"name": bucket_name,
				"planId": plan.id,
				"orderHint": " !",
		});

		println!("Add Bucket Body: {}", body);

		let res = client.post("https://graph.microsoft.com/beta/planner/buckets").json(&body).bearer_auth(&self.token.access_token);

		println!("Add Bucket Request: {:?}", res);

		match res.send().await {
			Ok(res) => {
				let json = res.json::<serde_json::Value>().await.unwrap();
				println!("Add Bucket Response: {}", json);
				let bucket: Bucket = match serde_json::from_value(json) {
					Ok(bucket) => bucket,
					Err(err) => return Err(err.to_string()),
				};
				Ok(bucket)
			}
			Err(err) => Err(err.to_string()),
		}
	}

	pub async fn automation_add_plan_tab_to_teams_channel(&self, tab_name: &str, team: Team, channel: Channel, plan: Plan) -> Result<TeamsTab, String> {
		let client = reqwest::Client::new();

                // add app to team
                let body = json!({
                        "teamsApp@odata.bind": "https://graph.microsoft.com/v1.0/appCatalogs/teamsApps/com.microsoft.teamspace.tab.planner"
                });
                let url = format!("https://graph.microsoft.com/beta/teams/{}/installedApps", team.id);

                println!("add app to team url: {}", url);
                println!("add app to team body: {}", body);

                let res = client.post(url).json(&body).bearer_auth(&self.token.access_token).send().await;
                match res {
                        Ok(res) => {
                                let json = res.json::<serde_json::Value>().await;
                                match json {
                                        Ok(json) => println!("add app to team response: {}", json),
                                        Err(err) => return Err(err.to_string()),
                                }
                        }
                        Err(err) => return Err(err.to_string()),
                }



                // add tab to channel
		let entity_id = format!("tt.c_{}_p_{}", channel.id.clone().unwrap(), plan.id);
		let content_url = format!("https://tasks.teams.microsoft.com/teamsui/{{tid}}/Home/PlannerFrame?page=7&auth_pvr=OrgId&auth_upn={{userPrincipalName}}&groupId={{groupId}}&planId=@{}&channelId={{channelId}}&entityId={{entityId}}&tid={{tid}}&userObjectId={{userObjectId}}&subEntityId={{subEntityId}}&sessionId={{sessionId}}&theme={{theme}}&mkt={{locale}}&ringId={{ringId}}&PlannerRouteHint={{tid}}", &plan.id);
		let remove_url = format!("https://tasks.teams.microsoft.com/teamsui/{{tid}}/Home/PlannerFrame?page=13&auth_pvr=OrgId&auth_upn={{userPrincipalName}}&groupId={{groupId}}&planId=@{}&channelId={{channelId}}&entityId={{entityId}}&tid={{tid}}&userObjectId={{userObjectId}}&subEntityId={{subEntityId}}&sessionId={{sessionId}}&theme={{theme}}&mkt={{locale}}&ringId={{ringId}}&PlannerRouteHint={{tid}}", &plan.id);
		let web_url = format!("https://tasks.office.com/{{tid}}/Home/PlanViews/@{}?Type=PlanLink&Channel=TeamsTab", &plan.id);
		let body = json!({
				"displayName": tab_name,
				"teamsApp@odata.bind": "https://graph.microsoft.com/v1.0/appCatalogs/teamsApps/com.microsoft.teamspace.tab.planner",
				"configuration": {
					"entityId": entity_id,
					"contentUrl": content_url,
					"removeUrl": remove_url,
					"websiteUrl": web_url,
				}
		});
                let url = format!("https://graph.microsoft.com/v1.0/teams/{}/channels/{}/tabs", team.id, channel.id.unwrap());

                println!("Add Tab URL: {}", url);

		println!("Add Tab Body: {}", body);

		let res = client.post(url).json(&body).bearer_auth(&self.token.access_token).send().await;

		match res {
			Ok(res) => {
                                let res = res.json::<serde_json::Value>().await.unwrap();
                                println!("Add Tab Response: {}", res);
                                let res = serde_json::from_value::<TeamsTab>(res);
                                match  res {
        				Ok(tab) => {
        					Ok(tab)
        				}
        				Err(err) => Err(err.to_string()),
        			}
                        },
			Err(err) => Err(err.to_string()),
		}
	}
}

