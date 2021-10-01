use reqwest::Client;

use super::request;
use super::response;

#[derive(Clone)]
pub struct AuthService {
  client: Client,
  auth_service_url: String,
}

impl AuthService {
  pub async fn new(auth_service_url: &str) -> Self {
    AuthService {
      auth_service_url: String::from(auth_service_url),
      client: Client::new(),
    }
  }

  // If the ApiKey is valid, returns the user it refers to.
  // If the ApiKey is invalid or the user doesn't exist, returns an error
  pub async fn get_user_by_api_key_if_valid(
    &self,
    api_key: String,
  ) -> Result<response::User, response::AuthError> {
    self
      .client
      .post(format!("{}/get_user_by_api_key_if_valid", self.auth_service_url))
      .json(&request::GetUserByApiKeyIfValid { api_key })
      .send()
      .await
      .map_err(|_| response::AuthError::Network)?
      .json()
      .await
      .map_err(|_| response::AuthError::DecodeError)?
  }

  // if the user_id is valid, returns a user.
  // if the user is invalid, returns an error
  pub async fn get_user_by_id(
    &self,
    user_id: i64,
  ) -> Result<response::User, response::AuthError> {
    self
      .client
      .post(format!("{}/get_user_by_id", self.auth_service_url))
      .json(&request::GetUserByIdProps { user_id })
      .send()
      .await
      .map_err(|_| response::AuthError::Network)?
      .json()
      .await
      .map_err(|_| response::AuthError::DecodeError)?
  }

  // fetches api information
  pub async fn info(
    &self,
  ) -> Result<Vec<response::Info>, response::AuthError> {
    self
      .client
      .post(format!("{}/public/info", self.auth_service_url))
      .send()
      .await
      .map_err(|_| response::AuthError::Network)?
      .json()
      .await
      .map_err(|_| response::AuthError::DecodeError)?
  }
}
