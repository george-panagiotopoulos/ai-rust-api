-- ============================================================================
-- AI-Rust-API Sample Data Script
-- ============================================================================
-- This script creates sample data for development and testing purposes
-- 
-- Usage:
--   psql postgresql://raguser:password@localhost:5434/ragdb < sample_data.sql
-- ============================================================================

-- ============================================================================
-- Sample Users
-- ============================================================================

-- Create sample regular users
INSERT INTO users (username, email, password_hash, is_active, is_admin) VALUES
    ('testuser', 'test@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go6LZibO2', true, false),
    ('researcher', 'researcher@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go6LZibO2', true, false),
    ('dataanalyst', 'analyst@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go6LZibO2', true, false),
    ('moderator', 'mod@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go6LZibO2', true, true)
ON CONFLICT (username) DO NOTHING;

-- ============================================================================
-- Sample Document Folders
-- ============================================================================

INSERT INTO document_folders (folder_name, folder_path) VALUES
    ('DistributedSystems', '/ai-rust-api/services/RAGAPI/documents/DistributedSystems'),
    ('Microservices', '/ai-rust-api/services/RAGAPI/documents/Microservices'),
    ('TechnicalDocs', '/ai-rust-api/services/RAGAPI/documents/TechnicalDocs'),
    ('ResearchPapers', '/ai-rust-api/services/RAGAPI/documents/ResearchPapers')
ON CONFLICT (folder_name) DO NOTHING;

-- ============================================================================
-- Sample Vectors
-- ============================================================================

INSERT INTO vectors (name, folder_name, description, document_count, embedding_count, created_by, is_active) VALUES
    ('Distributed Systems Knowledge Base', 'DistributedSystems', 'Vector containing documents about distributed systems, databases, and system design', 1, 1410, 1, true),
    ('Microservices Architecture', 'Microservices', 'Vector containing patterns and practices for microservices architecture', 1, 1200, 1, true),
    ('Technical Documentation', 'TechnicalDocs', 'General technical documentation and guides', 0, 0, 1, true),
    ('Research Papers Collection', 'ResearchPapers', 'Academic and industry research papers', 0, 0, 1, true)
ON CONFLICT DO NOTHING;

-- ============================================================================
-- Sample RAG Models
-- ============================================================================

INSERT INTO rag_models (name, vector_id, system_prompt, context, created_by, is_active) VALUES
    (
        'Distributed Systems Expert',
        1,
        'You are a knowledgeable expert in distributed systems, databases, and large-scale system design. You have deep understanding of concepts like consistency, availability, partition tolerance, data replication, sharding, and system scalability. Provide detailed, accurate, and practical answers based on the provided context. When discussing trade-offs, explain both benefits and drawbacks clearly.',
        'This model specializes in distributed systems concepts from Martin Kleppmann''s "Designing Data-Intensive Applications" and related materials.',
        1,
        true
    ),
    (
        'Microservices Architect', 
        2,
        'You are an experienced software architect specializing in microservices patterns and practices. You understand service decomposition, API design, data management in microservices, testing strategies, deployment patterns, and organizational considerations. Provide comprehensive guidance on microservices architecture patterns, anti-patterns, and best practices based on the provided context.',
        'This model focuses on microservices architecture patterns from Chris Richardson''s work and industry best practices.',
        1,
        true
    ),
    (
        'Technical Assistant',
        3,
        'You are a helpful technical assistant that can answer questions about various technical topics including software development, system administration, and technology concepts. Provide clear, concise, and accurate answers based on the provided documentation.',
        NULL,
        1,
        true
    ),
    (
        'Research Assistant',
        4,
        'You are an academic research assistant that can help analyze and explain research papers, methodologies, and findings. Provide detailed analysis of research content, explain complex concepts in accessible terms, and highlight key insights and implications.',
        'This model is designed to assist with academic and industry research materials.',
        1,
        true
    )
ON CONFLICT DO NOTHING;

-- ============================================================================
-- Sample Configuration Settings
-- ============================================================================

INSERT INTO config_settings (key, value, description) VALUES
    ('system.max_file_size', '100MB', 'Maximum file size for document uploads'),
    ('system.supported_formats', 'pdf,txt,md,docx', 'Supported document formats for processing'),
    ('rag.max_context_length', '8000', 'Maximum context length for RAG queries'),
    ('rag.chunk_size', '1000', 'Size of document chunks for processing'),
    ('rag.chunk_overlap', '250', 'Overlap between consecutive chunks'),
    ('embedding.model', 'amazon.titan-embed-text-v1', 'AWS Bedrock embedding model'),
    ('llm.default_model', 'anthropic.claude-3-sonnet-20240229-v1:0', 'Default LLM model for responses'),
    ('llm.max_tokens', '1000', 'Maximum tokens for LLM responses'),
    ('llm.temperature', '0.7', 'Temperature setting for LLM responses'),
    ('security.session_timeout', '86400', 'Session timeout in seconds (24 hours)'),
    ('security.max_login_attempts', '5', 'Maximum login attempts before lockout'),
    ('backup.enabled', 'true', 'Enable automatic database backups'),
    ('backup.frequency', '24', 'Backup frequency in hours'),
    ('monitoring.health_check_interval', '300', 'Health check interval in seconds'),
    ('ui.theme', 'light', 'Default UI theme'),
    ('ui.items_per_page', '20', 'Default items per page in data tables')
