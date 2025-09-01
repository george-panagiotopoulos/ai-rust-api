# AzureAPI

Azure OpenAI chat completion and embeddings microservice built with Rust, providing JWT-authenticated access to Azure OpenAI models.

## 🚀 Features

- **JWT Token Validation**: Validates tokens issued by AuthAPI service
- **Azure OpenAI Integration**: Access to GPT-4, GPT-3.5, and Ada embeddings
- **REST API**: Clean HTTP endpoints for chat completion and embeddings  
- **Standardized Error Handling**: Consistent error response format
- **Health Checks**: Service monitoring endpoint
- **Service Integration**: Seamlessly integrated with AuthAPI for authentication

## 📡 API Endpoints

### Chat Completion
- `POST /chat` - Standard chat with conversation ID
- `POST /simple-chat` - Advanced chat with configurable parameters

### Embeddings
- `POST /embeddings` - Generate embeddings using Azure OpenAI

### Monitoring  
- `GET /health` - Service health check

**Note**: Authentication is handled by AuthAPI (port 9102). Get your JWT token from AuthAPI first.

## 🔧 Configuration

### Environment Variables (.env)
```bash
# JWT Configuration (must match AuthAPI)
JWT_SECRET=your_super_secret_jwt_key_here

# Server Configuration
HOST=127.0.0.1
PORT=9104

# AuthAPI Integration
AUTH_API_URL=http://127.0.0.1:9102

# Azure OpenAI Configuration
AZURE_OPENAI_API_KEY=your_azure_api_key_here
AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com
AZURE_OPENAI_API_VERSION=2024-02-15-preview
AZURE_LLM_DEPLOYMENT_NAME=gpt-4
AZURE_EMBEDDING_DEPLOYMENT_NAME=text-embedding-ada-002
```

### Authentication
Get JWT tokens from AuthAPI (port 9102):
- **Default Admin**: username=admin, password=password
- **Endpoint**: `POST http://localhost:9102/login`

## 🚀 Usage Examples

### Get Authentication Token (from AuthAPI)
```bash
# Get JWT token from AuthAPI
TOKEN=$(curl -X POST http://localhost:9102/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}' | jq -r .access_token)
```

### Standard Chat
```bash
curl -X POST http://localhost:9104/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "message": "Write a haiku about programming"
  }'
```

### Advanced Chat with Parameters
```bash
curl -X POST http://localhost:9104/simple-chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "prompt": "Explain quantum computing in simple terms",
    "max_tokens": 500,
    "temperature": 0.7,
    "top_p": 0.9
  }'
```

### Generate Embeddings
```bash
curl -X POST http://localhost:9104/embeddings \
  -H "Content-Type: application/json" \
  -d '{
    "text": "This is a sample text for embedding generation"
  }'
```

## 🏗️ Architecture

```
AzureAPI
├── src/
│   ├── main.rs          # Server setup and routing
│   ├── azure_client.rs  # Azure OpenAI integration
│   ├── auth_client.rs   # AuthAPI integration
│   ├── handlers.rs      # API endpoint handlers
│   ├── config.rs        # Configuration management
│   └── error.rs         # Error handling
├── Cargo.toml           # Dependencies
├── .env                 # Configuration
└── README.md           # This documentation
```

## 🔧 Development

### Running Locally
```bash
cd services/AzureAPI

# Install dependencies
cargo build --release --bin azure-chat-api

# Run with environment variables
cargo run
```

### Testing
```bash
# Health check
curl http://localhost:9104/health

# Test with authentication
TOKEN=$(curl -X POST http://localhost:9102/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}' | jq -r .access_token)

# Test chat
curl -X POST http://localhost:9104/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"message": "Hello!"}'
```

## 🔗 Integration

This service is part of the AI-Rust-API microservices architecture:

- **AuthAPI (9102)**: Handles authentication and issues JWT tokens
- **RAGAPI (9101)**: Can use AzureAPI for embeddings and chat completion
- **UIConfigAPI (9103)**: Provides admin interface and backend selection
- **BedrockAPI (9100)**: Alternative AI provider for AWS Bedrock
- **React Frontend**: Complete user interface for all system features

## 📝 API Specification

### Request/Response Format

#### Standard Chat Request
```json
{
  "message": "Your message here"
}
```

#### Standard Chat Response
```json
{
  "id": "uuid-string",
  "response": "AI-generated response text"
}
```

#### Simple Chat Request
```json
{
  "prompt": "Your prompt here",
  "max_tokens": 500,
  "temperature": 0.7,
  "top_p": 0.9
}
```

#### Simple Chat Response
```json
{
  "response": "AI-generated response text",
  "token_count": 150
}
```

#### Embedding Request
```json
{
  "text": "Text to generate embedding for"
}
```

#### Embedding Response
```json
{
  "embedding": [0.1, 0.2, 0.3, ...],
  "dimension": 1536
}
```

#### Error Response Format
```json
{
  "error": {
    "code": "AZURE_ERROR",
    "message": "Detailed error message",
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

## 🛠️ Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `reqwest` - HTTP client for Azure OpenAI API
- `jsonwebtoken` - JWT handling
- `serde` - Serialization
- `tracing` - Logging
- `uuid` - UUID generation
- `chrono` - Date/time handling

## 🚨 Security Notes

- **Change JWT secrets** to match other services
- **Secure Azure API keys** and rotate regularly
- **Use HTTPS** in production environments
- **Monitor API usage** and implement rate limiting
- **Validate Azure credentials** and permissions

## 📊 Monitoring

Health check endpoint for service monitoring:

```bash
curl http://localhost:9104/health
# Returns: {"status":"healthy","service":"AzureAPI","version":"0.1.0"}
```

## 🔄 Azure OpenAI Integration

- **Chat Completions**: Uses Azure OpenAI Chat Completions API
- **Embeddings**: Uses Azure OpenAI Embeddings API
- **API Versioning**: Supports latest Azure OpenAI API versions
- **Model Deployments**: Configurable deployment names for different models
- **Error Handling**: Comprehensive Azure-specific error handling
- **Token Counting**: Usage tracking for cost monitoring

## 🌐 Deployment Considerations

- **Azure Resource**: Requires Azure OpenAI resource
- **API Keys**: Secure storage of Azure API keys
- **Quotas**: Monitor Azure OpenAI quotas and limits
- **Regions**: Choose appropriate Azure region for latency
- **Models**: Deploy required models (GPT-4, Ada-002) in Azure