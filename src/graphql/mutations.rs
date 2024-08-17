use juniper::{FieldResult, GraphQLObject};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "A block in the blockchain")]
struct Block {
    header: String,
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn init_new_blockchain(data: String) -> FieldResult<Block> {
        Ok(Block {
            header: data,
        })
    }
}