ON CONFLICT (key) DO NOTHING;

-- ============================================================================
-- Sample Chat History
-- ============================================================================

-- Generate some sample chat history for demonstration
INSERT INTO chat_history (user_id, conversation_id, user_message, assistant_response, rag_model_id, sources_used) VALUES
    (
        2, -- researcher user
        'conv-001',
        'What is eventual consistency in distributed systems?',
        'Eventual consistency is a consistency model used in distributed systems that guarantees that, in the absence of new updates, all replicas will eventually converge to the same value. Unlike strong consistency, it doesn''t require immediate consistency across all nodes, allowing for better availability and partition tolerance according to the CAP theorem.',
        1, -- Distributed Systems Expert model
        '[{"filename": "distributed_systems_chunk_1", "similarity": 0.85}, {"filename": "distributed_systems_chunk_15", "similarity": 0.78}]'::jsonb
    ),
    (
        3, -- dataanalyst user
        'conv-002', 
        'What are the benefits of using microservices architecture?',
        'Microservices architecture offers several key benefits: 1) Independent deployability - services can be deployed independently, 2) Technology diversity - different services can use different technologies, 3) Scalability - individual services can be scaled based on demand, 4) Team autonomy - small teams can own entire services, 5) Fault isolation - failures in one service don''t necessarily bring down the entire system.',
        2, -- Microservices Architect model
        '[{"filename": "microservices_chunk_3", "similarity": 0.92}, {"filename": "microservices_chunk_8", "similarity": 0.87}]'::jsonb
    ),
    (
        2, -- researcher user
        'conv-001',
        'How does data partitioning work in distributed databases?',
        'Data partitioning in distributed databases involves splitting data across multiple nodes to improve performance and scalability. Common strategies include: 1) Horizontal partitioning (sharding) - splitting rows across nodes, 2) Vertical partitioning - splitting columns across nodes, 3) Hash-based partitioning - using hash functions to distribute data, 4) Range-based partitioning - distributing data based on ranges of values. Each approach has trade-offs in terms of query performance, data distribution, and rebalancing complexity.',
        1, -- Distributed Systems Expert model  
        '[{"filename": "distributed_systems_chunk_42", "similarity": 0.91}, {"filename": "distributed_systems_chunk_67", "similarity": 0.83}]'::jsonb
    )
ON CONFLICT DO NOTHING;

-- ============================================================================
-- Sample Audit Log Entries
-- ============================================================================

INSERT INTO audit_log (user_id, action, resource_type, resource_id, details, ip_address, user_agent) VALUES
    (1, 'CREATE_VECTOR', 'vector', 1, '{"name": "Distributed Systems Knowledge Base", "folder_name": "DistributedSystems"}'::jsonb, '127.0.0.1', 'Mozilla/5.0 (Admin Browser)'),
    (1, 'CREATE_VECTOR', 'vector', 2, '{"name": "Microservices Architecture", "folder_name": "Microservices"}'::jsonb, '127.0.0.1', 'Mozilla/5.0 (Admin Browser)'),
    (1, 'CREATE_RAG_MODEL', 'rag_model', 1, '{"name": "Distributed Systems Expert", "vector_id": 1}'::jsonb, '127.0.0.1', 'Mozilla/5.0 (Admin Browser)'),
    (1, 'CREATE_RAG_MODEL', 'rag_model', 2, '{"name": "Microservices Architect", "vector_id": 2}'::jsonb, '127.0.0.1', 'Mozilla/5.0 (Admin Browser)'),
    (1, 'CREATE_USER', 'user', 2, '{"username": "testuser", "email": "test@example.com", "is_admin": false}'::jsonb, '127.0.0.1', 'Mozilla/5.0 (Admin Browser)'),
    (2, 'LOGIN', 'session', NULL, '{"successful": true}'::jsonb, '192.168.1.100', 'Mozilla/5.0 (User Browser)'),
    (3, 'LOGIN', 'session', NULL, '{"successful": true}'::jsonb, '192.168.1.101', 'Mozilla/5.0 (User Browser)')
ON CONFLICT DO NOTHING;

-- ============================================================================
-- Update Statistics and Cleanup
-- ============================================================================

-- Update table statistics for better query performance
ANALYZE users;
ANALYZE vectors;
ANALYZE rag_models;
ANALYZE config_settings;
ANALYZE chat_history;
ANALYZE audit_log;

-- Clean up any expired sessions
SELECT cleanup_expired_sessions();

-- ============================================================================
-- Completion Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '============================================================================';
    RAISE NOTICE 'Sample Data Creation Complete!';
    RAISE NOTICE '============================================================================';
    RAISE NOTICE 'Created:';
    RAISE NOTICE '  - 4 additional users (testuser, researcher, dataanalyst, moderator)';
    RAISE NOTICE '  - 4 document folders';
    RAISE NOTICE '  - 4 vectors (2 with sample data)';
    RAISE NOTICE '  - 4 RAG models';
    RAISE NOTICE '  - 16 configuration settings';
    RAISE NOTICE '  - 3 sample chat conversations';
    RAISE NOTICE '  - 7 audit log entries';
    RAISE NOTICE '';
    RAISE NOTICE 'All users have password: "password"';
    RAISE NOTICE 'Database is ready for testing and development!';
    RAISE NOTICE '============================================================================';
END $$;