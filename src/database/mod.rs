use crate::error::ApiError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing in Environment File");
        let manager = ConnectionManager::<PgConnection>::new(db_url);

        Pool::new(manager).expect("Failed to create DB connection pool")
    };
}

pub fn init() {
    info!("Initializing DB Connection");
    lazy_static::initialize(&POOL);

    let conn = connection().expect("Failed to gather DB connection");

    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed to gather database connection: {}", e)))
}
