mod chain;
use chain::Chain;
use std::io;
fn main(){
    println!("Print miner's address");
    let mut miners_add=String::new();
    io::stdin().read_line(&mut miners_add).expect("msg");
    
    println!("Print miner's difficulty");
    let mut diff=String::new();
    io::stdin().read_line(&mut diff).expect("msg");
    let p_difficulty:u32=diff.trim().parse().unwrap();

    println!("printing genesis block");
    let mut chain=Chain::new(miners_add.as_str(), p_difficulty, 100.0);

    loop {
        println!("Menu");
        println!("1-> New Transaction");
        println!("2-> Mine Block");
        println!("3-> Change difficulty");
        println!("4-> Change Reward");
        println!("0-> Exit");
    
        let mut choice=String::new();
        io::stdin().read_line(&mut choice).expect("msg");
        let parced_choice:u32=choice.trim().parse().unwrap();
        match parced_choice {
            1=>{
                println!("sender-");
                let mut sender=String::new();
                io::stdin().read_line(&mut sender).expect("msg");
                println!("reciever-");
                let mut reciever=String::new();
                io::stdin().read_line(&mut reciever).expect("msg");
                println!("amount-");
                let mut amt=String::new();
                io::stdin().read_line(&mut amt).expect("msg");
                let p_amt=amt.trim().parse().unwrap();
                let res=chain.new_transaction(sender,reciever, p_amt);
                match res {
                    Ok(_)=>{
                        println!("Transaction is created");
                    }
                    Err(_)=>{
                        println!("Something went wrong");
    
                    }
                }
            },
            2=>{
                println!("Generating new block");
                let res= chain.genrate_newBlock();
                match res {
                    Ok(_)=>{
                        println!("block is added");
                    }
                    Err(_)=>{
                        println!("Something went wrong");
    
                    }
                }
            },
            3=>{
                let mut new_diff=String::new();
                io::stdin().read_line(&mut new_diff).expect("msg");
                let p_new_diff:u32=new_diff.trim().parse().unwrap();
                let res=chain.change_diff(p_new_diff);
                match res {
                    Ok(_)=>{
                        println!("Difficulty changed");
                    }
                    Err(_)=>{
                        println!("Something went wrong");
    
                    }
                }
    
            },
            4=>{
                let mut new_rew=String::new();
                io::stdin().read_line(&mut new_rew).expect("msg");
                let p_new_rew:f32=new_rew.trim().parse().unwrap();
                let res=chain.change_reward(p_new_rew);
                match res {
                    Ok(_)=>{
                        println!("reward changed");
                    }
                    Err(_)=>{
                        println!("Something went wrong");
    
                    }
                }
            },
            _=>{
                break;
            }
        }   
    }

}