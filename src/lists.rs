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
  fn load(subscriber: Subscriber, lists: Vec<i32>) -> ListmonkCreateSubscriber {
    ListmonkCreateSubscriber {
      email: subscriber.email,
      name: subscriber.name,
      status: "enabled".to_string(),
      attribs: serde_json::value::Value::Null,
      lists,
      preconfirm_subscriptions: true,
    }
  }
}

impl MailingLists {
  pub async fn load(
    url: &Url,
    user: &str,
    password_file: &PathBuf,
    lists: &[i32],
  ) -> anyhow::Result<MailingLists> {
    let password = std::fs::read_to_string(password_file)?;

    Ok(MailingLists {
      url: url.join("/api/subscribers")?,
      user: user.to_owned(),
      password,
      lists: lists.to_owned(),
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

    let response = client
      .post(self.url.clone())
      .json(&ListmonkCreateSubscriber::load(
        new_subscriber,
        self.lists.clone(),
      ))
      .basic_auth(self.user.clone(), Some(self.password.clone()))
      .send()
      .await
      .map_err(|e| {
        error!("reqwest error while to sending to listmonk {:?}", e);
        MailingListsError::RequestError
      })?;

    if response.status() != reqwest::StatusCode::OK {
      Err(MailingListsError::ListmonkError)
    } else {
      Ok(())
    }
  }
}
