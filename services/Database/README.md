# Database Management

Complete database setup, initialization, backup, and migration scripts for the AI-Rust-API RAG system using PostgreSQL with pgvector.

## 🗄️ Database Schema

The RAG system uses PostgreSQL with the pgvector extension to store:
- **Users & Authentication**: User accounts, sessions, and permissions
- **Documents & Embeddings**: Document chunks with vector embeddings
- **RAG System**: Vectors, RAG models, and configurations
- **Audit & History**: System logs and chat history

### Tables Overview
- `users` - User accounts and authentication
- `user_sessions` - JWT session management
- `documents` - Document chunks and metadata
- `embeddings` - Vector embeddings (1536 dimensions)
- `document_folders` - Document organization
- `vectors` - RAG vector collections
- `rag_models` - RAG model configurations
- `config_settings` - System configuration
- `chat_history` - Conversation history
- `audit_log` - System activity logging

## 📁 Files

### Core Scripts
- **`init.sql`** - Complete database schema initialization
- **`sample_data.sql`** - Sample data for development and testing
- **`setup.sh`** - Automated database setup script
- **`backup.sh`** - Database backup and restore script
- **`docker-compose.yml`** - PostgreSQL with pgvector container

### Configuration
- **`.env.example`** - Example environment configuration
- **`backups/`** - Backup files directory (auto-created)

## 🚀 Quick Start

### 1. Setup Database
```bash
# Navigate to database directory
cd services/Database

# Run automated setup (recommended)
./setup.sh

# Or manual setup:
docker-compose up -d postgres
psql postgresql://raguser:password@localhost:5434/ragdb < init.sql
```

### 2. Add Sample Data (Optional)
```bash
# Add sample data for testing
psql postgresql://raguser:password@localhost:5434/ragdb < sample_data.sql
```

### 3. Verify Installation
```bash
# Check setup
./setup.sh verify

# Test connection
psql postgresql://raguser:password@localhost:5434/ragdb -c "SELECT 1;"
```

## 🔧 Detailed Setup

### Prerequisites
- Docker and Docker Compose
- PostgreSQL client tools (psql, pg_dump)

### Environment Variables
```bash
# Database Configuration
export DB_HOST=localhost
export DB_PORT=5434
export DB_NAME=ragdb
export DB_USER=raguser
export DB_PASSWORD=password
export DB_URL="postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
```

### Manual Setup Steps

#### 1. Start PostgreSQL Container
```bash
docker-compose up -d postgres

# Wait for startup
docker-compose logs -f postgres
```

#### 2. Initialize Schema
```bash
# Create all tables, indexes, functions, and views
psql "$DB_URL" -f init.sql
```

#### 3. Create Sample Data (Optional)
```bash
# Add sample users, vectors, and configurations
psql "$DB_URL" -f sample_data.sql
```

## 🔨 Management Scripts

### Database Setup Script (`setup.sh`)
Comprehensive setup automation with dependency checking:

```bash
# Full setup process
./setup.sh

# Individual commands
./setup.sh check      # Check dependencies
./setup.sh start      # Start database only
./setup.sh init       # Initialize schema only
./setup.sh verify     # Verify setup
./setup.sh info       # Show connection info
```

### Backup Script (`backup.sh`)
Complete backup and restore functionality:

```bash
# Create backups
./backup.sh full       # Full database backup
./backup.sh schema     # Schema-only backup
./backup.sh data       # Data-only backup
./backup.sh custom     # Custom format backup

# Management
./backup.sh list       # List available backups
./backup.sh cleanup 7  # Clean up backups older than 7 days

# Restore
./backup.sh restore backup_file.sql
```

## 🗄️ Database Schema Details

### Core Tables

#### Users & Authentication
```sql
-- User accounts with role-based access
users (id, username, email, password_hash, is_active, is_admin, created_at, updated_at, last_login)

-- JWT session management
user_sessions (id, user_id, token_hash, expires_at, created_at, user_agent, ip_address)
```

#### Document Management
```sql
-- Document chunks for RAG processing
documents (id, filename, content, file_hash, chunk_index, created_at, updated_at)

-- Vector embeddings (1536 dimensions for AWS Titan)
embeddings (id, document_id, embedding, created_at)

-- Document folder organization
document_folders (id, folder_name, folder_path, created_at)
```

