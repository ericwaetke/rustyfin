use reqwest;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::time::Duration;
use openapiv3::OpenAPI;

use crate::database::save_user;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientData {
	#[serde(rename = "User")]
	pub user: User,
	#[serde(rename = "SessionInfo")]
	pub session_info: SessionInfo,
	#[serde(rename = "AccessToken")]
	pub access_token: Option<String>,
	#[serde(rename = "ServerId")]
	pub server_id: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	#[serde(rename = "Name")]
	pub name: Option<String>,
	#[serde(rename = "ServerId")]
	server_id: Option<String>,
	#[serde(rename = "ServerName")]
	server_name: Option<String>,
	#[serde(rename = "Id")]
	id: String,
	#[serde(rename = "PrimaryImageTag")]
	primary_image_tag: Option<String>,
	#[serde(rename = "HasPassword")]
	has_password: bool,
	#[serde(rename = "HasConfiguredPassword")]
	has_configured_password: bool,
	#[serde(rename = "HasConfiguredEasyPassword")]
	has_configured_easy_password: bool,
	#[serde(rename = "EnableAutoLogin")]
	enable_auto_login: Option<bool>,
	#[serde(rename = "LastLoginDate")]
	last_login_date: Option<String>,
	#[serde(rename = "LastActivityDate")]
	last_activity_date: Option<String>,
	#[serde(rename = "Configuration")]
	configuration: Option<Configuration>,
	#[serde(rename = "Policy")]
	policy: Option<Policy>,
	primary_image_aspect_ratio: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    #[serde(rename = "AudioLanguagePreference")]
    pub audio_language_preference: Option<String>,
    #[serde(rename = "PlayDefaultAudioTrack")]
    pub play_default_audio_track: bool,
    #[serde(rename = "SubtitleLanguagePreference")]
    pub subtitle_language_preference: Option<String>,
    #[serde(rename = "DisplayMissingEpisodes")]
    pub display_missing_episodes: bool,
    #[serde(rename = "GroupedFolders")]
    pub grouped_folders: Vec<String>,
    #[serde(rename = "SubtitleMode")]
    pub subtitle_mode: SubtitleMode,
    #[serde(rename = "DisplayCollectionsView")]
    pub display_collections_view: bool,
    #[serde(rename = "EnableLocalPassword")]
    pub enable_local_password: bool,
    #[serde(rename = "OrderedViews")]
    pub ordered_views: Vec<String>,
    #[serde(rename = "LatestItemsExcludes")]
    pub latest_items_excludes: Vec<String>,
    #[serde(rename = "MyMediaExcludes")]
    pub my_media_excludes: Vec<String>,
    #[serde(rename = "HidePlayedInLatest")]
    pub hide_played_in_latest: bool,
    #[serde(rename = "RememberAudioSelections")]
    pub remember_audio_selections: bool,
    #[serde(rename = "RememberSubtitleSelections")]
    pub remember_subtitle_selections: bool,
    #[serde(rename = "EnableNextEpisodeAutoPlay")]
    pub enable_next_episode_auto_play: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubtitleMode {
	#[default]
	Default,
	Always,
	OnlyForced,
	None,
	Smart
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    #[serde(rename = "IsAdministrator")]
    pub is_administrator: bool,
    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,
    #[serde(rename = "IsDisabled")]
    pub is_disabled: bool,
    #[serde(rename = "BlockedTags")]
    pub blocked_tags: Vec<String>,
    #[serde(rename = "EnableUserPreferenceAccess")]
    pub enable_user_preference_access: bool,
	#[serde(rename = "MaxParentalRating")]
	max_parental_rating: Option<i32>,
    #[serde(rename = "AccessSchedules")]
    pub access_schedules: Vec<String>,
    #[serde(rename = "BlockUnratedItems")]
    pub block_unrated_items: Vec<String>,
    #[serde(rename = "EnableRemoteControlOfOtherUsers")]
    pub enable_remote_control_of_other_users: bool,
    #[serde(rename = "EnableSharedDeviceControl")]
    pub enable_shared_device_control: bool,
    #[serde(rename = "EnableRemoteAccess")]
    pub enable_remote_access: bool,
    #[serde(rename = "EnableLiveTvManagement")]
    pub enable_live_tv_management: bool,
    #[serde(rename = "EnableLiveTvAccess")]
    pub enable_live_tv_access: bool,
    #[serde(rename = "EnableMediaPlayback")]
    pub enable_media_playback: bool,
    #[serde(rename = "EnableAudioPlaybackTranscoding")]
    pub enable_audio_playback_transcoding: bool,
    #[serde(rename = "EnableVideoPlaybackTranscoding")]
    pub enable_video_playback_transcoding: bool,
    #[serde(rename = "EnablePlaybackRemuxing")]
    pub enable_playback_remuxing: bool,
    #[serde(rename = "ForceRemoteSourceTranscoding")]
    pub force_remote_source_transcoding: bool,
    #[serde(rename = "EnableContentDeletion")]
    pub enable_content_deletion: bool,
    #[serde(rename = "EnableContentDeletionFromFolders")]
    pub enable_content_deletion_from_folders: Vec<String>,
    #[serde(rename = "EnableContentDownloading")]
    pub enable_content_downloading: bool,
    #[serde(rename = "EnableSyncTranscoding")]
    pub enable_sync_transcoding: bool,
    #[serde(rename = "EnableMediaConversion")]
    pub enable_media_conversion: bool,
    #[serde(rename = "EnabledDevices")]
    pub enabled_devices: Option<Vec<String>>,
    #[serde(rename = "EnableAllDevices")]
    pub enable_all_devices: bool,
    #[serde(rename = "EnabledChannels")]
    pub enabled_channels: Option<Vec<String>>,
    #[serde(rename = "EnableAllChannels")]
    pub enable_all_channels: bool,
    #[serde(rename = "EnabledFolders")]
    pub enabled_folders: Option<Vec<String>>,
    #[serde(rename = "EnableAllFolders")]
    pub enable_all_folders: bool,
    #[serde(rename = "InvalidLoginAttemptCount")]
    pub invalid_login_attempt_count: i64,
    #[serde(rename = "LoginAttemptsBeforeLockout")]
    pub login_attempts_before_lockout: i64,
    #[serde(rename = "MaxActiveSessions")]
    pub max_active_sessions: i64,
    #[serde(rename = "EnablePublicSharing")]
    pub enable_public_sharing: bool,
    #[serde(rename = "BlockedMediaFolders")]
    pub blocked_media_folders: Option<Vec<String>>,
    #[serde(rename = "BlockedChannels")]
    pub blocked_channels: Option<Vec<String>>,
    #[serde(rename = "RemoteClientBitrateLimit")]
    pub remote_client_bitrate_limit: i64,
    #[serde(rename = "AuthenticationProviderId")]
    pub authentication_provider_id: Option<String>,
    #[serde(rename = "PasswordResetProviderId")]
    pub password_reset_provider_id: Option<String>,
    #[serde(rename = "SyncPlayAccess")]
    pub sync_play_access: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AccessSchedule {
	id: i32,
	user_id: String,
	day_of_week: DayOfWeek,
	start_hour: f64,
	end_hour: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
enum DayOfWeek {
	#[default]
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
enum UnrelatedItem {
	Movie,
	Trailer,
	Series,
	Music,
	Book,
	LiveTvChannel,
	LiveTvProgram,
	ChannelContent,
	#[default]
	Other,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
enum SyncPlayAccess {
	CreateAndJoinGroups,
	JoinGroups,
	#[default]
	None
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
	#[serde(rename = "PlayState")]
	play_state: Option<PlayState>,
	#[serde(rename = "AdditionalUsers")]
	additional_users: Vec<AdditionalUser>,
	#[serde(rename = "Capabilities")]
	capabilities: Option<Capabilities>,
	#[serde(rename = "RemoteEndPoint")]
	remote_end_point: Option<String>,
	#[serde(rename = "PlayableMediaTypes")]
	playable_media_types: Option<Vec<String>>,
	#[serde(rename = "Id")]
	id: Option<String>,
	#[serde(rename = "UserId")]
	user_id: String,
	#[serde(rename = "UserName")]
	user_name: Option<String>,
	#[serde(rename = "Client")]
	client: Option<String>,
	#[serde(rename = "LastActivityDate")]
	last_activity_date: String,
	#[serde(rename = "LastPlaybackCheckIn")]
	last_playback_check_in: String,
	#[serde(rename = "DeviceName")]
	device_name: Option<String>,
	device_type: Option<String>,
	now_playing_item: Option<BaseItemDto>,
	full_now_playing_item: Option<BaseItem>,
	now_viewing_item: Option<BaseItemDto>,
	#[serde(rename = "DeviceId")]
	device_id: Option<String>,
	#[serde(rename = "ApplicationVersion")]
	application_version: Option<String>,
	transcoding_info: Option<TranscodingInfo>,
	#[serde(rename = "IsActive")]
	is_active: bool,
	#[serde(rename = "SupportsMediaControl")]
	supports_media_control: bool,
	#[serde(rename = "SupportsRemoteControl")]
	supports_remote_control: bool,
	#[serde(rename = "NowPlayingQueue")]
	now_playing_queue: Option<Vec<QueueItem>>,
	#[serde(rename = "NowPlayingQueueFullItems")]
	now_playing_queue_full_items: Option<BaseItemDto>,
	#[serde(rename = "HasCustomDeviceName")]
	has_custom_device_name: bool,
	playlist_item_id: Option<String>,
	#[serde(rename = "ServerId")]
	server_id: Option<String>,
	user_primary_image_tag: Option<String>,
	#[serde(rename = "SupportedCommands")]
	supported_commands: Option<Vec<GeneralCommandType>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayState {
    #[serde(rename = "CanSeek")]
    pub can_seek: bool,
    #[serde(rename = "IsPaused")]
    pub is_paused: bool,
    #[serde(rename = "IsMuted")]
    pub is_muted: bool,
    #[serde(rename = "RepeatMode")]
    pub repeat_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Capabilities {
	#[serde(rename = "PlayableMediaTypes")]
	playable_media_types: Option<Vec<String>>,
	#[serde(rename = "SupportedCommands")]
	supported_commands: Option<Vec<GeneralCommandType>>,
	#[serde(rename = "SupportsMediaControl")]
	supports_media_control: bool,
	#[serde(rename = "SupportsContentUploading")]
	supports_content_uploading: bool,
	message_callback_url: Option<String>,
	#[serde(rename = "SupportsSync")]
	supports_sync: bool,
	#[serde(rename = "SupportsPersistentIdentifier")]
	supports_persistent_identifier: bool,
	device_profile: Option<DeviceProfile>,
	app_store_url: Option<String>,
	icon_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BaseItem {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BaseItemDto {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TranscodingInfo {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct QueueItem {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DeviceProfile {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AdditionalUser {
	user_id: String,
	user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum GeneralCommandType {
	MoveUp,
	MoveDown,
	MoveLeft,
	MoveRight,
	PageUp,
	PageDown,
	PreviousLetter,
	NextLetter,
	ToggleOsd,
	ToggleContextMenu,
	Select,
	Back,
	TakeScreenshot,
	SendKey,
	SendString,
	GoHome,
	GoToSettings,
	VolumeUp,
	VolumeDown,
	Mute,
	Unmute,
	ToggleMute,
	SetVolume,
	SetAudioStreamIndex,
	SetSubtitleStreamIndex,
	ToggleFullscreen,
	DisplayContent,
	GoToSearch,
	DisplayMessage,
	SetRepeatMode,
	ChannelUp,
	ChannelDown,
	Guide,
	ToggleStats,
	PlayMediaSource,
	PlayTrailers,
	SetShuffleQueue,
	PlayState,
	PlayNext,
	ToggleOsdMenu,
	Play,
	SetMaxStreamingBitrate
}

pub struct Jellyfin {

}

impl Jellyfin {
	pub async fn authenticate_user(username: &str, password: &str) -> Result<String, String> {
		println!("Attempting to authenticate user {} with password {}", username, password);
		let jellyfin_server_url: &str  = "https://jellyfin.ericwaetke.de";
		let request_url = format!("{}{}", jellyfin_server_url, "/users/AuthenticateByName");
	
	
		let mut body = HashMap::new();
		body.insert("Username", username);
		body.insert("Pw", password);
	
		let mut headers = header::HeaderMap::new();
		headers.insert("accept", header::HeaderValue::from_static("application/json"));
		headers.insert("content-type", header::HeaderValue::from_static("application/json"));
		headers.insert("X-Application", header::HeaderValue::from_static("Rustyfin/0.0.1"));
		headers.insert("Accept-Charset", header::HeaderValue::from_static("UTF-8,*"));
		headers.insert("Accept-encoding", header::HeaderValue::from_static("gzip"));
		headers.insert("User-Agent", header::HeaderValue::from_static("Rustyfin/0.0.1"));
		headers.insert("X-Emby-Authorization", header::HeaderValue::from_static("MediaBrowser Client='Rustyfin', Device='Windows', DeviceId='1', Version='0.0.1'"));
	
		let client = reqwest::Client::new();
		let req = client
			.post(request_url)
			.json(&body)
			.headers(headers)
			.timeout(Duration::from_secs(3))
			.send()
			.await;

		match req {
			Ok(res) => {


				match res.status() {
					reqwest::StatusCode::OK => {
						println!("OK");
						match res.json::<ClientData>().await {
							Ok(res) => {
								println!("res: {:?}", res);
								save_user(&res);
								Ok("User authenticated".to_string())
							},
							Err(err) => {
								println!("err: {:?}", err);
								Err("Error".to_string())
							}
						}
					},
					reqwest::StatusCode::UNAUTHORIZED => {
						println!("UNAUTHORIZED");
						return Err("Unauthorized".to_string());
					},
					_ => {
						println!("Other: {:?}", res.status());
						return Err("Other".to_string());
					}
				}


				// let res = res.text().await;
				// match res {
				// 	Ok(res) => {
				// 		println!("res: {:?}", res);
				// 		Ok(res)
				// 	},
				// 	Err(err) => {
				// 		println!("err: {:?}", err);
				// 		Err("Error".to_string())
				// 	}
				// }
			},
			Err(err) => {
				println!("err: {:?}", err);
				Err("Error".to_string())
			}
		}
	}
}



async fn get_jellyfin_server_info() -> Result<String, Box<dyn Error>> {
	let jellyfin_server_url: &str  = "https://jellyfin.ericwaetke.de";
	let request_url = format!("{}{}", jellyfin_server_url, "/System/Info/Public");

	let client = reqwest::Client::new();
	let res = client
		.get(request_url)
		.header("Accept", "application/json")
		.timeout(Duration::from_secs(3))
		.send()
		.await?
		.text()
		.await?;
	println!("{:}", res);
	Ok(res)
}
