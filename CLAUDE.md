# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

This is a comprehensive Rust-based microservices RAG (Retrieval-Augmented Generation) system with four main services:

- **AuthAPI** (`services/AuthAPI/`): JWT-based authentication and user management, runs on port 9102
- **BedrockAPI** (`services/BedrockAPI/`): AI chat completion service using AWS Bedrock, runs on port 9100  
- **RAGAPI** (`services/RAGAPI/`): Document retrieval and Q&A service with PostgreSQL/pgvector, runs on port 9101
- **UIConfigAPI** (`services/UIConfigAPI/`): Configuration and administration service with React frontend, runs on port 9103

All services are fully integrated with standardized error handling, JWT authentication, and comprehensive API documentation.

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
# AuthAPI
cd services/AuthAPI && cargo build --release --bin auth-api

# BedrockAPI
cd services/BedrockAPI && cargo build --release --bin bedrock-chat-api

# RAGAPI  
cd services/RAGAPI && cargo build --release --bin ragapi

# UIConfigAPI
cd services/UIConfigAPI && cargo build --release --bin ui-config-api

# React Frontend
cd useragent && npm install && npm run build
```

### Database Operations
```bash
cd services/RAGAPI
docker-compose up -d                                                    # Start PostgreSQL with pgvector
psql postgresql://raguser:password@localhost:5434/ragdb < init.sql      # Initialize schema
```

## Key Environment Variables

All services require `.env` files in their respective directories:

**AuthAPI/.env:**
- `JWT_SECRET`: JWT authentication secret
- `DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb`
- `HOST=127.0.0.1`, `PORT=9102`: Server configuration

**BedrockAPI/.env:**
- `JWT_SECRET`: JWT authentication secret
- `HOST=127.0.0.1`, `PORT=9100`: Server configuration
- `AUTH_API_URL=http://127.0.0.1:9102`: AuthAPI integration
- AWS credentials for Bedrock access

**RAGAPI/.env:**
- `DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb`
- `HOST=127.0.0.1`, `PORT=9101`: Server configuration
- `BEDROCK_API_URL=http://127.0.0.1:9100`: BedrockAPI integration
- `AUTH_API_URL=http://127.0.0.1:9102`: AuthAPI integration
- AWS credentials for embeddings (optional - uses mock if unavailable)

**UIConfigAPI/.env:**
- `DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb`
- `HOST=127.0.0.1`, `PORT=9103`: Server configuration
- `AUTH_API_URL=http://127.0.0.1:9102`: AuthAPI integration

## Service Integration

The complete system architecture works as follows:

### Authentication Flow:
1. User authenticates through React frontend or API → AuthAPI
2. AuthAPI validates credentials and issues JWT token
3. All subsequent API calls require Bearer token authentication

### RAG Pipeline:
1. User query → RAGAPI (authenticated)
2. RAGAPI embeds query and performs vector similarity search in PostgreSQL
3. RAGAPI retrieves relevant document chunks using RAG model names (e.g., "Microservices", "DistributedSystems")
4. RAGAPI sends context + query to BedrockAPI
5. BedrockAPI generates answer using AWS Bedrock
6. Response returns to user

### Admin Management:
1. Admin users access management features through React frontend
2. UIConfigAPI provides document management, user management, configuration, and system monitoring
3. All admin operations are JWT-authenticated and require admin privileges

## Document Management

Multiple approaches for document management:

### Filesystem-based (RAGAPI):
- Place documents in `services/RAGAPI/documents/` directory
- Supported formats: `.md`, `.txt`, `.pdf`, `.docx`
- Documents are automatically embedded on service startup

### Admin UI-based (UIConfigAPI):
- Upload documents through React admin interface
- Create document folders and organize documents
- Support for multipart file uploads
- Document processing and embedding generation

## Database Schema

Extended schema supporting full system features:

- **users**: User accounts with authentication data, admin flags, and activity tracking
- **documents**: Document content and metadata with chunking support
- **embeddings**: Vector(1536) embeddings for each document chunk using pgvector
- **rag_models**: Named RAG models (e.g., "Microservices", "DistributedSystems") with system prompts and contexts
- **vectors**: Vector configurations linking documents to processing pipelines
- **configs**: System configuration storage for environment variables and settings

## Binary Names

- AuthAPI binary: `auth-api`
- BedrockAPI binary: `bedrock-chat-api`
- RAGAPI binary: `ragapi`
- UIConfigAPI binary: `ui-config-api`

## Recent Features & Improvements

### System Consistency (2024):
- **Standardized Error Handling**: All APIs now use consistent `{error: {code, message, timestamp}}` format
- **RAG Model Names**: Changed from numeric IDs to meaningful names ("Microservices", "DistributedSystems")
- **Complete API Documentation**: 60+ endpoints fully documented with request/response schemas
- **Postman Collection**: Comprehensive collection with JWT auto-extraction and all endpoints

### Frontend Features:
- **React Admin Dashboard**: Complete admin interface with Material-UI components
- **API Documentation Component**: Interactive documentation with endpoint explorer
- **User Management**: Full CRUD operations for user accounts and permissions
- **Document Management**: Upload, organize, and manage documents through UI
- **Configuration Management**: Environment variable and system configuration management

### Authentication & Security:
- **JWT-based Authentication**: Centralized auth service with token validation
- **Role-based Access Control**: Admin/user role separation
- **Secure API Endpoints**: All sensitive operations require authentication
- **Session Management**: User activity tracking and session management

## Important Notes

- All four services use consistent port configurations and service integration
- Comprehensive error handling and fallback mechanisms throughout
- JWT authentication with proper token validation across all services
- PostgreSQL container name is `ragapi_postgres`
- Default admin credentials: admin/password (change in production)
- Full system supports both development and production deployments

## Development Guidelines

**NEVER stub or mock data in the codebase.** This is a production-quality RAG system that should work with real data. Always:
- Use actual document processing and embedding generation
- Implement proper database operations with real data
- Maintain the integrity of the RAG pipeline without shortcuts or placeholder data
- Test with real documents and queries to ensure system reliability