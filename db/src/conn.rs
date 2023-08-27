use deadpool::managed::Object;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use std::env;
use std::sync::OnceLock;

pub type PoolDB = Pool<AsyncPgConnection>;

static POOLDB: OnceLock<PoolDB> = OnceLock::new();
pub async fn get_or_init() -> Option<&'static PoolDB> {
    match POOLDB.get() {
        Some(p) => Some(p),
        None => {
            let db_url = env::var("DATABASE_URL")
                .map_err(anyhow::Error::from)
                .expect("Must secret key in enviroment!");
            let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
            let pool = Pool::builder(config).max_size(1).build();

            match pool {
                Ok(poo) => {
                    POOLDB.set(poo).ok();
                }
                Err(e) => println!("Error database: {:?}", e),
            }
            POOLDB.get()
        }
    }
}

type Conn<'a> = Object<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub struct DB;

impl DB {
    // Create the DBManager instance using DBOptions
    pub async fn conn() -> Result<Conn<'static>, anyhow::Error> {
        let pool = get_or_init().await.unwrap();
        let conn = pool.get().await.unwrap();
        Ok(conn)
    }
}
