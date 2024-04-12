use std::time::Duration;

use sqlx::{self, postgres::PgPoolOptions, Error, Pool, Postgres};

use crate::config;

pub type DB = Pool<Postgres>;

pub async fn connect(database: &config::Database) -> Result<DB, Error> {
    // See https://www.alexedwards.net/blog/configuring-sqldb
    // and https://making.pusher.com/production-ready-connection-pooling-in-go
    // for the details
    // ret.SetMaxOpenConns(int(poolSize))
    // ret.SetMaxIdleConns(int(poolSize / 2))
    // ret.SetConnMaxLifetime(30 * time.Minute)
    PgPoolOptions::new()
        .max_connections(database.pool_size)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect(&database.url)
        .await
        .map_err(|err| {
            tracing::error!("{:?}", err);
            err.into()
        })
}

pub async fn migrate(db: &DB) -> Result<(), Error> {
    match sqlx::migrate!("./migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(err) => {
            tracing::error!("{}", &err);
            Err(err)
        }
    }?;

    Ok(())
}
