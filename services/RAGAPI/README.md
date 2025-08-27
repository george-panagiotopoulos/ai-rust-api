# RAGAPI

Retrieval-Augmented Generation microservice built with Rust, providing intelligent document-based question answering using vector similarity search.

## 🚀 Features

- **Automatic Document Embedding**: AWS Titan embeddings with fallback
- **Vector Similarity Search**: pgvector-powered document retrieval
- **RAG Pipeline**: Context-aware question answering
- **Document Management**: Automatic ingestion and storage
- **REST API**: Clean HTTP endpoints for Q&A
- **Health Monitoring**: Service statistics and health checks

## 📡 API Endpoints

### Core Functionality
- `POST /query` - Ask questions with RAG retrieval
- `POST /search` - Search documents by vector similarity

### Monitoring
- `GET /health` - Service health check
- `GET /stats` - System statistics (document count, embedding count)

## 🔧 Configuration

### Environment Variables (.env)
```bash
# Database Configuration
DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb

# Server Configuration
HOST=127.0.0.1
PORT=9101

# BedrockAPI Integration
BEDROCK_API_URL=http://127.0.0.1:9100

# AWS Configuration (Optional - uses mock embeddings if unavailable)
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_REGION=us-east-1

# Embedding Configuration
EMBEDDING_MODEL=amazon.titan-embed-text-v1
EMBEDDING_DIMENSION=1536
```

### Database Setup
```bash
# Start PostgreSQL with pgvector
docker-compose up -d

# Initialize database schema
psql postgresql://raguser:password@localhost:5434/ragdb < init.sql
```

## 🚀 Usage Examples

### RAG Query
```bash
curl -X POST http://localhost:9101/query \
  -H "Content-Type: application/json" \
  -d '{"query": "Who is the author of the AI basics document?"}'
```

Response:
```json
{
  "answer": "Based on the provided context, the author of the AI basics document is George...",
  "sources": [
    {
      "filename": "ai_basics.md",
      "chunk_index": 0,
      "similarity": 1.0,
      "snippet": "Artificial Intelligence Fundamentals..."
    }
  ],
  "query": "Who is the author of the AI basics document?",
  "context_used": "Source: ai_basics.md (Chunk 1)...## Author\nThe author of the document is George"
}
```

### Document Search
```bash
curl -X POST http://localhost:9101/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "machine learning",
    "limit": 5,
    "similarity_threshold": 0.1
  }'
```

## 🏗️ Architecture

```
RAGAPI/
├── src/
│   ├── main.rs              # Server setup and routing
│   ├── handlers.rs          # HTTP request handlers
│   ├── rag.rs               # RAG pipeline logic
│   ├── database.rs          # PostgreSQL integration
│   ├── embeddings.rs        # AWS Titan embedding service
│   ├── bedrock_client.rs    # BedrockAPI communication
│   └── config.rs            # Configuration management
├── documents/               # Document storage directory
├── docker-compose.yml       # PostgreSQL with pgvector
├── init.sql                 # Database initialization
├── migrations/              # Schema migrations
├── Cargo.toml              # Dependencies
└── .env                    # Configuration
```

## 🔄 How RAG Works

1. **Document Processing**: On startup, documents in `/documents` are automatically embedded
2. **Query Embedding**: User questions are converted to vectors using AWS Titan
3. **Similarity Search**: pgvector finds most relevant document chunks
4. **Context Building**: Retrieved content is formatted as context
5. **Answer Generation**: BedrockAPI generates answers using the context

## 📁 Document Management

### Adding Documents
1. Place documents in the `documents/` directory
2. Supported formats: `.md`, `.txt`, `.pdf`, `.docx`
3. Documents are automatically processed on service restart

### Document Processing
- **Chunking**: Large documents are split into manageable chunks
- **Embedding**: Each chunk gets a 1536-dimensional vector representation
- **Storage**: Chunks and embeddings stored in PostgreSQL with pgvector

## 🛠️ Development

### Running Locally
```bash
cd services/RAGAPI

# Start database
docker-compose up -d

# Install dependencies and run
cargo build
cargo run
```

