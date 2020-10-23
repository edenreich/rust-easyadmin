use diesel::MysqlConnection;
use r2d2_diesel::ConnectionManager;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new("mysql://root:secret@127.0.0.1:3306/easyadmin");
    Pool::new(manager).expect("db pool to be created")
}