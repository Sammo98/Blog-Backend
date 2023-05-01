use std::sync::Arc;

use surrealdb::Surreal;
use surrealdb::engine::local::{File, Db};
use surrealdb::Result;
use crate::blog::{Blog, Metadata, Payload};


pub struct SDB(Surreal<Db>);

impl SDB {
    pub async fn init() -> Arc<Self> {
        let db = Surreal::new::<File>("main.db").await.expect("Failed to initiate file-based DB");
        db.use_ns("sam").use_db("blog").await.expect("Failed to connect to given namespace and database");
        Arc::new(SDB(db))
    }

    pub async fn create_blog(&self, payload:Payload) -> Result<()> {

        tracing::info!("Attempting to create blog: {:?}", payload);
        let id = payload.id.clone();

        let _: Blog = self.0
            .create(("blog", &id))
            .content(payload.blog)
            .await?;

        tracing::info!("Blog created with id: {id}.");
        Ok(())
    }

    pub async fn get_blog(&self, id:u32) -> anyhow::Result<Blog> {
        let blog: Option<Blog> = self.0
            .select(("blog", id.to_string()))
            .await?;

        match blog{
            Some(b) => Ok(b),
            None => Err(anyhow::anyhow!("Blog not found!"))
        }
    }

    pub async fn get_all_blogs_metadata(&self) -> Result<Vec<Metadata>> {
        let mut result = self.0
            .query("SELECT * FROM blog;")
            .await.unwrap();
        tracing::info!("{result:?}");
        let metadata:Vec<Metadata> = result.take("metadata").unwrap();
        Ok(metadata)
    }

    pub async fn delete_all_blogs(&self) -> Result<Vec<Blog>> {
        let deleted: Vec<Blog> = self.0.delete("blog").await.unwrap();
        Ok(deleted)
    }
}
