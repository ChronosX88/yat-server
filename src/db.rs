use dotenv::dotenv;
use std::env;
use diesel::SqliteConnection;
use r2d2::{Pool};
use r2d2_diesel::ConnectionManager;

embed_migrations!();

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn run_migrations(conn: &SqliteConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB pool.");
    run_migrations(&pool.get().unwrap());
    return pool;
}