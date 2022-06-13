use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;

use crate::database_errors::DbError;
use crate::settings::Settings;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;


embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let config = Settings::default();
        let manager = ConnectionManager::<PgConnection>::new(config.database_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, r2d2::Error> {
    POOL.get()
}