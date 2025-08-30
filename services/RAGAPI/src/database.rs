use anyhow::Result;
use chrono::{DateTime, Utc};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub filename: String,
    pub content: String,
    pub file_hash: String,
    pub chunk_index: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Embedding {
    pub id: i32,
    pub document_id: i32,
    #[serde(skip)]
    pub embedding: Vector,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentWithSimilarity {
    pub document: Document,
    pub similarity: f64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RagModel {
    pub id: i32,
    pub name: String,
    pub vector_id: i32,
    pub system_prompt: String,
    pub context: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct VectorInfo {
    pub id: i32,
    pub name: String,
    pub folder_name: String,
    pub description: Option<String>,
    pub document_count: Option<i32>,
    pub embedding_count: Option<i32>,
    pub created_by: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, sqlx::Error> {
        self.pool.begin().await
    }

    pub async fn search_similar_documents(
        &self,
        query_embedding: &[f32],
        limit: i32,
        similarity_threshold: f64,
    ) -> Result<Vec<DocumentWithSimilarity>> {
        use tracing::info;

        info!("Searching for similar documents with embedding dimension: {}, limit: {}, threshold: {}",
              query_embedding.len(), limit, similarity_threshold);

        // Convert query embedding to pgvector format
        let query_vector = Vector::from(query_embedding.to_vec());

        // Execute the vector similarity search using sqlx
        let rows = sqlx::query(
            r#"
            SELECT
                d.id, d.filename, d.content, d.file_hash, d.chunk_index,
                d.created_at, d.updated_at,
                1 - ((e.embedding <=> $1)/2) as similarity
            FROM documents d
            JOIN embeddings e ON d.id = e.document_id
            WHERE 1 - ((e.embedding <=> $1)/2) >= $3
            ORDER BY 1 - ((e.embedding <=> $1)/2) DESC
            LIMIT $2
            "#,
        )
        .bind(&query_vector)
        .bind(limit)
        .bind(similarity_threshold)
        .fetch_all(&self.pool)
        .await?;

        info!("Vector search query executed, found {} rows", rows.len());
        info!("Query embedding dimension: {}", query_embedding.len());
        info!("Similarity threshold: {}", similarity_threshold);

        let mut results = Vec::new();

        for row in rows {
            let id: i32 = row.get("id");
            let filename: String = row.get("filename");
            let content: String = row.get("content");
            let file_hash: String = row.get("file_hash");
            let chunk_index: i32 = row.get("chunk_index");
            let created_at: DateTime<Utc> = row.get("created_at");
            let updated_at: DateTime<Utc> = row.get("updated_at");
            let similarity: f64 = row.get("similarity");

            let document = Document {
                id,
                filename,
                content,
                file_hash,
                chunk_index,
                created_at,
                updated_at,
            };

            info!("Found similar document: id={}, filename={}, similarity={:.4}",
                  document.id, document.filename, similarity);

            results.push(DocumentWithSimilarity {
                document,
                similarity,
            });
        }

        info!("Returning {} similar documents", results.len());
        Ok(results)
    }

    pub async fn get_document_count(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM documents")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.get("count"))
    }

    pub async fn get_embedding_count(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM embeddings")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get("count"))
    }

    pub async fn store_document_and_embedding(
        &self,
        filename: &str,
        content: &str,
        embedding: &[f32],
    ) -> Result<i32> {
        use tracing::info;

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content);
        let file_hash = hex::encode(hasher.finalize());
        let query_vector = Vector::from(embedding.to_vec());

        // Begin transaction
        let mut tx = self.pool.begin().await?;

        // Insert document
        let document_id = sqlx::query(
            r#"
            INSERT INTO documents (filename, content, file_hash, chunk_index)
            VALUES ($1, $2, $3, 0)
            RETURNING id
            "#,
        )
        .bind(filename)
        .bind(content)
        .bind(file_hash)
        .fetch_one(&mut *tx)
        .await?
        .get("id");

        // Insert embedding
        sqlx::query(
            r#"
            INSERT INTO embeddings (document_id, embedding)
            VALUES ($1, $2)
            "#,
        )
        .bind(document_id)
        .bind(&query_vector)
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        info!("Stored document {} with embedding (dimension: {})", filename, embedding.len());
        Ok(document_id)
    }

    pub async fn get_document_content(&self, document_id: i32) -> Result<Option<(String, String)>> {
        let row = sqlx::query(
            r#"
            SELECT filename, content
            FROM documents
            WHERE id = $1
            "#,
        )
        .bind(document_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let filename: String = row.get("filename");
                let content: String = row.get("content");
                Ok(Some((filename, content)))
            }
            None => Ok(None),
        }
    }

    pub async fn store_document_embedding(&self, document_id: i32, embedding: &[f32]) -> Result<()> {
        let query_vector = pgvector::Vector::from(embedding.to_vec());

        sqlx::query(
            r#"
            INSERT INTO embeddings (document_id, embedding)
            VALUES ($1, $2)
            "#,
        )
        .bind(document_id)
        .bind(&query_vector)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_rag_model(&self, rag_model_id: i32) -> Result<Option<RagModel>> {
        let row = sqlx::query_as::<_, RagModel>(
            r#"
            SELECT id, name, vector_id, system_prompt, context, created_by, created_at, updated_at, is_active
            FROM rag_models
            WHERE id = $1 AND is_active = true
            "#,
        )
        .bind(rag_model_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn search_vector_documents(
        &self,
        query_embedding: &[f32],
        vector_id: i32,
        limit: i32,
        similarity_threshold: f64,
    ) -> Result<Vec<DocumentWithSimilarity>> {
        use tracing::info;

        info!("Searching for similar documents in vector {} with embedding dimension: {}, limit: {}, threshold: {}",
              vector_id, query_embedding.len(), limit, similarity_threshold);

        // Convert query embedding to pgvector format
        let query_vector = pgvector::Vector::from(query_embedding.to_vec());

        // Execute the vector similarity search filtered by vector_id using filename prefix
        let filename_prefix = format!("{}_", vector_id);
        let rows = sqlx::query(
            r#"
            SELECT
                d.id, d.filename, d.content, d.file_hash, d.chunk_index,
                d.created_at, d.updated_at,
                1 - ((e.embedding <=> $1)/2) as similarity
            FROM documents d
            JOIN embeddings e ON d.id = e.document_id
            WHERE d.filename LIKE $4 
              AND 1 - ((e.embedding <=> $1)/2) >= $3
            ORDER BY 1 - ((e.embedding <=> $1)/2) DESC
            LIMIT $2
            "#,
        )
        .bind(&query_vector)
        .bind(limit)
        .bind(similarity_threshold)
        .bind(format!("{}%", filename_prefix))
        .fetch_all(&self.pool)
        .await?;

        info!("Vector-specific search query executed for vector {}, found {} rows", vector_id, rows.len());

        let mut results = Vec::new();

        for row in rows {
            let id: i32 = row.get("id");
            let filename: String = row.get("filename");
            let content: String = row.get("content");
            let file_hash: String = row.get("file_hash");
            let chunk_index: i32 = row.get("chunk_index");
            let created_at: DateTime<Utc> = row.get("created_at");
            let updated_at: DateTime<Utc> = row.get("updated_at");
            let similarity: f64 = row.get("similarity");

            let document = Document {
                id,
                filename,
                content,
                file_hash,
                chunk_index,
                created_at,
                updated_at,
            };

            info!("Found similar document in vector {}: id={}, filename={}, similarity={:.4}",
                  vector_id, document.id, document.filename, similarity);

            results.push(DocumentWithSimilarity {
                document,
                similarity,
            });
        }

        info!("Returning {} similar documents for vector {}", results.len(), vector_id);
        Ok(results)
    }

    pub async fn get_rag_model_by_name(&self, rag_model_name: &str) -> Result<Option<RagModel>> {
        let row = sqlx::query_as::<_, RagModel>(
            r#"
            SELECT id, name, vector_id, system_prompt, context, created_by, created_at, updated_at, is_active
            FROM rag_models
            WHERE name = $1 AND is_active = true
            "#,
        )
        .bind(rag_model_name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}