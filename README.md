# AI-Rust-API - RAG System

A complete Retrieval-Augmented Generation (RAG) system built with Rust microservices, featuring automatic document embedding, vector similarity search, and AI-powered question answering.

## 🏗️ Architecture

This project consists of two main microservices:

### 🤖 BedrockAPI (`/services/BedrockAPI/`)
- **Purpose**: AI chat completion service
- **Technology**: Rust, Axum, AWS Bedrock
- **Port**: 9100
- **Features**:
  - JWT-based authentication
  - Chat completion via AWS Bedrock
  - REST API endpoints

### 🔍 RAGAPI (`/services/RAGAPI/`)
- **Purpose**: Document retrieval and Q&A service
- **Technology**: Rust, Axum, PostgreSQL, pgvector
- **Port**: 9101
- **Features**:
  - Automatic document embedding generation
  - Vector similarity search
  - RAG-powered question answering
  - Document storage and retrieval

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- Docker & Docker Compose
- AWS credentials (for Bedrock access)

### Setup & Run
```bash
# Clone and navigate to project
cd ai-rust-api

# Start all services (database, BedrockAPI, RAGAPI)
./start.sh

# Or start services individually:
./start.sh bedrock    # Start only BedrockAPI
./start.sh rag        # Start only RAGAPI
./start.sh all        # Start everything (default)
```

## 📡 API Endpoints

### RAGAPI (Port 9101)
- `GET /health` - Health check
- `GET /stats` - System statistics
- `POST /query` - Ask questions with RAG
- `POST /search` - Search documents by similarity

### BedrockAPI (Port 9100)
- `POST /oauth/token` - Get JWT token
- `POST /chat` - Chat completion
- `POST /simple-chat` - Simple chat without auth

## 🔧 Configuration

### Environment Variables
Create `.env` files in each service directory:

**RAGAPI/.env:**
```bash
# Database
DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb

# Server
HOST=127.0.0.1
PORT=9101

# BedrockAPI Service
BEDROCK_API_URL=http://127.0.0.1:9100

# AWS (for embeddings - optional, uses mock if unavailable)
AWS_ACCESS_KEY_ID=your_key
AWS_SECRET_ACCESS_KEY=your_secret
AWS_REGION=us-east-1
```

**BedrockAPI/.env:**
```bash
# JWT Secret
JWT_SECRET=your_jwt_secret_here

# Server
HOST=127.0.0.1
PORT=9100

# AWS Bedrock
AWS_ACCESS_KEY_ID=your_key
AWS_SECRET_ACCESS_KEY=your_secret
AWS_REGION=us-east-1
```

### Database Setup
```bash
# Start PostgreSQL with pgvector
cd services/RAGAPI
docker-compose up -d

# Initialize database
psql postgresql://raguser:password@localhost:5434/ragdb < init.sql
```

## 📁 Project Structure

```
ai-rust-api/
├── README.md                    # This overview
├── start.sh                     # Start script for all services
├── services/
│   ├── BedrockAPI/             # AI chat completion service
│   │   ├── README.md           # BedrockAPI documentation
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── .env
│   └── RAGAPI/                 # RAG document Q&A service
│       ├── README.md           # RAGAPI documentation
│       ├── src/
│       ├── Cargo.toml
│       ├── docker-compose.yml
│       ├── init.sql
│       └── .env
```

## 🔄 How RAG Works

1. **Document Ingestion**: Documents are automatically embedded using AWS Titan
2. **Query Processing**: User questions are converted to embeddings
3. **Similarity Search**: Vector search finds most relevant documents
4. **Answer Generation**: AI generates answers using retrieved context

## 🛠️ Development

### Adding Documents
Place documents in `services/RAGAPI/documents/` - they will be automatically embedded on service start.

### API Testing
```bash
# Test RAGAPI
curl -X POST http://localhost:9101/query \
  -H "Content-Type: application/json" \
  -d '{"query": "What is AI?"}'

# Test BedrockAPI
curl -X POST http://localhost:9100/simple-chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello!"}'
```

## 📝 License

This project is for educational and demonstration purposes.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

---

**Status**: ✅ Fully functional RAG system with working embeddings, vector search, and AI-powered Q&A.