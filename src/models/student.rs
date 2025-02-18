use crate::data_collection::DataCollection;
use crate::enums::difficulty::Difficulty;
use crate::traits::collect::Collect;
use crate::traits::gen_data_id::GenDataId;
#[derive(Debug, Clone)]
pub struct Student {
    id: u32,
    name: String,
}
impl Student {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            id: 0,
        }
    }

    pub fn from(name: String) -> Self {
        Self { name, id: 0 }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

}

impl Collect for Student {
    fn collect() -> Self {
        let name = DataCollection::input("Enter student name:");
        
        Self::from(name)
    }
}
impl GenDataId<u32> for Student {
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    
    fn get_id(&self) -> u32 {
        self.id
    }
}