use crate::data_collection::DataCollection;
use crate::enums::difficulty::Difficulty;
use crate::traits::collect::Collect;
use crate::traits::gen_data_id::GenDataId;
#[derive(Debug,Clone)]
pub struct Topic {
    id: u32,
    title: String,
    difficulty: Difficulty,
}
impl Topic {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::from(""),
            difficulty: Difficulty::Easy,
        }
    }

   
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }
  
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty.clone()
    }
}
impl Collect for Topic {
    fn collect() -> Self {
        let mut topic = Self::new();
        //println!("Enter the topics. \n Type 'done' when completed.");

        topic.title = DataCollection::input("Eter topic titles");

        let difficulty = DataCollection::input("Enter topic difficulty");
        topic.difficulty = Difficulty::from(difficulty.as_str());
        topic
    }
}


impl GenDataId<u32> for Topic {
    fn set_id(&mut self, id: u32) {
        self.id = id+1;
        
    }
    
    fn get_id(&self) -> u32 {
        self.id
    }
}