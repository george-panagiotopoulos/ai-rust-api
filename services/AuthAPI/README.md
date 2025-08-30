# AuthAPI

User authentication and authorization microservice built with Rust, providing JWT-based authentication for the RAG system.

## 🚀 Features

- **JWT Authentication**: Secure token-based authentication with configurable expiration
- **User Management**: Registration, login, profile management, logout
- **Role-Based Access Control**: Admin and regular user roles with permissions
- **Token Validation**: Service-to-service token verification for all microservices
- **Session Management**: User activity tracking and authentication statistics
- **Password Security**: BCrypt hashing with secure password storage
- **Standardized Error Handling**: Consistent `{error: {code, message, timestamp}}` format
- **Database Integration**: PostgreSQL storage with user data persistence
- **Health Checks**: Comprehensive service monitoring endpoint

## 📡 API Endpoints

### Authentication
- `POST /login` - User authentication with JWT token generation
- `POST /register` - Create new user account  
- `POST /logout` - Invalidate JWT token (authenticated)
- `POST /validate` - Validate JWT tokens (used by other services)

### User Management  
- `GET /profile` - Get current user profile (authenticated)
- `GET /stats` - Get authentication service statistics

### Monitoring
- `GET /health` - Service health check

## 🔧 Configuration

### Environment Variables (.env)
```bash
# Database Configuration
DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb

# Server Configuration
HOST=127.0.0.1
PORT=9102

# JWT Configuration
JWT_SECRET=your_super_secret_jwt_key_here
JWT_EXPIRATION_HOURS=24
```

### Default Admin Account
- **Username**: admin
- **Email**: admin@example.com
- **Password**: password
- **Role**: admin

## 🚀 Usage Examples

### User Registration
```bash
curl -X POST http://localhost:9102/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "mypassword123"
  }'
```

### User Login
```bash
# Login and save token
TOKEN=$(curl -X POST http://localhost:9102/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}' | jq -r .token)

echo "Token: $TOKEN"
```

### Token Validation
```bash
curl -X POST http://localhost:9102/validate \
  -H "Content-Type: application/json" \
  -d '{"token": "'$TOKEN'"}'
```

## 🏗️ Architecture

```
AuthAPI
├── src/
│   ├── main.rs          # Server setup and routing
│   ├── auth.rs          # JWT authentication logic
│   ├── handlers.rs      # API endpoint handlers
│   ├── models.rs        # Data models
│   ├── database.rs      # Database operations
│   ├── error.rs         # Error handling
│   └── config.rs        # Configuration management
├── Cargo.toml           # Dependencies
├── .env                 # Configuration
└── README.md           # This documentation
```

## 🔧 Development

### Running Locally
```bash
cd services/AuthAPI

# Install dependencies
cargo build

# Run with environment variables
cargo run
```

### Testing
```bash
# Health check
curl http://localhost:9102/health

# Test registration
curl -X POST http://localhost:9102/register \
  -H "Content-Type: application/json" \
  -d '{"username": "test", "email": "test@test.com", "password": "test123"}'

# Test login
curl -X POST http://localhost:9102/login \
  -H "Content-Type: application/json" \
  -d '{"username": "test", "password": "test123"}'
```

## 🔗 Integration

AuthAPI is the central authentication service for the AI-Rust-API microservices system:

- **BedrockAPI (9100)**: Validates JWT tokens for AI chat completion endpoints
- **RAGAPI (9101)**: Validates JWT tokens for RAG queries and document operations
- **UIConfigAPI (9103)**: Validates admin tokens for management operations and system configuration
- **React Frontend**: Provides authentication UI and token management for all user interactions

## 📝 API Specification

### Request/Response Format

#### Register Request
```json
{
  "username": "string",
  "email": "string",
  "password": "string"
}
```

#### Register Response
```json
{
  "success": true,
  "message": "User registered successfully",
  "user_id": 123
}
```

#### Login Request
```json
{
  "username": "string",
  "password": "string"
}
```

#### Login Response
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": 1,
    "username": "admin",
    "email": "admin@example.com",
    "is_admin": true,
    "is_active": true
  }
}
```

#### Token Validation Request
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

#### Token Validation Response
```json
{
  "valid": true,
  "user": {
    "id": 1,
    "username": "admin",
    "email": "admin@example.com",
    "is_admin": true,
    "is_active": true
  }
}
```

## 🛠️ Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `sqlx` - Database toolkit with PostgreSQL support
- `bcrypt` - Password hashing
- `jsonwebtoken` - JWT handling
- `serde` - Serialization
- `chrono` - Date/time handling
- `uuid` - UUID generation
- `tracing` - Logging

## 🚨 Security Notes

- **JWT Secret**: Use a strong, randomly generated JWT secret in production
- **Password Policy**: Implement password complexity requirements
- **Rate Limiting**: Consider implementing rate limiting for authentication endpoints
- **HTTPS**: Always use HTTPS in production
- **Token Expiration**: Configure appropriate token expiration times
- **Database Security**: Ensure database connections are encrypted

## 📊 Monitoring

The service provides comprehensive health checks:

```bash
curl http://localhost:9102/health
# Returns: {"status":"healthy","service":"auth-api","database":"connected"}
```

## 🗄️ Database Schema

### Users Table
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 🔄 Token Lifecycle

1. **Registration/Login**: User provides credentials
2. **Validation**: Credentials verified against database
3. **Token Generation**: JWT token created with user claims
4. **Token Distribution**: Token sent to client
5. **Token Usage**: Client includes token in API requests
6. **Token Validation**: Other services validate token via `/validate` endpoint
7. **Token Expiration**: Tokens expire after configured time period