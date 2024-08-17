mod db;
mod graphql;


use db::conn::establish_connection;
use graphql::handler::start_server;


fn main()  {
    let conn = &mut  establish_connection();

    if let Err(err) =  start_server() {
        println!("{}", &err);
        std::process::exit(1);
    }

}

