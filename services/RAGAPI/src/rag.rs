use crate::bedrock_client::{BedrockApiClient, RAGRequest};
use crate::database::{Database, DocumentWithSimilarity};
use crate::embeddings::EmbeddingService;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct RAGResponse {
    pub answer: String,
    pub sources: Vec<DocumentSource>,
    pub query: String,
    pub context_used: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentSource {
    pub filename: String,
    pub chunk_index: i32,
    pub similarity: f64,
    pub snippet: String,
}

pub struct RAGService {
    pub database: Database,
    pub embedding_service: EmbeddingService,
    pub bedrock_client: BedrockApiClient,
}

impl RAGService {
    pub fn new(
        database: Database,
        embedding_service: EmbeddingService,
        bedrock_client: BedrockApiClient,
    ) -> Self {
        Self {
            database,
            embedding_service,
            bedrock_client,
        }
    }

    pub async fn query(&self, request: RAGRequest, token: &str) -> Result<RAGResponse> {
        info!("Processing RAG query: {}", request.query);

        // Generate embedding for the query
        let query_embedding = self.embedding_service.get_embedding(&request.query).await?;
        info!("Generated query embedding with dimension: {}", query_embedding.len());

        // Search for similar documents
        let similar_docs = self
            .database
            .search_similar_documents(&query_embedding, 5, -10.0f64)
            .await?;

        if similar_docs.is_empty() {
            warn!("No relevant documents found for query: {}", request.query);
        }

        // Build context from retrieved documents
        let mut context_parts = Vec::new();
        let mut sources = Vec::new();

        // Limit context size to avoid model input limits
        const MAX_CONTEXT_LENGTH: usize = 8000; // Conservative limit for most models
        const MAX_DOCUMENTS: usize = 10; // Limit number of documents to include

        for (i, doc_with_sim) in similar_docs.iter().enumerate() {
            if i >= MAX_DOCUMENTS {
                break; // Only use top 10 most relevant documents
            }

            // Truncate document content to fit within context limits
            let max_content_length = MAX_CONTEXT_LENGTH / MAX_DOCUMENTS;
            let content = if doc_with_sim.document.content.len() > max_content_length {
                // Try to find a good breaking point (sentence end)
                let mut truncate_at = max_content_length;
                if let Some(last_sentence) = doc_with_sim.document.content[..max_content_length]
                    .rfind(|c: char| c == '.' || c == '!' || c == '?') {
                    truncate_at = last_sentence + 1;
                }

                format!("{}... [TRUNCATED]", &doc_with_sim.document.content[..truncate_at])
            } else {
                doc_with_sim.document.content.clone()
            };

            let snippet = if content.len() > 300 {
                format!("{}...", &content[..300])
            } else {
                content.clone()
            };

            context_parts.push(format!(
                "Source: {} (Similarity: {:.3})\n{}",
                doc_with_sim.document.filename,
                doc_with_sim.similarity,
                content
            ));

            sources.push(DocumentSource {
                filename: doc_with_sim.document.filename.clone(),
                chunk_index: doc_with_sim.document.chunk_index,
                similarity: doc_with_sim.similarity,
                snippet,
            });
        }

        let retrieved_context = context_parts.join("\n\n---\n\n");

        // Combine with user-provided context if any
        let full_context = match &request.context {
            Some(user_context) if !user_context.is_empty() => {
                format!("{}\n\n---\n\n{}", user_context, retrieved_context)
            }
            _ => retrieved_context.clone(),
        };

        // Build the prompt for the LLM
        let system_prompt = request.system_prompt.unwrap_or_else(|| {
            "You are a helpful assistant that answers questions based on the provided context. \
             Use the context information to provide accurate and relevant answers. \
             If the context doesn't contain enough information to answer the question, \
             say so clearly and explain what information is missing."
                .to_string()
        });

        let prompt = if full_context.is_empty() {
            format!(
                "System: {}\n\nUser: {}\n\nAssistant:",
                system_prompt, request.query
            )
        } else {
            format!(
                "System: {}\n\nContext:\n{}\n\nUser: {}\n\nAssistant:",
                system_prompt, full_context, request.query
            )
        };

        info!("Generated prompt length: {} characters", prompt.len());

        // Get response from Bedrock via BedrockAPI
        let bedrock_response = self
            .bedrock_client
            .generate_response(
                &prompt,
                request.max_tokens.or(Some(1000)),
                request.temperature.or(Some(0.7)),
                token,
            )
            .await?;

        Ok(RAGResponse {
            answer: bedrock_response.response,
            sources,
            query: request.query,
            context_used: full_context,
        })
    }

    pub async fn get_stats(&self) -> Result<RAGStats> {
        let document_count = self.database.get_document_count().await?;
        let embedding_count = self.database.get_embedding_count().await?;

        Ok(RAGStats {
            document_count,
            embedding_count,
        })
    }

    pub async fn process_document(
        &self,
        filename: &str,
        content: &str,
    ) -> Result<i32> {
        // Generate embedding for the document content
        let embedding = self.embedding_service.get_embedding(content).await?;

        // Store the document and embedding in database
        self.database.store_document_and_embedding(filename, content, &embedding).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RAGStats {
    pub document_count: i64,
    pub embedding_count: i64,
}