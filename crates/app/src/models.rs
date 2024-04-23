use sqlx::{FromRow,Error};
use image::{ImageBuffer, Luma};
use tokio::fs;
use std::path::Path;
// use tokio::stream::StreamExt;



#[derive(FromRow, Debug)]
// #[table_name = "images"]
pub struct Image {
    pub id: i32,
    pub image_name: String,
    pub image_data: Vec<u8>,
}

impl Image {
  
    pub async fn create(image_name: &str, image_data: Vec<u8>, pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Image>("INSERT INTO images (image_name, image_data) VALUES ($1, $2) RETURNING *")
            .bind(image_name)
            .bind(image_data)
            .fetch_one(pool)
            .await?;
        Ok(())
    }

    pub async fn update(id: i32, image_name: &str, image_data: &[u8], pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE images SET image_name = $1, image_data = $2 WHERE id = $3")
            .bind(image_name)
            .bind(image_data)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete( pool: &sqlx::PgPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM images WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
    
}