use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::thread;
extern crate r2d2;

pub fn connect() {
    let conn_str: String = dotenv::var("CONN_STRING").unwrap();

    let manager = PostgresConnectionManager::new(conn_str.parse().unwrap(), NoTls);

    let pool = r2d2::Pool::new(manager).unwrap();

    thread::spawn(move || {
        let mut client = pool.get().unwrap();
/*         client.batch_execute(
                "
                CREATE TABLE wifi_companies (
                    company_id  SERIAL PRIMARY KEY,
                    name    TEXT NOT NULL
                );
            ",
            )
            .unwrap(); */

        client.batch_execute(
                "
                CREATE TABLE wifi_services (
                    wifi_id SERIAL PRIMARY KEY,
                    name TEXT NOT NULL,
                    price INT,
                    quantity INT
                );
            ",
            ).unwrap();
    });

    /*     let mut client = Client::connect(&conn_str, NoTls).unwrap(); */
}
