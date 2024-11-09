use crate::block::Block;
use crate::transaction::Transaction;
use crate::transactions::Transaction;

#[derive(Debug)]
pub struct Blockchain{
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>, //Hold Pending Transactions
    }

impl Blockchain{
    pub fn new() ->Blockchain{
        let mut blockchain=Blockchain{
            chain: vec![],
            pending_transactions:vec![],
            //Initialized an empty pool
        };
        blockchain.add_genesis_block();
        blockchain
    }

    pub fn add_genesis_block(&mut self){
        let genesis_block=Block::new(1, vec![],"0".to_string());
        //Empty Transaction for genesis block
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self){
        let previous_block = self.chain.last().unwrap();
        let mut new_block=Block::new(
            previous_block.index +1,
            self.pending_transactions.clone(),
            previous_block.hash.clone(),
        );

        new_block.mine_block(1);

        self.chain.push(new_block);
        self.pending_transactions.clear();
    }

    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: f64) -> u64{
        let transaction = Transaction{
            sender,
            recipient,
            amount,
        };
        self.pending_transactions.push(transaction);

        self.chain.last().unwrap().index+1 //return the index of next block
    }

    pub fn is_chain_valid(&self) ->bool{
        for i in 1..self.chain.len(){
            let current_block= &self.chain[i];
            let previous_block= &self.chain[i-1];

            if current_block.hash != Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.transactions,
                &current_block.previous_hash,
                current_block.nonce,
            ){
                return false;
            }

            if current_block.previous_hash != previous_block.hash{
                return false;
            }
        }
        true
    }
}