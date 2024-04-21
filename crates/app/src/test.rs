// use axum::{
//     response::{IntoResponse, Response},
//     Json,
// };
// use chrono::{DateTime, Utc};
// use hyper::StatusCode;
// use serde::{Deserialize, Serialize};
// use sqlx::prelude::FromRow;
// use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, FromRow, Default, Clone)]
pub struct ProductBrand {
    pub id: uuid::Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub value: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Default)]
pub struct ProductBrands {
    pub items: Vec<ProductBrand>,
}

// Вспомогательные структуры для ввода/вывода данных
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ProductBrandBodyInput {
    pub id: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct ProductBrandCreateBodyInput {
    pub value: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ProductBrandUpdateBodyInput {
    pub id: Uuid,
    pub value: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductBrandDeleteBodyInput {
    pub id: uuid::Uuid,
}

// Методы для работы с ProductBrand
impl ProductBrand {
    pub async fn create(pool: &sqlx::PgPool, new_brand: ProductBrand) -> Result<Self, sqlx::Error> {
        let brand = sqlx::query_as!(
            ProductBrand,
            r#"
          INSERT INTO brands (value, metadata)
          VALUES ($1, $2)
          RETURNING *
          "#,
            new_brand.value,
            new_brand.metadata as Option<serde_json::Value>
        )
        .fetch_one(pool)
        .await?;

        Ok(brand)
    }

    pub async fn read(pool: &sqlx::PgPool, brand_id: uuid::Uuid) -> Result<Self, sqlx::Error> {
        let brand = sqlx::query_as!(
            ProductBrand,
            r#"
            SELECT * FROM brands WHERE id = $1
          "#,
            brand_id
        )
        .fetch_one(pool)
        .await?;

        Ok(brand)
    }

    pub async fn read_active(
        pool: &sqlx::PgPool,
        brand_id: uuid::Uuid,
    ) -> Result<Self, sqlx::Error> {
        let brand = sqlx::query_as!(
            ProductBrand,
            r#"
            SELECT * FROM brands WHERE id = $1 AND deleted_at IS NULL
          "#,
            brand_id
        )
        .fetch_one(pool)
        .await?;

        Ok(brand)
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        brand_id: uuid::Uuid,
        updated_brand: ProductBrand,
    ) -> Result<Self, sqlx::Error> {
        let brand = sqlx::query_as!(
            ProductBrand,
            r#"
            UPDATE brands
            SET
            value = COALESCE($2, value),
            metadata = COALESCE($3, metadata),
            updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING *
          "#,
            brand_id,
            updated_brand.value,
            updated_brand.metadata as Option<serde_json::Value>
        )
        .fetch_one(pool)
        .await?;

        Ok(brand)
    }

    pub async fn delete(pool: &sqlx::PgPool, brand_id: uuid::Uuid) -> Result<u64, sqlx::Error> {
        let rows_deleted = sqlx::query!(
            r#"
          DELETE FROM brands WHERE id = $1
          "#,
            brand_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_deleted)
    }

    pub async fn soft_delete(
        pool: &sqlx::PgPool,
        brand_id: uuid::Uuid,
    ) -> Result<u64, sqlx::Error> {
        let rows_updated = sqlx::query!(
            r#"
          UPDATE brands
          SET deleted_at = NOW()
          WHERE id = $1
          "#,
            brand_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_updated)
    }
}

// Методы для работы с ProductBrands
impl ProductBrands {
    pub async fn read_active(pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
        let brands = sqlx::query_as!(
            ProductBrand,
            r#"
            SELECT * FROM brands WHERE deleted_at IS NULL
          "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(ProductBrands { items: brands })
    }
}

// Реализация IntoResponse для отправки ответов
impl IntoResponse for ProductBrand {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for ProductBrands {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}