
use mongodb::{bson::{doc, Document, oid::ObjectId}, Collection};
use futures::stream::StreamExt;
use async_trait::async_trait;


use crate::modules::pandas::schema::Panda;
use crate::modules::pandas::dto::UpdatePandaDto;

#[async_trait]
pub trait PandaRepository {
    async fn create_panda(&mut self, panda: Panda) -> Panda;
    async fn get_pandas(&mut self) -> Vec<Panda>;
    async fn get_panda(&mut self, id: &str) -> Option<Panda>;
    async fn update_panda(&mut self, id: &str, dto: UpdatePandaDto) -> Option<Panda>;
    async fn delete_panda(&mut self, id: &str) -> bool;
}

pub struct MongoPandaRepository {
    db: mongodb::Database,
}

impl MongoPandaRepository {
    pub fn new(db: mongodb::Database) -> Self {
        MongoPandaRepository { db }
    }
}

#[async_trait]
impl PandaRepository for MongoPandaRepository {
    async fn create_panda(&mut self, panda: Panda) -> Panda {
        let collection = self.db.collection("pandas");
        let mut panda = panda;
        match collection.insert_one(panda.clone(), None).await {
            Ok(result) => {
                panda.id = result.inserted_id.as_object_id().map(|oid| oid.to_hex());
                panda
            },
            Err(_) => panic!("Error inserting panda"),
        }
    }

    async fn get_pandas(&mut self) -> Vec<Panda> {
        let collection: Collection<Document> = self.db.collection("pandas");
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

     async fn get_panda(&mut self, id: &str) -> Option<Panda> {
        let collection: Collection<Document> = self.db.collection("pandas");

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

     async fn update_panda(&mut self, id: &str, dto: UpdatePandaDto) -> Option<Panda> {
        let collection: Collection<Document> = self.db.collection("pandas");

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

     async fn delete_panda(&mut self, id: &str) -> bool {
        let collection: Collection<Document> = self.db.collection("pandas");

        if let Ok(object_id) = ObjectId::parse_str(id) {
            match collection.delete_one(doc! {"_id": object_id}, None).await {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}



