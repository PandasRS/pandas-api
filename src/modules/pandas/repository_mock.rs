use async_trait::async_trait;
use crate::modules::pandas::dto::UpdatePandaDto;
use crate::modules::pandas::schema::Panda;
use std::collections::HashMap;

use super::repository::PandaRepository;


pub struct MockPandaRepository {
    pub mock_pandas: HashMap<String, Panda>,
}

impl MockPandaRepository {
    pub fn new() -> Self {
        MockPandaRepository {
            mock_pandas: HashMap::new(),
        }
    }

    pub fn insert_mock_panda(&mut self, id: &str, name: &str, age: i32) {
        let panda = Panda {
            id: Some(id.to_string()),
            name: name.to_string(),
            age,
        };
        self.mock_pandas.insert(id.to_string(), panda);
    }
}

#[async_trait]
impl PandaRepository for MockPandaRepository {
    async fn create_panda(&mut self, mut panda: Panda) -> Panda {
        let new_id = "new_id"; 
        panda.id = Some(new_id.to_string());
        panda
    }

    async fn get_pandas(&mut self) -> Vec<Panda> {
        self.mock_pandas.values().cloned().collect()
    }

    async fn get_panda(&mut self, id: &str) -> Option<Panda> {
        self.mock_pandas.get(id).cloned()
    }

    async fn update_panda(&mut self, id: &str, dto: UpdatePandaDto) -> Option<Panda> {
        if let Some(panda) = self.mock_pandas.get_mut(id) {
            if let Some(name) = dto.name {
                panda.name = name;
            }
            if let Some(age) = dto.age {
                panda.age = age;
            }
            return Some(panda.clone());
        }
        None
    }

    async fn delete_panda(&mut self, id: &str) -> bool {
        self.mock_pandas.remove(id).is_some()
    }
}
