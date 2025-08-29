# AI-Rust-API - Complete RAG System

A comprehensive Retrieval-Augmented Generation (RAG) system built with Rust microservices and React frontend, featuring automatic document embedding, vector similarity search, AI-powered question answering, and complete user management with admin interface.

## 🏗️ Architecture

This project consists of five main components:

### 🤖 BedrockAPI (`/services/BedrockAPI/`)
- **Purpose**: AI chat completion service using AWS Bedrock
- **Technology**: Rust, Axum, AWS Bedrock
- **Port**: 9100
- **Features**:
  - Chat completion via AWS Bedrock (Claude, etc.)
  - REST API endpoints for AI interactions
  - Integration with RAG system

### 🔍 RAGAPI (`/services/RAGAPI/`)
- **Purpose**: Document retrieval and Q&A service with vector search
- **Technology**: Rust, Axum, PostgreSQL, pgvector
- **Port**: 9101
- **Features**:
  - Document processing with PDF text extraction
  - Vector embeddings using AWS Titan
  - Vector similarity search with pgvector
  - RAG model management with vector isolation
  - Document chunking and storage

### 🔐 AuthAPI (`/services/AuthAPI/`)
- **Purpose**: User authentication and authorization service
- **Technology**: Rust, Axum, PostgreSQL
- **Port**: 9102
- **Features**:
  - JWT-based authentication
  - User management (registration, login)
  - Role-based access control (admin/user)
  - Token validation for other services

### ⚙️ UIConfigAPI (`/services/UIConfigAPI/`)
- **Purpose**: Configuration and admin management service
- **Technology**: Rust, Axum, PostgreSQL
- **Port**: 9103
- **Features**:
  - Environment configuration management
  - Document management and uploads
  - Vector processing and management
  - RAG model creation and management
  - Admin dashboard APIs
  - Background vector processing

### 🎨 User Interface (`/useragent/`)
- **Purpose**: React-based web interface for RAG system
- **Technology**: React.js, Material-UI
- **Port**: 3000
- **Features**:
  - User authentication and registration
  - Interactive RAG chat interface
  - Admin dashboard for system management
  - Document upload and management
  - Vector and RAG model management
  - Real-time system statistics

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
- `POST /process-document` - Process document for embedding

### BedrockAPI (Port 9100)
- `POST /chat` - Chat completion via AWS Bedrock
- `POST /simple-chat` - Simple chat without auth
- `GET /health` - Health check

### AuthAPI (Port 9102)
- `POST /register` - User registration
- `POST /login` - User login
- `POST /validate` - Token validation
- `GET /health` - Health check

### UIConfigAPI (Port 9103)
- `GET /admin/overview` - Admin system overview
- `GET /admin/users` - User management
- `POST /admin/configs` - Environment configuration
- `GET /documents` - Document management
- `POST /vectors` - Vector creation and management
- `GET /rag-models` - RAG model management
- `GET /health` - Health check

### User Interface (Port 3000)
- Web-based React interface for all system operations
- Admin dashboard for system management
- User-friendly RAG chat interface

## 🔧 Configuration

### Environment Variables
Create `.env` files in each service directory:

**AuthAPI/.env:**
```bash
# Database
DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb

# Server
HOST=127.0.0.1
PORT=9102

# JWT
JWT_SECRET=your_super_secret_jwt_key_here
```

**BedrockAPI/.env:**
```bash
# Server
HOST=127.0.0.1
PORT=9100

# AWS Bedrock
AWS_ACCESS_KEY_ID=your_key
AWS_SECRET_ACCESS_KEY=your_secret
AWS_REGION=us-east-1
```

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

