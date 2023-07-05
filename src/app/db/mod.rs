use postgres::{NoTls};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub type DbPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<postgres::NoTls>>;

pub fn connection_pool() -> DbPool {
    let conn_str: String = dotenv::var("CONN_STRING").unwrap();


    let manager = PostgresConnectionManager::new(conn_str.parse().unwrap(), NoTls);

    println!("Connected to Postgres!");
    Pool::builder()
    .build(manager)
    .expect("Falted to create database connection pool")
}
