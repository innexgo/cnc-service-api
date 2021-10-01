use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthError {
  NoCapability,
  ApiKeyUnauthorized,
  NoPermission,
  PasswordIncorrect,
  PasswordInsecure,
  PasswordCannotCreateForOthers,
  UserNonexistent,
  UserDataNonexistent,
  ApiKeyNonexistent,
  UserExistent,
  UserNameEmpty,
  UserEmailEmpty,
  UserEmailInvalidated,
  NegativeDuration,
  CannotAlterPast,
  VerificationChallengeNonexistent,
  VerificationChallengeTimedOut,
  VerificationChallengeUsed,
  VerificationChallengeWrongKind,
  ParentPermissionNonexistent,
  ParentPermissionExistent,
  PasswordExistent,
  PasswordNonexistent,
  EmailExistent,
  EmailNonexistent,
  PasswordResetNonexistent,
  PasswordResetTimedOut,
  EmailBounced,
  EmailUnknown,
  DecodeError,
  InternalServerError,
  MethodNotAllowed,
  BadRequest,
  NotFound,
  Unknown,
  Network,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationChallenge {
  pub creation_time: i64,
  pub to_parent: bool,
  pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub user_id: i64,
  pub creation_time: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub user_data_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
  pub email_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub verification_challenge: VerificationChallenge
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPermission {
  pub parent_permission_id: i64,
  pub creation_time: i64,
  pub user_id: i64,
  pub verification_challenge: Option<VerificationChallenge>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordReset {
  pub creation_time: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Password {
  pub password_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub password_reset: Option<PasswordReset>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "api_key_kind")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiKeyData {
  // the interior of the struct should be normal, but the VALID and CANCEL tags should be screaming case
  #[serde(rename_all = "camelCase")]
  Valid {
      key: Option<String>,
      duration: i64
  },
  Cancel
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
  pub api_key_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  #[serde(flatten)]
  pub api_key_data: ApiKeyData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
  pub service: String,
  pub version_major: i64,
  pub version_minor: i64,
  pub version_rev: i64,
}
