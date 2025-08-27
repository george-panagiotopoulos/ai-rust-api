# BedrockAPI

AI chat completion microservice built with Rust, providing JWT-authenticated access to AWS Bedrock models.

## 🚀 Features

- **JWT Authentication**: Secure token-based authentication
- **AWS Bedrock Integration**: Access to Claude and other Bedrock models
- **REST API**: Clean HTTP endpoints for chat completion
- **Error Handling**: Comprehensive error responses
- **Health Checks**: Service monitoring endpoint

## 📡 API Endpoints

### Authentication
- `POST /oauth/token` - Get JWT access token
  ```bash
  curl -X POST http://localhost:9100/oauth/token \
    -H "Content-Type: application/json" \
    -d '{"username": "admin", "password": "password"}'
  ```

### Chat Completion
- `POST /chat` - Authenticated chat completion
- `POST /simple-chat` - Simple chat without authentication

### Monitoring
- `GET /health` - Service health check

## 🔧 Configuration

### Environment Variables (.env)
```bash
# JWT Configuration
JWT_SECRET=your_super_secret_jwt_key_here

# Server Configuration
HOST=127.0.0.1
PORT=9100

# AWS Bedrock
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_REGION=us-east-1
```

### Default Credentials
- **Username**: admin
- **Password**: password

## 🚀 Usage Examples

### Simple Chat (No Auth)
```bash
curl -X POST http://localhost:9100/simple-chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Explain quantum computing in simple terms",
    "max_tokens": 500,
    "temperature": 0.7
  }'
```

### Authenticated Chat
```bash
# 1. Get token
TOKEN=$(curl -X POST http://localhost:9100/oauth/token \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}' | jq -r .access_token)

# 2. Use token for chat
curl -X POST http://localhost:9100/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "message": "Write a haiku about programming",
    "max_tokens": 200
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

This service is designed to work with RAGAPI for providing AI chat completion capabilities. The RAGAPI service communicates with BedrockAPI for generating answers based on retrieved document context.

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
