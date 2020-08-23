use crate::{error::Error::*, Result};
use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{options::ClientOptions, options::UpdateOptions, Client, Collection};

const DB_NAME: &str = "test-req";
const APPS_COL: &str = "apps";
const INTERACTIONS_COL: &str = "interactions";
const MESSAGES_COL: &str = "messages";

#[derive(Debug)]
pub struct App {
  pub name: String,
  pub phone: String,
}

#[derive(Debug)]
pub struct Interaction {
  pub app_name: String,
  pub to: String,
  pub last_message: Message,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Message {
  pub id: ObjectId,
  pub app_name: String,
  pub text: String,
  pub to: String,
  pub from: String,
  pub status: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct DB {
  pub client: Client,
}

impl DB {
  pub async fn init() -> Result<Self> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    Ok(Self {
      client: Client::with_options(client_options)?,
    })
  }

  pub async fn find_app_by_name(&self, app_name: &str) -> Result<App> {
    let doc = self
      .col_apps()
      .find_one(
        doc! {
            "name": app_name
        },
        None,
      )
      .await?
      .unwrap();

    let app = App {
      name: doc.get_str("name")?.to_owned(),
      phone: doc.get_str("phone")?.to_owned(),
    };

    Ok(app)
  }

  pub async fn save_message(&self, app: &App, to: &str, text: &str) -> Result<Message> {
    let now = Utc::now();
    let status = String::from("stored");

    let insert_result = self
      .col_messages()
      .insert_one(
        doc! {
          "appName": &app.name,
          "from": &app.phone,
          "to": &to,
          "status": &status,
          "text": &text,
          "createdAt": &now,
          "updatedAt": &now
        },
        None,
      )
      .await?;

    let message = Message {
      id: insert_result.inserted_id.as_object_id().unwrap().clone(),
      app_name: app.name.to_owned(),
      from: app.phone.to_owned(),
      to: to.to_owned(),
      status: status,
      text: text.to_owned(),
      created_at: now,
      updated_at: now,
    };

    Ok(message)
  }

  pub async fn set_interaction_last_message(&self, message: &Message) -> Result<()> {
    let update_options = UpdateOptions::builder().upsert(true).build();
    self
      .col_interactions()
      .update_one(
        doc! { "appName": &message.app_name, "to": &message.to },
        doc! {
          "$set": {
            "lastMessage": {
                "_id": &message.id,
                "appName": &message.app_name,
                "from": &message.from,
                "to": &message.to,
                "status": &message.status,
                "text": &message.text,
                "createdAt": &message.created_at,
                "updatedAt": &message.updated_at,
            },
            "updatedAt": &message.updated_at
          },
          "$setOnInsert": { "createdAt": &message.created_at },
        },
        update_options,
      )
      .await
      .map_err(MongoQueryError)?;

    Ok(())
  }

  fn col_apps(&self) -> Collection {
    self.client.database(DB_NAME).collection(APPS_COL)
  }

  fn col_interactions(&self) -> Collection {
    self.client.database(DB_NAME).collection(INTERACTIONS_COL)
  }

  fn col_messages(&self) -> Collection {
    self.client.database(DB_NAME).collection(MESSAGES_COL)
  }
}
