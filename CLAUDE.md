# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

This is a Rust-based microservices RAG (Retrieval-Augmented Generation) system with two main services:

- **BedrockAPI** (`services/BedrockAPI/`): AI chat completion service using AWS Bedrock, runs on port 9100
- **RAGAPI** (`services/RAGAPI/`): Document retrieval and Q&A service with PostgreSQL/pgvector, runs on port 9101

Both services communicate with each other: RAGAPI retrieves relevant document context and sends it to BedrockAPI for answer generation.

## Essential Commands

### Start All Services
```bash
./start.sh              # Start everything (PostgreSQL, BedrockAPI, RAGAPI)
./start.sh all           # Same as above
./start.sh bedrock       # Start only BedrockAPI
./start.sh rag           # Start only RAGAPI
```

### Build Services
```bash
# BedrockAPI
cd services/BedrockAPI && cargo build --release --bin bedrock-chat-api

# RAGAPI  
cd services/RAGAPI && cargo build --release --bin ragapi

# Or use the test build script
cd services/RAGAPI && ./test_build.sh
```

### Database Operations
```bash
cd services/RAGAPI
docker-compose up -d                                                    # Start PostgreSQL with pgvector
psql postgresql://raguser:password@localhost:5434/ragdb < init.sql      # Initialize schema
```

## Key Environment Variables

Both services require `.env` files in their respective directories:

**BedrockAPI/.env:**
- `JWT_SECRET`: JWT authentication secret
- `HOST=127.0.0.1`, `PORT=9100`: Server configuration
- AWS credentials for Bedrock access

**RAGAPI/.env:**
- `DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb`
- `HOST=127.0.0.1`, `PORT=9101`: Server configuration
- `BEDROCK_API_URL=http://127.0.0.1:9100`: BedrockAPI integration
- AWS credentials for embeddings (optional - uses mock if unavailable)

## Service Integration

The RAG pipeline works as follows:
1. User query → RAGAPI
2. RAGAPI embeds query and performs vector similarity search in PostgreSQL
3. RAGAPI retrieves relevant document chunks
4. RAGAPI sends context + query to BedrockAPI
5. BedrockAPI generates answer using AWS Bedrock
6. Response returns to user

## Document Management

- Place documents in `services/RAGAPI/documents/` directory
- Supported formats: `.md`, `.txt`, `.pdf`, `.docx`
- Documents are automatically embedded on service startup
- Database stores document chunks and their vector embeddings using pgvector

## Database Schema

- **documents**: Stores document content and metadata with chunking
- **embeddings**: Stores vector(1536) embeddings for each document chunk
- Uses pgvector extension for similarity search operations

## Binary Names

- BedrockAPI binary: `bedrock-chat-api`
- RAGAPI binary: `ragapi`

## Important Notes

- Both services use consistent port configurations across all components
- Services have comprehensive error handling and fallback mechanisms (mock embeddings if AWS unavailable)
- JWT authentication is implemented in BedrockAPI with default credentials (admin/password)
- PostgreSQL container name is `ragapi_postgres`

## Development Guidelines

**NEVER stub or mock data in the codebase.** This is a production-quality RAG system that should work with real data. Always:
- Use actual document processing and embedding generation
- Implement proper database operations with real data
- Maintain the integrity of the RAG pipeline without shortcuts or placeholder data
- Test with real documents and queries to ensure system reliability