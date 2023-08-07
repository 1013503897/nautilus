use grammartec::recursion_info::RecursionInfo;
use grammartec::tree::Tree;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Clone, Deserialize)]
pub enum InputState {
    Init(usize),
    Det((usize, usize)),
    Random,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct QueueItem {
    pub id: usize,
    pub tree: Tree,
    pub fresh_bits: HashSet<usize>,
    pub state: InputState,
    pub recursions: Option<Vec<RecursionInfo>>,
}

impl QueueItem {
    pub fn new(id: usize, tree: Tree, fresh_bits: HashSet<usize>) -> Self {
        QueueItem {
            id,
            tree,
            fresh_bits,
            state: InputState::Init(0),
            recursions: None,
        }
    }
}

pub struct Queue {
    pub current_id: usize,
    pub work_dir: String,
    pub con: redis::Client,
    pub list_key: String,
}

impl Queue {
    pub fn gen_item(&mut self, tree: Tree, new_bits: &Vec<usize>) -> QueueItem {
        let fresh_bits: HashSet<usize> = HashSet::from_iter(new_bits.iter().cloned());
        QueueItem::new(self.current_id, tree, fresh_bits)
    }

    pub fn add_item(&mut self, item: QueueItem) {
        // Add entry to queue
        self.add_to_redis(&item).expect("Failed to add to redis");

        //Increase current_id
        if self.current_id == usize::max_value() {
            self.current_id = 0;
        } else {
            self.current_id += 1;
        };
    }

    pub fn unparse_to_file(&mut self, code: &[u8]) -> String {
        let file_name = format!("{}/outputs/queue/id:{:09}", self.work_dir, self.current_id);
        let mut file = File::create(&file_name).unwrap();
        if file.write_all(code).is_err() {
            warn!("Failed to write to file {}", file_name);
        }
        file_name
    }

    pub fn new(work_dir: String, redis_addr: String) -> Self {
        Queue {
            // processed: vec![],
            current_id: 0,
            work_dir,
            con: redis::Client::open(redis_addr).unwrap(),
            list_key: std::env::var("HOSTNAME").unwrap_or("unknown".to_string()),
        }
    }

    pub fn pop(&mut self) -> Option<QueueItem> {
        let option = self.pop_from_redis();
        if let Some(item) = option {
            return Some(item);
        }
        None
    }

    pub fn add_to_redis(&mut self, item: &QueueItem) -> redis::RedisResult<()> {
        let json_data = serde_json::to_string(&item).unwrap();
        self.con.rpush(&self.list_key, json_data)
    }

    pub fn pop_from_redis(&mut self) -> Option<QueueItem> {
        let json_data: Result<String, _> = self.con.lpop(&self.list_key, None);
        match json_data {
            Ok(data) => match serde_json::from_str::<QueueItem>(&data) {
                Ok(item) => Some(item),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}
