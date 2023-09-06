use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use tracing::error;
use url::Url;

use reqwest::Client;
//use reqwest::Error;

#[derive(Debug, Clone)]
pub(crate) struct MailingLists {
  url: Url,
  user: String,
  password: String,
  lists: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Subscriber {
  name: String,
  email: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ListmonkCreateSubscriber {
  email: String,
  name: String,
  status: String, // "enabled" or "disabled"
  lists: Vec<i32>,
  attribs: serde_json::value::Value,
  preconfirm_subscriptions: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ListmonkUser {
  id: i32,
  /*created_at: String,
  updated_at: String,
  uuid: String,
  email: String,
  name: String,
  attribs: serde_json::value::Value,
  status: String,
  lists: Vec<i32>*/
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ListmonkUserCreateResponse {
  data: ListmonkUser,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ListmonkAddSubscribers {
  ids: Vec<i32>,
  action: String,
  target_list_ids: Vec<i32>,
  status: String,
}

#[derive(Debug, Clone)]
pub enum MailingListsError {
  InvalidMailingListId,
  RequestError,
  ListmonkError,
}

impl fmt::Display for MailingListsError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MailingListsError::InvalidMailingListId => {
        write!(f, "Invalid Mailing List ID")
      }
      MailingListsError::ListmonkError => {
        write!(f, "Couldn't create subscriber in listmonk")
      }
      MailingListsError::RequestError => {
        write!(f, "Couldn't make the request to listmonk")
      }
    }
  }
}

impl Error for MailingListsError {}

impl Subscriber {
  pub fn valid_email(&self) -> bool {
    EmailAddress::is_valid(&self.email)
  }
}

impl ListmonkCreateSubscriber {
  fn load(subscriber: &Subscriber) -> ListmonkCreateSubscriber {
    ListmonkCreateSubscriber {
      email: subscriber.email.clone(),
      name: subscriber.name.clone(),
      status: "enabled".to_string(),
      attribs: serde_json::value::Value::Null,
      lists: vec![],
      preconfirm_subscriptions: true,
    }
  }
}

impl ListmonkAddSubscribers {
  fn load(subscriber: i32, desired_list: i32) -> ListmonkAddSubscribers {
    ListmonkAddSubscribers {
      ids: vec![subscriber],
      action: "add".to_string(),
      target_list_ids: vec![desired_list],
      status: "confirmed".to_string(),
    }
  }
}

impl MailingLists {
  pub async fn load(
    url: &Url,
    user: &str,
    password_file: &PathBuf,
    lists: &str,
  ) -> anyhow::Result<MailingLists> {
    let password = std::fs::read_to_string(password_file)?;

    Ok(MailingLists {
      url: url.clone(),
      user: user.to_owned(),
      password,
      lists: serde_json::from_str(lists)?,
    })
  }

  pub async fn submit_to_listmonk(
    &self,
    new_subscriber: Subscriber,
    desired_list: i32,
  ) -> Result<(), MailingListsError> {
    if !self.lists.contains(&desired_list) {
      return Err(MailingListsError::InvalidMailingListId);
    }

    let client = Client::new();

    let response_create = client
      .post(
        self
          .url
          .clone()
          .join("/api/subscribers")
          .expect("invalid url"),
      )
      .json(&ListmonkCreateSubscriber::load(&new_subscriber))
      .basic_auth(self.user.clone(), Some(self.password.clone()))
      .send()
      .await
      .map_err(|e| {
        error!("reqwest error while to sending to listmonk {:?}", e);
        MailingListsError::RequestError
      })?;

    if response_create.status() != reqwest::StatusCode::OK {
      return Err(MailingListsError::ListmonkError);
    }

    let json_body: ListmonkUserCreateResponse = match response_create.json().await {
      Ok(data) => data,
      Err(e) => {
        error!("invalid response {:?}", e);
        return Err(MailingListsError::ListmonkError);
      }
    };

    let mailing_list_add = client
      .put(
        self
          .url
          .clone()
          .join("/api/subscribers/lists")
          .expect("invalid url"),
      )
      .json(&ListmonkAddSubscribers::load(
        json_body.data.id,
        desired_list,
      ))
      .basic_auth(self.user.clone(), Some(self.password.clone()))
      .send()
      .await
      .map_err(|e| {
        error!("reqwest error while to sending to listmonk {:?}", e);
        MailingListsError::RequestError
      })?;

    if mailing_list_add.status() != reqwest::StatusCode::OK {
      Err(MailingListsError::ListmonkError)
    } else {
      Ok(())
    }
  }
}
