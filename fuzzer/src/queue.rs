use forksrv::exitreason::ExitReason;
use grammartec::context::Context;
use grammartec::recursion_info::RecursionInfo;
use grammartec::tree::Tree;
use grammartec::tree::TreeLike;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

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
    pub all_bits: Vec<u8>,
    pub exitreason: ExitReason,
    pub state: InputState,
    pub recursions: Option<Vec<RecursionInfo>>,
    pub execution_time: u32,
}

impl QueueItem {
    pub fn new(
        id: usize,
        tree: Tree,
        fresh_bits: HashSet<usize>,
        all_bits: Vec<u8>,
        exitreason: ExitReason,
        execution_time: u32,
    ) -> Self {
        QueueItem {
            id,
            tree,
            fresh_bits,
            all_bits,
            exitreason,
            state: InputState::Init(0),
            recursions: None,
            execution_time,
        }
    }
}

pub struct Queue {
    pub processed: Vec<QueueItem>,
    pub bit_to_inputs: HashMap<usize, Vec<usize>>,
    pub current_id: usize,
    pub work_dir: String,
    pub con: redis::Client,
    pub list_key: String,
}

impl Queue {
    pub fn add(
        &mut self,
        tree: Tree,
        all_bits: Vec<u8>,
        exitreason: ExitReason,
        ctx: &Context,
        execution_time: u32,
    ) -> String {
        if all_bits
            .iter()
            .enumerate()
            .all(|(i, elem)| (*elem == 0) || self.bit_to_inputs.contains_key(&i))
        {
            return String::new();
        }
        let mut fresh_bits = HashSet::new();
        //Check which bits are new and insert them into fresh_bits
        for (i, elem) in all_bits.iter().enumerate() {
            if *elem != 0 {
                if !self.bit_to_inputs.contains_key(&i) {
                    fresh_bits.insert(i);
                }
                self.bit_to_inputs
                    .entry(i)
                    .or_default()
                    .push(self.current_id);
            }
        }

        let file_name = format!(
            "{}/outputs/queue/id:{:09},er:{exitreason:?}",
            self.work_dir, self.current_id
        );
        //Create File for entry
        let mut file = File::create(&file_name).expect("file create error");
        tree.unparse_to(ctx, &mut file);

        let inp = QueueItem::new(
            self.current_id,
            tree,
            fresh_bits,
            all_bits,
            exitreason,
            execution_time,
        );

        // Add entry to queue
        self.add_to_redis(&inp);

        //Increase current_id
        if self.current_id == usize::max_value() {
            self.current_id = 0;
        } else {
            self.current_id += 1;
        };
        file_name
    }

    pub fn new(work_dir: String, redis_addr: String) -> Self {
        Queue {
            processed: vec![],
            bit_to_inputs: HashMap::new(),
            current_id: 0,
            work_dir,
            con: redis::Client::open(redis_addr).unwrap(),
            list_key: String::from("my_list_data"),
        }
    }

    pub fn pop(&mut self) -> Option<QueueItem> {
        let option = self.pop_from_redis();
        if let Some(item) = option {
            let id = item.id;
            let keys: Vec<_> = self.bit_to_inputs.keys().cloned().collect();

            for k in keys {
                let mut v = self.bit_to_inputs.remove(&k).expect("RAND_2593710501");
                v.retain(|&x| x != id);
                if !v.is_empty() {
                    self.bit_to_inputs.insert(k, v);
                }
            }
            return Some(item);
        }
        None
    }

    pub fn finished(&mut self, item: QueueItem) {
        if item
            .all_bits
            .iter()
            .enumerate()
            .all(|(i, elem)| (*elem == 0) || self.bit_to_inputs.contains_key(&i))
        {
            //If file was created for this entry, delete it.
            match fs::remove_file(format!(
                "{}/outputs/queue/id:{:09},er:{:?}",
                self.work_dir, item.id, item.exitreason
            )) {
                Err(ref err) if err.kind() != ErrorKind::NotFound => {
                    println!("Error while deleting file: {err}");
                }
                _ => {}
            }
            return;
        }

        //Check which bits are new and insert them into fresh_bits
        let mut fresh_bits = HashSet::new();
        for (i, elem) in item.all_bits.iter().enumerate() {
            if *elem != 0 {
                if !self.bit_to_inputs.contains_key(&i) {
                    fresh_bits.insert(i);
                }
                self.bit_to_inputs.entry(i).or_default().push(item.id);
            }
        }
        self.processed.push(item);
    }

    pub fn len(&mut self) -> usize {
        self.con.llen(&self.list_key).unwrap()
    }

    fn push_to_list<T: redis::ToRedisArgs>(&mut self, items: &[T]) -> redis::RedisResult<()> {
        self.con.rpush(&self.list_key, items)
    }

    pub fn new_round(&mut self) {
        let json_vec: Vec<_> = self
            .processed
            .iter()
            .map(|item| serde_json::to_string(&item).unwrap())
            .collect();

        self.push_to_list(&json_vec);
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
