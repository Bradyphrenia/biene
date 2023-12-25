use postgres::{Client, NoTls};

use crate::database::structs::Database;

/// connect to a database and return postgres client or throw error
pub fn connect(db_settings: &Database) -> Result<Client, String> {
    match Client::connect(
        // postgresql://USER:PASSWORD@URL:PORT/DATABASE
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            db_settings.user,
            db_settings.password,
            db_settings.url,
            db_settings.port,
            db_settings.database
        )
        .as_str(),
        NoTls,
    ) {
        Ok(client) => Ok(client),
        Err(_e) => Err(format!("{}", _e)),
    }
}

/// disconnect database
pub fn disconnect() {
    // TODO: implement me
}
