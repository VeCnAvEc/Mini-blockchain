extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use std::thread;
use std::time::Duration;

use indicatif::ProgressBar;

use self::sha2::{Sha256, Digest};
use std::{fmt::Write};

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain: Chain  = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 50.0,
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction {
            sender,
            receiver,
            amount,
        });
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };

        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty:u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self) -> bool {
        self.reward = self.reward / 2f32;
        println!("New reward : {}", self.reward);
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header: BlockHeader = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty
        };

        let reward_trans: Transaction = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone,
            amount: self.reward
        };

        let mut block: Block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone);
        Chain::proof_of_work(&mut BlockHeader);

        println!("last {:#?}", block);
        self.chain.push(block);
        true
    }
}

fn get_merkle(curr_trans: Vec<Transaction>) -> String {
    let mut merkle = Vec::new();

    for t in &curr_trans {
        let hash = Chain::hash(t);
        merkle.push(hash);
    }

    if merkle.len() % 2 == 1 {
        let last = merkle.last().cloned().unwrap();
        merkle.push(last);
    }

    while merkle.len() > 1 {
        let mut h1 = merkle.remove(0);
        let mut h2 = merkle.remove(0);
        h1.push_str(&mut h2);
        let nh = Chain::hash(&h1);
        merkle.push(nh);
    }

    merkle.pop().unwrap()
}

pub fn proof_of_work(header: &mut BlockHeader) {
    println!("");
    let difficulty = header.difficulty as u64;
    let pb = ProgressBar::new(1024);
    let delta = 8 / difficulty;
    let handle = thread::spawn(move || {
        for _ in 0..(1024/(delta)) {
            pb.inc(delta);
            thread::sleep(Duration::from_millis(difficulty*10));
        }

        pb.finish_with_message("done");
    });

    loop {
        let hash = Chain::hash(header);
        let slice = &hash[..header.difficulty as usize];

        let mut m: String = String::from("");
        match slice.parse() as u32 {
            Ok(val) => {
                if val != 0 {
                    header.nonce += 1;
                } else {
                    // println!()
                    // println!("Block hash {}", hash);
                    // println!("");
                    m = hash;
                    break;
                }
            },
            Err(_) => {
                header.nonce += 1;
                continue;
            }
        };
        handle.join().unwrap();
        println!("");
        println!("Block hash: {}", m);
        println!("");
    }
}

pub fn hash <T: serde::Serialize>(item: &T) -> String {
    let input: String = serde_json::to_string(&item).unwrap();
    let mut hasher: Sha256 = Sha256::default();
    hasher.input(input.as_bytes());
    let res = hasher.result();
    let vec_res = res.to_vec();

    Chain::hex_to_string(vec_res.as_slice())
}

pub fn hex_to_string(vec_res: &[u8]) -> String {
    let mut s = String::new();
    for b in vec_res {
        write!(&mut s, "{:x}", b).expect("unable to write");
    }

    s
}