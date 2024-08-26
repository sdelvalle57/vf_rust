## Install postgres in Ubuntu:
`sudo apt install postgresql postgresql-contrib`

`sudo systemctl enable postgresql`

`sudo systemctl start postgresql`

`sudo systemctl status postgresql`

## Install Dbeaver-ce
`sudo apt install dbeaver-ce`

## Config Postgres
`sudo nano /etc/postgresql/14/main/pg_hba.conf` 

Look for the following line: 

host    all             all             127.0.0.1/32            md5

`sudo nano /etc/postgresql/14/main/postgresql.conf`

Ensure that listen_addresses is set to '*' to allow connections from any IP 

`sudo systemctl restart postgresql`

## Create Postgres DB
`sudo -i -u postgres`

`psql`

`CREATE USER value_flows WITH PASSWORD 'valueflows';`

`CREATE DATABASE vf5;`

`GRANT ALL PRIVILEGES ON DATABASE vf5 TO value_flows;`

`\du`

`\q`

`exit`

## Install Diesel CLI
`cargo install diesel_cli --no-default-features --features postgres`

## Update .env in the project:
`DATABASE_URL=postgres://your_user_name:your_password@localhost/your_database_name`

## Run Diesel in your project
`diesel setup`

`diesel migration generate`

Copy the content of table into up.sql

`diesel migration run`

## Run The Project
`cargo run`

`diesel print-schema > src/db/schema.rs`
