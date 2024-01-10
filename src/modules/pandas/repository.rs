// modules/pandas/repository.rs

use mongodb::{bson::{doc, Document, oid::ObjectId}, Collection};
use futures::stream::StreamExt;

use crate::modules::pandas::schema::Panda;
use crate::modules::pandas::dto::UpdatePandaDto;

pub async fn create_panda(panda: Panda, db: &mongodb::Database) -> Panda {
    let collection = db.collection("pandas");
    let mut panda = panda;
    match collection.insert_one(panda.clone(), None).await {
        Ok(result) => {
            panda.id = result.inserted_id.as_object_id().map(|oid| oid.to_hex());
            panda
        },
        Err(_) => panic!("Error inserting panda"),
    }
}

pub async fn get_pandas(db: &mongodb::Database) -> Vec<Panda> {
    let collection: Collection<Document> = db.collection("pandas");
    let mut cursor = collection.find(doc! {}, None).await.unwrap();
    let mut pandas: Vec<Panda> = Vec::new();
     while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let document_clone = document.clone(); 
                let mut panda: Panda = mongodb::bson::from_document(document_clone).unwrap();

                if let Some(oid) = document.get_object_id("_id").ok() {
                    panda.id = Some(oid.to_hex()); 
                }
                pandas.push(panda);
            },
            Err(_) => panic!("Error getting panda"),
        }
    }
    pandas
}

pub async fn get_panda(id: &str, db: &mongodb::Database) -> Option<Panda> {
    let collection: Collection<Document> = db.collection("pandas");

    if let Ok(object_id) = ObjectId::parse_str(id) {
        match collection.find_one(doc! {"_id": object_id}, None).await {
            Ok(document) => {
                document.map(|doc| {
                    let mut panda: Panda = mongodb::bson::from_document(doc).unwrap();
                    panda.id = Some(id.to_string());
                    panda
                })
            },
            Err(_) => panic!("Error getting panda"),
        }
    } else {
        None
    }
}

pub async fn update_panda(id: &str, dto: UpdatePandaDto, db: &mongodb::Database) -> Option<Panda> {
    let collection: Collection<Document> = db.collection("pandas");

    if let Ok(object_id) = ObjectId::parse_str(id) {
        let mut update_doc = doc! {};
        if let Some(name) = dto.name {
            update_doc.insert("name", name);
        }
        if let Some(age) = dto.age {
            update_doc.insert("age", age);
        }

        match collection.update_one(doc! {"_id": object_id}, doc! {"$set": update_doc}, None).await {
            Ok(_) => collection.find_one(doc! {"_id": object_id}, None).await.unwrap().map(|doc| {
                let mut panda: Panda = mongodb::bson::from_document(doc).unwrap();
                panda.id = Some(id.to_string());
                panda
            }),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub async fn delete_panda(id: &str, db: &mongodb::Database) -> bool {
    let collection: Collection<Document> = db.collection("pandas");

    if let Ok(object_id) = ObjectId::parse_str(id) {
        match collection.delete_one(doc! {"_id": object_id}, None).await {
            Ok(_) => true,
            Err(_) => false,
        }
    } else {
        false
    }
}