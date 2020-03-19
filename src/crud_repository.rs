use bson::{doc, Bson::Document as BsonDocument, Document};
use log::info;
use mongodb::error::Result as MongoResult;
use mongodb::options::{UpdateModifications, UpdateOptions, DeleteOptions};
use mongodb::results::InsertOneResult;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub fn find_one<T>(
    filter_document: Document,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Box<dyn std::error::Error + Send + Sync + 'static>>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    info!("find_one_by_query");
    let coll = db.collection(collection_name);
    let result = coll.find_one(filter_document, None)?;
    if let Some(document) = result {
        let t = bson::from_bson::<T>(BsonDocument(document))?;
        return Ok(Some(t));
    } else {
        return Ok(None);
    }
}

pub fn _find_one_by_field<T>(
    field_name: String,
    value: String,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Box<dyn std::error::Error + Send + Sync + 'static>>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    self::find_one(doc! {field_name: value}, collection_name, db)
}

pub fn add<T>(t: &T, collection_name: &str, db: &Database) -> MongoResult<InsertOneResult>
where
    for<'a> T: Debug + Serialize + Deserialize<'a>,
{
    let serialized_item = bson::to_bson(&t)?;

    if let BsonDocument(document) = serialized_item {
        let coll = db.collection(collection_name);
        return coll.insert_one(document, None);
    } else {
        panic!("Error converting the BSON object into a MongoDB document");
    }
}

pub fn update_one(
    query: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
    collection_name: &str,
    db: &Database,
) {
    let coll = db.collection(collection_name);
    let _result = coll.update_one(query, update, options).unwrap();
}


pub fn delete_one(
    query: Document,
    options: impl Into<Option<DeleteOptions>>,
    collection_name: &str,
    db: &Database,
) {
    let coll = db.collection(collection_name);
    let _result = coll.delete_one(query, options).unwrap();
}
