mod block;
mod blockchain;
mod transactions;

use crate::blockchain::Blockchain

fn main() {
    let mut blockchain = Blockchain::new();

    //Add Transaction
    blockchain.new_transaction("Alice".to_string(),"Bob".to_string(),30.0);

    //Mine another block
    blockchain.add_block();

    //Add another transaction
    blockchain.new_transaction("Charlie".to_string(),"Alice".to_string(),50.0);

    println!("{:?}", blockchain);

    if blockchain.is_chain_valid(){
        println!("The blockchain is valid!");
    }else{
        println!("The blockchain is not valid");
    }

}

//User input transaction
//Limit number of pending transaction
//Verify