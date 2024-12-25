use deadpool_diesel::sqlite::{Manager, Pool, Runtime};
use std::{env, path::Path};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn establish_connection() -> anyhow::Result<Pool> {
    let database_url = env::var("DATABASE_URL")?;
    let path = Path::new(&database_url);
    if !path.exists() {
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent)?;
        std::fs::File::create(path)?;
    }
    // set up connection pool
    let manager = Manager::new(database_url, Runtime::Tokio1);
    let pool = Pool::builder(manager).max_size(8).build().unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    Ok(pool)
}
