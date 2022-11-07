use chrono::prelude::*;
use sha2::{Digest,Sha256};
use std::fmt::Write;
use serde_derive::{Deserialize,Serialize};
#[derive(Debug)]
struct Block{
    header:BlockHeader,
    transaction:Vec<Transction>,
    count:u32
}
#[derive(Debug,Deserialize,Serialize)]
struct BlockHeader{
    time_stamp:i64,
    nonce:u64,
    prev_hash:String,
    current_hash:String,
    difficulty:u32
}
#[derive(Debug,Deserialize,Serialize,Clone)]
struct Transction{
    sender:String,
    reciever:String,
    amount:f32
}
#[derive(Debug)]
pub struct Chain<'a>{
    chain:Vec<Block>,
    miner_add:&'a str,
    reward:f32,
    difficulty:u32,
    curr_transaction:Vec<Transction>
}
impl <'a> Chain<'a> {
    pub fn new(miner_add:&'a str,difficulty:u32,reward:f32)->Self {
        let mut chain= Chain{ chain: vec![], miner_add, reward, difficulty, curr_transaction: vec![] };
        chain.genrate_newBlock();
        chain
    }
    pub fn change_diff(&mut self,difficulty:u32)->Result<(),String> {
        self.difficulty=difficulty;
        Ok(())
    }
    pub fn change_reward(&mut self,reward:f32)->Result<(),String> {
        self.reward=reward;
        Ok(())
    }
    pub fn new_transaction(&mut self,sender:String,reciever:String,amount:f32)->Result<(),String> {
        let trans=Transction{
            sender,reciever,amount
        };
        self.curr_transaction.push(trans);
        self.genrate_newBlock();
        Ok(())
    }

    pub fn genrate_newBlock(&mut self)->Result<(),String> {
        let header=BlockHeader { time_stamp: Utc::now().timestamp()
        , nonce: 0
        , prev_hash: self.get_prev_hash()
        , current_hash: String::new()
        , difficulty: self.difficulty
     };
     let tran=Transction{
        sender:"Root".to_owned(),
        reciever:self.miner_add.to_owned(),
        amount:100.0
     };


        let mut block=Block{
            header:header,transaction:vec![],count:0
        };
        block.transaction.push(tran);
        block.transaction.append(&mut self.curr_transaction);
        block.count=block.transaction.len() as u32;
        block.header.current_hash=Self::get_currentHash(block.transaction.clone());
        Self::POW(&mut block.header);
        println!("{:#?}",block);
        self.chain.push(block);
        Ok(())
    } 
    
    fn POW(header:&mut BlockHeader) {
        loop {
            let hash=Self::hash(header);
            let slice=&hash[..header.difficulty as usize];
            let p_slice=slice.parse::<u32>();
            match p_slice {
                Ok(val)=>{
                    if val!=0{
                        header.nonce+=1;
                    }else {
                        println!("{:?}",hash);
                        break;
                    }
                }
                Err(_)=>{
                    header.nonce+=1;
                    continue;
                }
            }
        }
            
    }


    fn get_currentHash(current_tran:Vec<Transction>)->String {
       
        let mut markle=Vec::new();
        for i in &current_tran {
            let hash=Self::hash(i); 
            markle.push(hash);
        }
        if markle.len()%2==1 {
            let prev=markle.last().cloned().unwrap();
            markle.push(prev);
        }
        while markle.len()>1 {
            let mut h1=markle.remove(0);
            let mut h2=markle.remove(0);
            h1.push_str(&mut h2);
            let combine_hash=Self::hash(&h1);
            markle.push(combine_hash);
        }
        markle.pop().unwrap()
    }



    fn get_prev_hash(&self)->String {
        let prev_block=match self.chain.last(){
            Some(block)=>{
                block
            }
            None=>{return String::from("0000000000000000000000000000000")}
        };
        Self::hash(&prev_block.header)
    }
    fn hash<T: serde::ser::Serialize>( item:&T)->String {
        let item_string=serde_json::to_string(&item).unwrap();
        let mut hasher=Sha256::new();
        hasher.update(item_string.as_bytes());
        let res=hasher.finalize();
        let item_vec=res.to_vec();
        Self::hex_to_string(item_vec)
    }
    fn hex_to_string(item:Vec<u8>)->String {
        let mut s=String::new();
        for b in item {
            write!(&mut s,"{:?}",b);
        }
        s
    }
}