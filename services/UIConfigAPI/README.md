# UIConfigAPI

Configuration and administration management microservice built with Rust, providing comprehensive system management for the RAG system.

## 🚀 Features

- **Environment Configuration**: Secure storage and management of system configurations
- **Document Management**: File upload, processing, and organization
- **Vector Processing**: Background vector generation from document folders
- **RAG Model Management**: Creation and management of RAG models
- **User Management**: Admin tools for user administration
- **System Analytics**: Real-time system statistics and monitoring
- **Security**: JWT authentication and role-based access control
- **Background Processing**: Async task processing for vector generation

## 📡 API Endpoints

### Admin Dashboard
- `GET /admin/overview` - System overview with statistics
- `GET /admin/system-health` - System health status
- `GET /admin/system-stats` - Detailed system statistics
- `GET /admin/users` - User management
- `POST /admin/users` - Create new user
- `PUT /admin/users/{id}` - Update user
- `DELETE /admin/users/{id}` - Delete user

### Configuration Management
- `GET /admin/configs` - List all configurations
- `POST /admin/configs` - Create new configuration
- `PUT /admin/configs/{id}` - Update configuration
- `DELETE /admin/configs/{id}` - Delete configuration

### Document Management
- `GET /documents` - List documents and folders
- `POST /documents/folders` - Create new folder
- `POST /documents/upload` - Upload documents
- `DELETE /documents` - Delete files/folders

### Vector Management
- `GET /vectors` - List all vectors
- `POST /vectors` - Create new vector
- `PUT /vectors/{id}` - Update vector
- `DELETE /vectors/{id}` - Delete vector
- `POST /vectors/{id}/process` - Process folder to vector

### RAG Model Management
- `GET /rag-models` - List RAG models (public endpoint)
- `GET /admin/rag-models` - List RAG models (admin)
- `POST /admin/rag-models` - Create new RAG model
- `PUT /admin/rag-models/{id}` - Update RAG model
- `DELETE /admin/rag-models/{id}` - Delete RAG model

### Monitoring
- `GET /health` - Service health check

## 🔧 Configuration

### Environment Variables (.env)
```bash
# Database Configuration
DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb

# Server Configuration
HOST=127.0.0.1
PORT=9103

# External Services
RAGAPI_URL=http://127.0.0.1:9101
AUTH_API_URL=http://127.0.0.1:9102

# Document Storage
DOCUMENTS_BASE_PATH=/Users/youruser/ai-rust-api/services/RAGAPI/documents

# Security
ENCRYPTION_KEY=your_32_character_encryption_key_here
```

## 🚀 Usage Examples

### System Overview
```bash
# Get system overview (requires admin token)
curl -X GET http://localhost:9103/admin/overview \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN"
```

### Document Management
```bash
# List documents
curl -X GET http://localhost:9103/documents \
  -H "Authorization: Bearer YOUR_TOKEN"

# Create new folder
curl -X POST http://localhost:9103/documents/folders \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"name": "NewDocuments"}'

# Upload document (multipart form)
curl -X POST http://localhost:9103/documents/upload \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -F "file=@document.pdf" \
  -F "folder=MyFolder"
```

### Vector Management
```bash
# Create new vector
curl -X POST http://localhost:9103/vectors \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN" \
  -d '{
    "name": "Scientific Papers",
    "folder_name": "ScienceDocs",
    "description": "Collection of scientific papers"
  }'

# Process folder to vector (background task)
curl -X POST http://localhost:9103/vectors/1/process \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN"
```

### RAG Model Management
```bash
# Create RAG model
curl -X POST http://localhost:9103/admin/rag-models \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN" \
  -d '{
    "name": "Science Assistant",
    "vector_id": 1,
    "system_prompt": "You are a helpful science assistant...",
    "context": "Additional context for the model"
  }'

# List RAG models (public - for regular users)
curl -X GET http://localhost:9103/rag-models \
  -H "Authorization: Bearer YOUR_TOKEN"
```

## 🏗️ Architecture

```
UIConfigAPI
├── src/
│   ├── main.rs                    # Server setup and routing
│   ├── config.rs                  # Configuration management
│   ├── database.rs                # Database operations
│   ├── env_manager.rs             # Environment variable handling
│   ├── handlers/                  # API endpoint handlers
│   │   ├── admin.rs              # Admin dashboard endpoints
│   │   ├── auth.rs               # Authentication handlers
│   │   ├── config.rs             # Configuration endpoints
│   │   ├── documents.rs          # Document management
│   │   ├── rag_models.rs         # RAG model management
│   │   ├── vector_processing.rs  # Vector processing logic
│   │   └── vectors.rs            # Vector management
│   ├── middleware/                # Custom middleware
│   │   └── auth.rs               # Authentication middleware
│   ├── models/                    # Data models
│   │   ├── config.rs             # Configuration models
│   │   ├── document.rs           # Document models
│   │   ├── rag_model.rs          # RAG model models
│   │   ├── user.rs               # User models
│   │   └── vector.rs             # Vector models
│   └── utils/                     # Utility functions
│       ├── encryption.rs         # Configuration encryption
│       └── file_utils.rs         # File handling utilities
├── Cargo.toml                     # Dependencies
├── .env                          # Configuration
└── README.md                     # This documentation
```

