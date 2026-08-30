use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};

use crate::common::app_state::DbPool;

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub trait ConnectionProvider {
    fn pool(&self) -> &DbPool;

    fn get_connection(&self) -> Result<Conn, String> {
        self.pool()
            .get()
            .map_err(|e| format!("Error connecting to database: {}", e))
    }
}
