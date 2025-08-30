# BedrockAPI

AI chat completion microservice built with Rust, providing JWT-authenticated access to AWS Bedrock models.

## 🚀 Features

- **JWT Token Validation**: Validates tokens issued by AuthAPI service
- **AWS Bedrock Integration**: Access to Claude and other Bedrock models  
- **REST API**: Clean HTTP endpoints for chat completion
- **Standardized Error Handling**: Consistent error response format
- **Health Checks**: Service monitoring endpoint
- **Service Integration**: Seamlessly integrated with AuthAPI for authentication

## 📡 API Endpoints

### Chat Completion
- `POST /chat` - Standard chat with conversation ID
- `POST /simple-chat` - Advanced chat with configurable parameters

### Monitoring  
- `GET /health` - Service health check

**Note**: Authentication is now handled by AuthAPI (port 9102). Get your JWT token from AuthAPI first.

## 🔧 Configuration

### Environment Variables (.env)
```bash
# JWT Configuration (must match AuthAPI)
JWT_SECRET=your_super_secret_jwt_key_here

# Server Configuration
HOST=127.0.0.1
PORT=9100

# AuthAPI Integration
AUTH_API_URL=http://127.0.0.1:9102

# AWS Bedrock
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_REGION=us-east-1
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
curl -X POST http://localhost:9100/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "message": "Write a haiku about programming"
  }'
```

### Advanced Chat with Parameters
```bash
curl -X POST http://localhost:9100/simple-chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "prompt": "Explain quantum computing in simple terms",
    "max_tokens": 500,
    "temperature": 0.7,
    "top_p": 0.9
  }'
```

## 🏗️ Architecture

```
BedrockAPI
├── src/
│   ├── main.rs          # Server setup and routing
│   ├── auth.rs          # JWT authentication logic
│   └── error.rs         # Error handling
├── Cargo.toml           # Dependencies
├── .env                 # Configuration
└── README.md           # This documentation
```

## 🔧 Development

### Running Locally
```bash
cd services/BedrockAPI

# Install dependencies
cargo build

# Run with environment variables
cargo run
```

### Testing
```bash
# Health check
curl http://localhost:9100/health

# Simple chat test
curl -X POST http://localhost:9100/simple-chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello!"}'
```

## 🔗 Integration

This service is part of the AI-Rust-API microservices architecture:

- **AuthAPI (9102)**: Handles authentication and issues JWT tokens
- **RAGAPI (9101)**: Sends document context and queries to BedrockAPI for answer generation  
- **UIConfigAPI (9103)**: Provides admin interface and system configuration
- **React Frontend**: Complete user interface for all system features

## 📝 API Specification

### Request/Response Format

#### Simple Chat Request
```json
{
  "message": "Your message here",
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

#### Authenticated Chat Request
```json
{
  "message": "Your message here",
  "max_tokens": 500,
  "temperature": 0.7
}
```

#### Authenticated Chat Response
```json
{
  "response": "AI-generated response text",
  "token_count": 150,
  "model": "claude-3-sonnet"
}
```

## 🛠️ Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `aws-sdk-bedrockruntime` - AWS Bedrock client
- `jsonwebtoken` - JWT handling
- `serde` - Serialization
- `tracing` - Logging

## 🚨 Security Notes

- Change the default JWT secret in production
- Use strong passwords for authentication
- Consider implementing rate limiting
- Validate AWS credentials and permissions

## 📊 Monitoring

The service provides health check endpoint for monitoring:

```bash
curl http://localhost:9100/health
# Returns: {"status":"healthy","service":"bedrock-chat-api"}
```
