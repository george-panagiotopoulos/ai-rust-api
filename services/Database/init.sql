-- ============================================================================
-- AI-Rust-API Database Initialization Script
-- ============================================================================
-- This script initializes the complete database schema for the RAG system
-- including all tables for users, documents, vectors, RAG models, and configs
-- 
-- Usage:
--   psql postgresql://raguser:password@localhost:5434/ragdb < init.sql
-- ============================================================================

-- Enable required PostgreSQL extensions
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- User Management Tables
-- ============================================================================

-- Users table for authentication and authorization
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    is_active BOOLEAN DEFAULT true,
    is_admin BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP WITH TIME ZONE
);

-- User sessions table for token management
CREATE TABLE IF NOT EXISTS user_sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    user_agent TEXT,
    ip_address TEXT
);

-- ============================================================================
-- Document Management Tables
-- ============================================================================

-- Documents table for storing processed document chunks
CREATE TABLE IF NOT EXISTS documents (
    id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    file_hash VARCHAR(64) NOT NULL,
    chunk_index INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(file_hash, chunk_index)
);

-- Embeddings table for vector storage
CREATE TABLE IF NOT EXISTS embeddings (
    id SERIAL PRIMARY KEY,
    document_id INTEGER REFERENCES documents(id) ON DELETE CASCADE,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Document folders for organization
CREATE TABLE IF NOT EXISTS document_folders (
    id SERIAL PRIMARY KEY,
    folder_name VARCHAR(255) NOT NULL UNIQUE,
    folder_path TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Vector Management Tables
-- ============================================================================

-- Vectors table for RAG vector management
CREATE TABLE IF NOT EXISTS vectors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    folder_name VARCHAR(255) NOT NULL,
    description TEXT,
    document_count INTEGER DEFAULT 0,
    embedding_count INTEGER DEFAULT 0,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

-- RAG models table for model configuration
CREATE TABLE IF NOT EXISTS rag_models (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    vector_id INTEGER REFERENCES vectors(id) ON DELETE CASCADE,
    system_prompt TEXT NOT NULL,
    context TEXT,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

-- ============================================================================
-- Configuration Management Tables
-- ============================================================================

-- Configuration settings for environment management
CREATE TABLE IF NOT EXISTS config_settings (
    key VARCHAR(100) PRIMARY KEY,
    value TEXT NOT NULL,
    is_encrypted BOOLEAN DEFAULT false,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Chat History Tables
-- ============================================================================

-- Chat history for conversation tracking
CREATE TABLE IF NOT EXISTS chat_history (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    conversation_id VARCHAR(255) NOT NULL,
    user_message TEXT NOT NULL,
    assistant_response TEXT NOT NULL,
    rag_model_id INTEGER REFERENCES rag_models(id),
    sources_used JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- System Audit Tables
-- ============================================================================

-- Audit log for tracking administrative actions
CREATE TABLE IF NOT EXISTS audit_log (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50) NOT NULL,
    resource_id INTEGER,
    details JSONB,
    ip_address TEXT,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Indexes for Performance
-- ============================================================================

-- User indexes
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_active_admin ON users(is_active, is_admin);

-- Session indexes
CREATE INDEX IF NOT EXISTS idx_sessions_token_hash ON user_sessions(token_hash);
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON user_sessions(expires_at);

-- Document indexes
CREATE INDEX IF NOT EXISTS idx_documents_file_hash ON documents(file_hash);
CREATE INDEX IF NOT EXISTS idx_documents_filename ON documents(filename);
CREATE INDEX IF NOT EXISTS idx_documents_created_at ON documents(created_at);

-- Vector similarity search index (critical for performance)
CREATE INDEX IF NOT EXISTS embeddings_vector_idx ON embeddings 
    USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);

-- Vector and RAG model indexes
CREATE INDEX IF NOT EXISTS idx_vectors_folder_name ON vectors(folder_name);
CREATE INDEX IF NOT EXISTS idx_vectors_active ON vectors(is_active);
CREATE INDEX IF NOT EXISTS idx_vectors_created_by ON vectors(created_by);

CREATE INDEX IF NOT EXISTS idx_rag_models_vector_id ON rag_models(vector_id);
CREATE INDEX IF NOT EXISTS idx_rag_models_active ON rag_models(is_active);
CREATE INDEX IF NOT EXISTS idx_rag_models_created_by ON rag_models(created_by);

-- Chat history indexes
CREATE INDEX IF NOT EXISTS idx_chat_history_user_id ON chat_history(user_id);
CREATE INDEX IF NOT EXISTS idx_chat_history_conversation_id ON chat_history(conversation_id);
CREATE INDEX IF NOT EXISTS idx_chat_history_rag_model_id ON chat_history(rag_model_id);
CREATE INDEX IF NOT EXISTS idx_chat_history_created_at ON chat_history(created_at);

-- Configuration indexes
CREATE INDEX IF NOT EXISTS idx_config_settings_updated_at ON config_settings(updated_at);

-- Audit log indexes
CREATE INDEX IF NOT EXISTS idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_action ON audit_log(action);
CREATE INDEX IF NOT EXISTS idx_audit_log_resource ON audit_log(resource_type, resource_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_created_at ON audit_log(created_at);

-- ============================================================================
-- Functions for Automatic Timestamp Updates
-- ============================================================================

-- Function to update the updated_at column
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for automatic timestamp updates
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_documents_updated_at ON documents;
CREATE TRIGGER update_documents_updated_at 
    BEFORE UPDATE ON documents 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_vectors_updated_at ON vectors;
CREATE TRIGGER update_vectors_updated_at 
    BEFORE UPDATE ON vectors 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_rag_models_updated_at ON rag_models;
CREATE TRIGGER update_rag_models_updated_at 
    BEFORE UPDATE ON rag_models 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_config_settings_updated_at ON config_settings;
CREATE TRIGGER update_config_settings_updated_at 
    BEFORE UPDATE ON config_settings 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Default Data
-- ============================================================================

-- Insert default admin user
-- Password is bcrypt hash of "password" (for development only - change in production!)
INSERT INTO users (username, email, password_hash, is_active, is_admin) 
VALUES (
    'admin', 
    'admin@example.com', 
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go6LZibO2',
    true, 
    true
)
ON CONFLICT (username) DO NOTHING;

-- Insert sample configuration
INSERT INTO config_settings (key, value, description) VALUES
    ('system.version', '1.0.0', 'System version number'),
    ('system.maintenance_mode', 'false', 'System maintenance mode flag'),
    ('rag.default_chunks', '10', 'Default number of chunks to retrieve for RAG queries'),
    ('rag.similarity_threshold', '-10.0', 'Minimum similarity score for document retrieval')
ON CONFLICT (key) DO NOTHING;

-- ============================================================================
-- Helpful Views
-- ============================================================================

-- View for user statistics
CREATE OR REPLACE VIEW user_statistics AS
SELECT 
    COUNT(*) as total_users,
    COUNT(CASE WHEN is_active THEN 1 END) as active_users,
    COUNT(CASE WHEN is_admin THEN 1 END) as admin_users,
    COUNT(CASE WHEN last_login > (CURRENT_TIMESTAMP - INTERVAL '24 hours') THEN 1 END) as recent_logins
FROM users;

-- View for system statistics
CREATE OR REPLACE VIEW system_statistics AS
SELECT 
    (SELECT COUNT(*) FROM documents) as total_documents,
    (SELECT COUNT(*) FROM embeddings) as total_embeddings,
    (SELECT COUNT(*) FROM vectors WHERE is_active = true) as active_vectors,
    (SELECT COUNT(*) FROM rag_models WHERE is_active = true) as active_rag_models,
    (SELECT COUNT(*) FROM chat_history) as total_conversations;

-- View for RAG models with vector information
CREATE OR REPLACE VIEW rag_models_with_vectors AS
SELECT 
    rm.id,
    rm.name,
    rm.vector_id,
    v.name as vector_name,
    v.folder_name as vector_folder,
    rm.system_prompt,
    rm.context,
    rm.created_by,
    rm.created_at,
    rm.updated_at,
    rm.is_active,
    v.document_count,
    v.embedding_count
FROM rag_models rm
JOIN vectors v ON rm.vector_id = v.id;

-- ============================================================================
-- Database Functions
-- ============================================================================

-- Function to clean up expired sessions
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM user_sessions WHERE expires_at < CURRENT_TIMESTAMP;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Function to get system health information
CREATE OR REPLACE FUNCTION get_system_health()
RETURNS TABLE(
    component TEXT,
    status TEXT,
    details TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        'database'::TEXT as component,
        'healthy'::TEXT as status,
        'Connected successfully'::TEXT as details
    UNION ALL
    SELECT 
        'users'::TEXT as component,
        CASE WHEN COUNT(*) > 0 THEN 'healthy'::TEXT ELSE 'warning'::TEXT END as status,
        CONCAT('Total users: ', COUNT(*))::TEXT as details
    FROM users
    UNION ALL
    SELECT 
        'documents'::TEXT as component,
        CASE WHEN COUNT(*) > 0 THEN 'healthy'::TEXT ELSE 'warning'::TEXT END as status,
        CONCAT('Total documents: ', COUNT(*))::TEXT as details
    FROM documents;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Completion Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '============================================================================';
    RAISE NOTICE 'AI-Rust-API Database Initialization Complete!';
    RAISE NOTICE '============================================================================';
    RAISE NOTICE 'Tables created: users, user_sessions, documents, embeddings, document_folders,';
    RAISE NOTICE '               vectors, rag_models, config_settings, chat_history, audit_log';
    RAISE NOTICE '';
    RAISE NOTICE 'Default admin user created:';
    RAISE NOTICE '  Username: admin';
    RAISE NOTICE '  Password: password (CHANGE IN PRODUCTION!)';
    RAISE NOTICE '  Email: admin@example.com';
    RAISE NOTICE '';
    RAISE NOTICE 'System is ready for use!';
    RAISE NOTICE '============================================================================';
END $$;