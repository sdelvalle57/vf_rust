use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    

    use super::*;
    use diesel::sql_query;


    #[test]
    fn test_connection() {
        // Use the same establish_connection function
        let mut conn = establish_connection();
        let result = sql_query("SELECT 1").execute(&mut conn);
        assert!(result.is_ok());
    }

    #[test]
    fn test_some_db_functionality() {
        use crate::agent::Agent;

        let conn = &mut establish_connection();

        // Run a SELECT query and load the results
        let results: Vec<Agent> = sql_query("SELECT * FROM agents")
            .load::<Agent>(conn)
            .expect("Failed to load agents");

        // Print out the results
        for agent in &results {
            println!("{:?}", agent);
        }

        assert!(!results.is_empty(), "No agents found");
    }
}