### Testing
```bash
# Health check
curl http://localhost:9101/health

# Statistics
curl http://localhost:9101/stats

# Simple search
curl -X POST http://localhost:9101/search \
  -H "Content-Type: application/json" \
  -d '{"query": "test"}'
```

## 🔗 Integration

### Dependencies
- **BedrockAPI**: For AI chat completion and answer generation
- **PostgreSQL + pgvector**: For document and embedding storage
- **AWS Bedrock**: For document embedding (optional - uses mock if unavailable)

### Service Communication
```
User Query → RAGAPI → Vector Search → Context Retrieval → BedrockAPI → Answer Generation
```

## 📊 Database Schema

### Documents Table
```sql
CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    filename VARCHAR NOT NULL,
    content TEXT NOT NULL,
    file_hash VARCHAR NOT NULL,
    chunk_index INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Embeddings Table
```sql
CREATE TABLE embeddings (
    id SERIAL PRIMARY KEY,
    document_id INTEGER REFERENCES documents(id),
    embedding vector(1536) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## 🔧 Advanced Configuration

### Similarity Threshold
- Adjust `similarity_threshold` in search requests (0.0 to 1.0)
- Lower values return more results, higher values are more restrictive
- Default: 0.1 for most use cases

### Chunking Strategy
- Modify chunking logic in document processing
- Current: Simple text splitting by paragraphs
- Future: Semantic chunking based on embeddings

## 📝 API Specification

### Query Request
```json
{
  "query": "Your question here",
  "system_prompt": "Optional custom system prompt",
  "context": "Optional additional context",
  "max_tokens": 500,
  "temperature": 0.7
}
```

### Query Response
```json
{
  "answer": "AI-generated answer based on retrieved context",
  "sources": [
    {
      "filename": "document.md",
      "chunk_index": 0,
      "similarity": 0.95,
      "snippet": "Relevant text snippet..."
    }
  ],
  "query": "Original query",
  "context_used": "Full context used for answer generation"
}
```

### Search Request
```json
{
  "query": "search terms",
  "limit": 10,
  "similarity_threshold": 0.1
}
```

### Search Response
```json
{
  "documents": [
    {
      "document": {
        "id": 1,
        "filename": "document.md",
        "content": "Full document content",
        "file_hash": "hash_value",
        "chunk_index": 0,
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
      },
      "similarity": 0.95
    }
  ]
}
```

## 🛠️ Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `sqlx` - Database ORM
- `pgvector` - Vector operations
- `aws-sdk-bedrockruntime` - AWS Bedrock client
- `reqwest` - HTTP client for BedrockAPI
- `serde` - Serialization
- `tracing` - Logging

## 🚨 Troubleshooting

### Common Issues

**"No documents found"**
- Check if documents exist in `documents/` directory
- Verify embeddings were generated on startup
- Check database connection and pgvector extension

**"AWS credential errors"**
- Service uses mock embeddings as fallback
- Check AWS credentials if real embeddings needed
- Verify AWS region and permissions

**"Database connection failed"**
- Ensure PostgreSQL is running: `docker-compose ps`
- Check DATABASE_URL in .env file
- Verify pgvector extension is installed

### Logs
```bash
# View service logs
cargo run 2>&1 | tee ragapi.log

# View database logs
docker-compose logs postgres
```

## 📈 Performance

### Optimization Tips
- **Document Chunking**: Adjust chunk size for better retrieval
- **Embedding Dimensions**: 1536 provides good balance of quality vs speed
- **Similarity Threshold**: Tune based on use case requirements
- **Database Indexing**: pgvector uses HNSW indexing for fast search

### Monitoring
```bash
# Service statistics
curl http://localhost:9101/stats

# Response format:
{
  "document_count": 5,
  "embedding_count": 25
}
```

## 🔮 Future Enhancements

- Semantic document chunking
- Multi-language support
- Custom embedding models
- Document versioning
- Real-time document updates
- Advanced retrieval strategies (reranking, hybrid search)

---

**Status**: ✅ Production-ready RAG system with automatic document processing, vector search, and AI-powered Q&A.
