extern crate diesel;
use mongodb::{bson::{doc, oid::ObjectId}, results::DeleteResult};
pub mod models;

use crate::models::{ContractJson, EventsModel};
use mongodb::{
    bson::extjson::de::Error,
    results::{InsertManyResult, InsertOneResult},
    Client, Collection,
};

use futures::stream::StreamExt;
// use futures::stream::{StreamExt, TryStreamExt};

#[derive(Clone)]
pub struct MongoRepo {
    events: Collection<EventsModel>,
}

impl MongoRepo {
    pub async fn init(uri: &str, database: &str, collection: Vec<&str>) -> Self {
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(&database);
        let events: Collection<EventsModel> = db.collection::<EventsModel>(&collection[0]);
        MongoRepo { events }
    }

    pub async fn insert_event(
        collection: &Self,
        event: EventsModel,
    ) -> Result<InsertOneResult, Error> {
        let result = collection
            .events
            .insert_one(event, None)
            .await
            .ok()
            .expect("Error creating event");

        Ok(result)
    }

    pub async fn get_event_data(collection: &Self) -> Result<Vec<EventsModel>, Error> {
        let mut cursor = collection
            .events
            .find(None, None)
            .await
            .ok()
            .expect("Error getting events");

        let mut data: Vec<EventsModel> = Vec::new();

        while let Some(doc) = cursor.next().await {
            data.push(doc.unwrap());
        }

        Ok(data)
    }

}

#[cfg(test)]
mod tests {

    use crate::{
        models::{ContractJson, EventsModel},
        MongoRepo,
    };

    async fn get_connection() -> MongoRepo {
        let db_url = "mongodb://tango:tango@localhost:27017";
        let mut collections = Vec::new();
        collections.push("events");
        collections.push("contracts");
        let connector = MongoRepo::init(&db_url, "tango_db", collections).await;

        connector
    }

    #[tokio::test]

    async fn connecting_test() {
        let _connector = get_connection().await;
    }

    #[tokio::test]
    async fn insert_events() {
        let mut topic:Vec<H256>  = Vec::new();
        let s = H256::zero();
        topic.push(s);
        let address = H160::zero();
    
        let input = EventsModel::new(
            address,
            topic,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
    
        let _connector = get_connection().await;
        let _result = MongoRepo::insert_event(&_connector, input).await.unwrap();
    }

    #[tokio::test]
    async fn retrive_events() {
        let _connector = get_connection().await;
        let _result = MongoRepo::get_event_data(&_connector).await.unwrap();
    }


}