## 🔧 Development

### Running Locally
```bash
cd services/UIConfigAPI

# Install dependencies
cargo build

# Run with environment variables
DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb cargo run
```

### Testing
```bash
# Health check
curl http://localhost:9103/health

# Test with authentication
TOKEN=$(curl -X POST http://localhost:9102/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}' | jq -r .token)

# Test admin overview
curl -X GET http://localhost:9103/admin/overview \
  -H "Authorization: Bearer $TOKEN"
```

## 🔗 Integration

### Service Dependencies
- **AuthAPI**: Token validation for all secured endpoints
- **RAGAPI**: Document processing and embedding generation
- **PostgreSQL**: Data storage for configurations, vectors, and models
- **File System**: Document storage and management

### Background Processing
The service uses Tokio for background vector processing:
1. Admin triggers vector processing for a folder
2. Background task spawned using `tokio::spawn`
3. Documents are processed and sent to RAGAPI
4. Vector counts updated in database upon completion

## 📝 API Specification

### Key Request/Response Formats

#### Create Vector Request
```json
{
  "name": "Vector Name",
  "folder_name": "DocumentFolder",
  "description": "Optional description"
}
```

#### Vector Response
```json
{
  "id": 1,
  "name": "Vector Name",
  "folder_name": "DocumentFolder",
  "description": "Optional description",
  "document_count": 10,
  "embedding_count": 150,
  "is_active": true,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

#### Create RAG Model Request
```json
{
  "name": "Model Name",
  "vector_id": 1,
  "system_prompt": "You are a helpful assistant specialized in...",
  "context": "Additional context for the model (optional)"
}
```

#### System Overview Response
```json
{
  "total_users": 5,
  "active_users": 4,
  "admin_users": 1,
  "total_configs": 10,
  "service_status": {
    "auth_api": "healthy",
    "rag_api": "healthy",
    "bedrock_api": "healthy"
  }
}
```

## 🛠️ Dependencies

- `axum` - Web framework with multipart support
- `tokio` - Async runtime for background processing
- `sqlx` - Database toolkit with PostgreSQL support
- `reqwest` - HTTP client for service communication
- `serde` - Serialization/deserialization
- `uuid` - UUID generation
- `bcrypt` - Password hashing
- `aes-gcm` - Configuration encryption
- `multer` - Multipart form handling
- `pdf-extract` - PDF text extraction
- `regex` - Regular expressions
- `base64` - Base64 encoding/decoding
- `rand` - Random number generation
- `tracing` - Comprehensive logging

## 🚨 Security Features

### Authentication & Authorization
- JWT token validation for all protected endpoints
- Role-based access control (admin vs regular users)
- Token verification with AuthAPI

### Configuration Security
- AES-GCM encryption for sensitive configurations
- Encrypted storage in PostgreSQL
- Key rotation support

### File Security
- Secure file upload handling
- Path traversal protection
- File type validation

## 📊 Monitoring & Analytics

### System Health Monitoring
```bash
curl http://localhost:9103/admin/system-health
# Returns detailed health status of all services
```

### System Statistics
```bash
curl http://localhost:9103/admin/system-stats
# Returns comprehensive system metrics
```

## 🔄 Vector Processing Workflow

1. **Folder Preparation**: Admin creates document folder
2. **Vector Creation**: Admin creates vector linked to folder
3. **Processing Trigger**: Admin initiates vector processing
4. **Background Task**: System spawns async processing task
5. **Document Discovery**: System scans folder for supported files
6. **Text Extraction**: PDF text extracted using pdf-extract
7. **Document Chunking**: Content split into optimized chunks
8. **RAGAPI Integration**: Chunks sent to RAGAPI for embedding
9. **Progress Tracking**: Database updated with processing results
10. **Completion**: Vector ready for RAG model creation

## 📈 Performance Features

- **Async Processing**: Non-blocking background tasks
- **Connection Pooling**: Efficient database connections
- **Chunk Optimization**: 1000-character chunks with 25% overlap
- **Parallel Processing**: Multiple documents processed concurrently
- **Error Recovery**: Robust error handling with continuation
- **Progress Tracking**: Real-time processing status updates

This service is the administrative backbone of the RAG system, providing comprehensive management capabilities while maintaining security and performance.