use juniper::{FieldResult, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: _id.to_owned(),
            name: "Luke".to_owned(),
            home_planet: "Mars".to_owned(),
        })
    }
}



// use async_graphql::{Context, Object, Result};
// use uuid::Uuid;
// use crate::{agent::Agent, db::conn::establish_connection};

// pub struct Query;

// #[Object]
// impl Query {
//     async fn agent(&self, ctx: &Context<'_>, id: String) -> Result<Agent> {
//         let id = Uuid::try_parse(&id)
//             .expect("Unable to parse id");
//         let conn = &mut establish_connection();
//         let result = agents.filter(id.eq(id))
//             .first::<Agent>(conn)
//             .expect("Error loading agent");
//         Ok(result)
//     }
// }



