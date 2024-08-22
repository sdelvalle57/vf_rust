use juniper::{graphql_object, FieldResult, GraphQLObject};

#[derive(GraphQLObject)]
struct Human {
    id: String,
    name: String,
    home_planet: String,
}

pub struct QueryRoot;

#[graphql_object(Context = crate::graphql::context::Context)]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: _id.to_owned(),
            name: "Luke".to_owned(),
            home_planet: "Mars".to_owned(),
        })
    }
}
