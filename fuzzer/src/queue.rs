use forksrv::exitreason::ExitReason;
use grammartec::context::Context;
use grammartec::recursion_info::RecursionInfo;
use grammartec::tree::Tree;
use grammartec::tree::TreeLike;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;

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
    pub fn add(
        &mut self,
        tree: Tree,
        exitreason: ExitReason,
        ctx: &Context,
        new_bits: &Vec<usize>,
    ) -> String {
        let fresh_bits: HashSet<usize> = HashSet::from_iter(new_bits.iter().cloned());
        // Check which bits are new and insert them into fresh_bits

        let file_name = format!(
            "{}/outputs/queue/id:{:09},er:{exitreason:?}",
            self.work_dir, self.current_id
        );
        //Create File for entry
        let mut file = File::create(&file_name).expect("file create error");
        tree.unparse_to(ctx, &mut file);

        let inp = QueueItem::new(self.current_id, tree, fresh_bits);
        self.add_item(inp);
        file_name
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