#### RAG System
```sql
-- Vector collections (isolated document groups)
vectors (id, name, folder_name, description, document_count, embedding_count, created_by, created_at, updated_at, is_active)

-- RAG model configurations
rag_models (id, name, vector_id, system_prompt, context, created_by, created_at, updated_at, is_active)

-- System configuration settings
config_settings (key, value, is_encrypted, description, created_at, updated_at)
```

#### Audit & History
```sql
-- Chat conversation history
chat_history (id, user_id, conversation_id, user_message, assistant_response, rag_model_id, sources_used, created_at)

-- System activity logging
audit_log (id, user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at)
```

### Performance Features

#### Indexes
- **Vector Search**: IVFFlat index on embeddings for fast similarity search
- **User Lookups**: Indexes on username, email for authentication
- **Document Search**: Indexes on filename, file_hash for retrieval
- **Audit Queries**: Indexes on user_id, action, timestamp for monitoring

#### Functions & Triggers
- **Automatic Timestamps**: Auto-update `updated_at` columns
- **Session Cleanup**: Function to remove expired sessions
- **System Health**: Function to check system status

#### Views
- **User Statistics**: Aggregated user metrics
- **System Statistics**: Overall system metrics  
- **RAG Models with Vectors**: Joined view for model management

## 🔐 Default Credentials

**Admin User (Development Only)**
- Username: `admin`
- Password: `password`
- Email: `admin@example.com`

**⚠️ IMPORTANT**: Change default credentials in production!

## 📊 Sample Data

The `sample_data.sql` script creates:
- 4 additional users (testuser, researcher, dataanalyst, moderator)
- 4 document folders for different content types
- 4 vectors with sample configurations
- 4 RAG models with specialized prompts
- 16 system configuration settings
- Sample chat history and audit logs

## 🛠️ Maintenance

### Regular Maintenance
```sql
-- Clean up expired sessions
SELECT cleanup_expired_sessions();

-- Update table statistics
ANALYZE documents;
ANALYZE embeddings;
ANALYZE vectors;
ANALYZE rag_models;

-- Check system health
SELECT * FROM get_system_health();
```

### Monitoring Queries
```sql
-- User activity
SELECT * FROM user_statistics;

-- System metrics
SELECT * FROM system_statistics;

-- Recent activity
SELECT * FROM audit_log ORDER BY created_at DESC LIMIT 50;

-- Vector search performance
EXPLAIN ANALYZE SELECT * FROM embeddings ORDER BY embedding <-> '[...]' LIMIT 10;
```

### Backup Strategy
- **Daily**: Automated full backups with compression
- **Weekly**: Schema-only backups for structure changes
- **Monthly**: Custom format backups for fast restore
- **Retention**: 30 days for full backups, 90 days for schema

## 🚨 Production Considerations

### Security
- Change all default passwords
- Use strong JWT secrets
- Enable SSL/TLS for database connections
- Implement proper backup encryption
- Regular security audits

### Performance
- Monitor vector index performance
- Adjust `lists` parameter for IVFFlat index based on data size
- Regular `VACUUM` and `ANALYZE` operations
- Monitor connection pool usage

### Scaling
- Consider read replicas for query-heavy workloads
- Partition large tables by date if needed
- Monitor disk space for embeddings growth
- Plan for vector index rebuilding as data grows

## 🔧 Troubleshooting

### Common Issues

**Connection Failed**
```bash
# Check if container is running
docker-compose ps

# Check logs
docker-compose logs postgres

# Test connection
psql "$DB_URL" -c "SELECT 1;"
```

**pgvector Extension Missing**
```sql
-- Install extension
CREATE EXTENSION IF NOT EXISTS vector;

-- Verify installation
SELECT * FROM pg_extension WHERE extname = 'vector';
```

**Slow Vector Searches**
```sql
-- Check index usage
EXPLAIN ANALYZE SELECT * FROM embeddings ORDER BY embedding <-> '[...]' LIMIT 10;

-- Rebuild index with different parameters
DROP INDEX embeddings_vector_idx;
CREATE INDEX embeddings_vector_idx ON embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 1000);
```

**Storage Issues**
```bash
# Check database size
psql "$DB_URL" -c "SELECT pg_size_pretty(pg_database_size('ragdb'));"

# Check largest tables
psql "$DB_URL" -c "SELECT schemaname, tablename, pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) as size FROM pg_tables ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;"
```

## 📞 Support

For database-related issues:
1. Check Docker container status
2. Verify environment variables
3. Review PostgreSQL logs
4. Test connectivity
5. Check pgvector extension installation

The database is the foundation of the RAG system - proper setup and maintenance ensure optimal performance and reliability.