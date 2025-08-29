use std::path::Path;
use tokio::fs;
use tracing::{info, error, warn};
use anyhow::Result;
use serde_json::json;

pub struct VectorProcessor {
    pub documents_base_path: String,
    pub ragapi_url: String,
}

impl VectorProcessor {
    pub fn new(documents_base_path: String, ragapi_url: String) -> Self {
        Self {
            documents_base_path,
            ragapi_url,
        }
    }

    pub async fn process_folder_to_vector(&self, vector_id: i32, folder_name: &str) -> Result<VectorProcessingResult> {
        info!("Starting vector processing for vector ID: {} from folder: {}", vector_id, folder_name);

        let folder_path = Path::new(&self.documents_base_path).join(folder_name);
        
        if !folder_path.exists() || !folder_path.is_dir() {
            return Err(anyhow::anyhow!("Folder does not exist: {}", folder_name));
        }

        // Count documents in folder
        let document_files = self.get_document_files(&folder_path).await?;
        let document_count = document_files.len();
        
        info!("Found {} documents in folder: {}", document_count, folder_name);

        if document_count == 0 {
            return Ok(VectorProcessingResult {
                success: true,
                document_count: 0,
                embedding_count: 0,
                message: "No documents found in folder".to_string(),
            });
        }

        // Process each document through RAGAPI
        let mut total_embeddings = 0;
        let mut processed_documents = 0;

        for file_path in document_files {
            match self.process_document(&file_path, vector_id).await {
                Ok(embedding_count) => {
                    total_embeddings += embedding_count;
                    processed_documents += 1;
                    info!("Processed document: {} ({} embeddings)", file_path.display(), embedding_count);
                }
                Err(e) => {
                    error!("Failed to process document {}: {}", file_path.display(), e);
                    // Continue processing other documents
                }
            }
        }

        info!("Vector processing completed. Processed {}/{} documents with {} total embeddings", 
              processed_documents, document_count, total_embeddings);

        Ok(VectorProcessingResult {
            success: processed_documents > 0,
            document_count: processed_documents,
            embedding_count: total_embeddings,
            message: if processed_documents == document_count as i32 {
                format!("Successfully processed all {} documents", document_count)
            } else {
                format!("Processed {}/{} documents successfully", processed_documents, document_count)
            },
        })
    }

    async fn get_document_files(&self, folder_path: &Path) -> Result<Vec<std::path::PathBuf>> {
        let mut document_files = Vec::new();
        let allowed_extensions = ["pdf", "txt", "md", "docx"];

        let mut entries = fs::read_dir(folder_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                    if allowed_extensions.contains(&extension.to_lowercase().as_str()) {
                        document_files.push(path);
                    }
                }
            }
        }

        Ok(document_files)
    }

    async fn process_document(&self, file_path: &Path, vector_id: i32) -> Result<i32> {
        use std::io::Read;
        
        info!("Processing document: {} for vector {}", file_path.display(), vector_id);

        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Extract text content based on file type
        let content = if file_name.ends_with(".pdf") {
            // Extract actual text from PDF
            match pdf_extract::extract_text(file_path) {
                Ok(text) => {
                    info!("Successfully extracted {} characters from PDF: {}", text.len(), file_name);
                    text
                }
                Err(e) => {
                    error!("Failed to extract text from PDF {}: {}", file_name, e);
                    // Fallback to reading as binary and extracting what we can
                    return Err(anyhow::anyhow!("PDF text extraction failed: {}", e));
                }
            }
        } else {
            // For text files, read actual content
            let mut file = std::fs::File::open(file_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            content
        };

        if content.is_empty() {
            return Ok(0);
        }

        // Split content into chunks for better RAG retrieval
        let chunks = self.split_into_chunks(&content, 1000); // 1000 character chunks with overlap
        let total_chunks = chunks.len();
        info!("Split document {} into {} chunks", file_name, total_chunks);
        
        let mut total_chunks_processed = 0;
        let client = reqwest::Client::new();
        
        // Process each chunk separately
        for (chunk_index, chunk_content) in chunks.into_iter().enumerate() {
            let chunk_filename = format!("{}_{}_chunk_{}", vector_id, file_name, chunk_index);
            
            let response = client
                .post(&format!("{}/process-document", self.ragapi_url))
                .json(&serde_json::json!({
                    "filename": chunk_filename,
                    "content": chunk_content
                }))
                .send()
                .await?;

            if response.status().is_success() {
                total_chunks_processed += 1;
                info!("Successfully processed chunk {} of {}", chunk_index + 1, file_name);
            } else {
                error!("Failed to process chunk {} of {} via RAGAPI: {}", chunk_index, file_name, response.status());
                // Continue processing other chunks even if one fails
            }
        }

        info!("Processed {}/{} chunks for document {}", total_chunks_processed, total_chunks, file_name);
        Ok(total_chunks_processed)
    }

    fn split_into_chunks(&self, content: &str, chunk_size: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let words: Vec<&str> = content.split_whitespace().collect();
        
        if words.is_empty() {
            return chunks;
        }
        
        let mut current_chunk = String::new();
        let overlap_size = chunk_size / 4; // 25% overlap between chunks
        
        for word in words {
            // Check if adding this word would exceed chunk size
            if current_chunk.len() + word.len() + 1 > chunk_size && !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                
                // Create overlap by keeping last portion of previous chunk
                let chunk_words: Vec<&str> = current_chunk.split_whitespace().collect();
                if chunk_words.len() > overlap_size / 10 {
                    let overlap_start = chunk_words.len().saturating_sub(overlap_size / 10);
                    current_chunk = chunk_words[overlap_start..].join(" ");
                    current_chunk.push(' ');
                    current_chunk.push_str(word);
                } else {
                    current_chunk = word.to_string();
                }
            } else {
                if !current_chunk.is_empty() {
                    current_chunk.push(' ');
                }
                current_chunk.push_str(word);
            }
        }
        
        // Add the last chunk if it's not empty
        if !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }
        
        chunks
    }
}

#[derive(Debug)]
pub struct VectorProcessingResult {
    pub success: bool,
    pub document_count: i32,
    pub embedding_count: i32,
    pub message: String,
}

// Background task runner
pub async fn run_vector_processing_task(
    processor: VectorProcessor,
    vector_id: i32,
    folder_name: String,
    database: std::sync::Arc<crate::database::Database>,
) {
    info!("Starting background vector processing task for vector ID: {}", vector_id);

    match processor.process_folder_to_vector(vector_id, &folder_name).await {
        Ok(result) => {
            info!("Vector processing completed: {:?}", result);
            
            // Update vector counts in database
            if let Err(e) = database.update_vector_counts(vector_id, result.document_count, result.embedding_count).await {
                error!("Failed to update vector counts in database: {}", e);
            } else {
                info!("Updated vector {} with {} documents and {} embeddings", 
                      vector_id, result.document_count, result.embedding_count);
            }
        }
        Err(e) => {
            error!("Vector processing failed for vector ID {}: {}", vector_id, e);
        }
    }
}