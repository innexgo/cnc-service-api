// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::AsRefStr;

// creates new api key
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyNewValidProps {
  pub user_email: String,
  pub user_password: String,
  pub duration: i64,
}

// cancels your existing api key
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyNewCancelProps {
  pub api_key_to_cancel: String,
  pub api_key: String,
}

// this sends an email to your account, click on it to change your email
// this sends an email to the parent_email specified, parent can authorize it to give you a permission
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationChallengeNewProps {
  pub email: String,
  pub to_parent: bool,
  pub api_key: String,
}

// this actually changes your email
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailNewProps {
  pub verification_challenge_key: String,
}

// parent will get key from email. once this call is made, then the student will be authorized
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPermissionNewProps {
  pub verification_challenge_key: String,
}

// this initially sets up your account, but you will need to verify your email before you can do tasks
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserNewProps {
  pub user_name: String,
  pub user_email: String,
  pub user_password: String,
  pub parent_email: Option<String>,
}

// lets you change your name
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDataNewProps {
  pub user_name: String,
  pub api_key: String,
}

// reset password
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetNewProps {
  pub user_email: String,
}

// change your password
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordNewChangeProps {
  pub old_password: String,
  pub new_password: String,
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordNewCancelProps {
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordNewResetProps {
  pub password_reset_key: String,
  pub new_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserViewProps {
  pub user_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDataViewProps {
  pub user_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub only_recent:bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationChallengeViewProps {
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub to_parent: Option<bool>,
  pub email: Option<Vec<String>>,
  pub api_key: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailViewProps {
  pub email_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub email: Option<Vec<String>>,
  pub only_recent:bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPermissionViewProps {
  pub parent_permission_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub user_id: Option<Vec<i64>>,
  pub from_challenge: Option<bool>,
  pub only_recent:bool,
  pub parent_email: Option<Vec<String>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordViewProps {
  pub password_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub from_reset: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiKeyKind {
  Valid,
  Cancel,
}

impl TryFrom<u8> for ApiKeyKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<ApiKeyKind, u8> {
    match val {
      x if x == ApiKeyKind::Valid as u8 => Ok(ApiKeyKind::Valid),
      x if x == ApiKeyKind::Cancel as u8 => Ok(ApiKeyKind::Cancel),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyViewProps {
  pub api_key_id: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub min_duration: Option<i64>,
  pub max_duration: Option<i64>,
  pub api_key_kind: Option<ApiKeyKind>,
  pub only_recent: bool,
  pub api_key: String,
}

// Private Api
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserByIdProps {
  pub user_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserByApiKeyIfValid {
  pub api_key: String,
}
