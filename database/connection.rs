use diesel::MysqlConnection;
use r2d2_diesel::ConnectionManager;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub fn init_pool() -> Pool {
    let manager: ConnectionManager<MysqlConnection> = ConnectionManager::<MysqlConnection>::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL to be set!"),
    );
    Pool::builder()
        .build(manager)
        .expect("failed to create pool")
}