**UIConfigAPI/.env:**
```bash
# Database
DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb

# Server
HOST=127.0.0.1
PORT=9103

# External Services
RAGAPI_URL=http://127.0.0.1:9101
AUTH_API_URL=http://127.0.0.1:9102

# Document Storage
DOCUMENTS_BASE_PATH=/path/to/documents
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
├── README.md                    # This comprehensive overview
├── CLAUDE.md                    # Development guidelines and instructions
├── start.sh                     # Master start script for all services
├── services/                    # Backend microservices
│   ├── AuthAPI/                # Authentication service (Port 9102)
│   │   ├── README.md           # AuthAPI documentation
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── .env
│   ├── BedrockAPI/             # AI chat completion service (Port 9100)
│   │   ├── README.md           # BedrockAPI documentation
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── .env
│   ├── RAGAPI/                 # RAG document service (Port 9101)
│   │   ├── README.md           # RAGAPI documentation
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   ├── docker-compose.yml  # PostgreSQL with pgvector
│   │   ├── init.sql           # Database initialization
│   │   ├── documents/         # Document storage
│   │   │   ├── DistributedSystems/
│   │   │   └── Microservices/
│   │   └── .env
│   ├── UIConfigAPI/            # Configuration service (Port 9103)
│   │   ├── README.md           # UIConfigAPI documentation
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── .env
│   └── Database/               # Shared PostgreSQL data
└── useragent/                  # React frontend (Port 3000)
    ├── README.md               # Frontend documentation
    ├── src/
    ├── package.json
    └── .env
```

## 🔄 How the Complete RAG System Works

### Document Processing Pipeline
1. **Document Upload**: Users upload documents via web interface
2. **Vector Creation**: Admin creates vectors for document folders
3. **Document Processing**: UIConfigAPI processes documents and sends to RAGAPI
4. **Text Extraction**: RAGAPI extracts text (PDF support with pdf-extract)
5. **Document Chunking**: Content split into 1000-character chunks with overlap
6. **Embedding Generation**: AWS Titan creates vector embeddings for each chunk
7. **Storage**: Document chunks and embeddings stored in PostgreSQL with pgvector

### RAG Query Pipeline
1. **User Authentication**: AuthAPI validates user tokens
2. **Query Submission**: User submits question via web interface
3. **RAG Model Selection**: User selects specific RAG model (vector isolation)
4. **Query Embedding**: RAGAPI converts question to embedding
5. **Vector Search**: pgvector finds top 10 most similar document chunks
6. **Context Building**: Retrieved chunks formatted as context
7. **Answer Generation**: BedrockAPI generates response using AWS Bedrock
8. **Response Display**: Answer shown with source attributions

### Vector Isolation
- Each vector represents documents from a specific folder
- RAG models are linked to specific vectors
- Queries only search within the selected vector's documents
- Complete isolation between different document collections

## 🛠️ Development

### Adding Documents
1. Create folder in `services/RAGAPI/documents/`
2. Upload documents (PDF, TXT, MD, DOCX supported)
3. Use admin interface to create vector for the folder
4. Create RAG model linked to the vector
5. Documents are automatically processed and embedded

### API Testing
```bash
# Test complete authentication flow
curl -X POST http://localhost:9102/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'

# Test RAG query with authentication
curl -X POST http://localhost:9101/query \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"query": "What is microservices architecture?", "rag_model_id": 1}'

# Test document processing
curl -X POST http://localhost:9101/process-document \
  -H "Content-Type: application/json" \
  -d '{"filename": "test.txt", "content": "Test content"}'
```

### Web Interface Access
```bash
# Start frontend
cd useragent
npm start

# Access web interface
open http://localhost:3000

# Default admin credentials:
# Username: admin
# Password: password
```

## 📝 License

This project is for educational and demonstration purposes.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 🎯 Key Features

### ✅ Complete RAG Implementation
- Real PDF text extraction with pdf-extract
- Document chunking with overlap for optimal retrieval
- Vector isolation for independent document collections
- AWS Titan embeddings with pgvector storage
- Top-10 chunk retrieval with similarity scoring

### 🔒 Security & Authentication
- JWT-based authentication system
- Role-based access control (admin/user)
- Token validation across all services
- Secure configuration management

### 🎨 User Experience
- Modern React web interface with Material-UI
- Interactive RAG chat with source attribution
- Comprehensive admin dashboard
- Real-time system statistics and monitoring

### 🏗️ Production Architecture
- Five independent microservices
- PostgreSQL with pgvector for scalable storage
- Background processing for vector generation
- Comprehensive error handling and logging
- Health checks for all services

---

**Status**: ✅ Production-ready RAG system with complete web interface, vector isolation, real document processing, and comprehensive admin tools.