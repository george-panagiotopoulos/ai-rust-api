-- ============================================================================
-- Backend Configuration Management Migration
-- ============================================================================
-- This script adds the backend_configs table for managing AWS/Azure backends
-- ============================================================================

-- Backend configurations table for AWS and Azure providers
CREATE TABLE IF NOT EXISTS backend_configs (
    id SERIAL PRIMARY KEY,
    provider VARCHAR(20) NOT NULL UNIQUE, -- 'aws' or 'azure'
    is_active BOOLEAN DEFAULT false,
    
    -- LLM Configuration
    llm_api_key TEXT,
    llm_endpoint TEXT,
    llm_model_name VARCHAR(255),
    llm_max_tokens INTEGER,
    llm_temperature DECIMAL(3,2),
    
    -- Embedding Configuration  
    embedding_api_key TEXT,
    embedding_endpoint TEXT,
    embedding_model_name VARCHAR(255),
    embedding_dimension INTEGER,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT chk_provider CHECK (provider IN ('aws', 'azure')),
    CONSTRAINT chk_temperature CHECK (llm_temperature >= 0.0 AND llm_temperature <= 2.0)
);

-- Index for performance
CREATE INDEX IF NOT EXISTS idx_backend_configs_provider ON backend_configs(provider);
CREATE INDEX IF NOT EXISTS idx_backend_configs_active ON backend_configs(is_active);

-- Trigger for automatic timestamp updates
DROP TRIGGER IF EXISTS update_backend_configs_updated_at ON backend_configs;
CREATE TRIGGER update_backend_configs_updated_at 
    BEFORE UPDATE ON backend_configs 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default backend configurations
INSERT INTO backend_configs (provider, is_active) VALUES 
    ('aws', true),
    ('azure', false)
ON CONFLICT (provider) DO NOTHING;

-- Log completion
DO $$
BEGIN
    RAISE NOTICE '============================================================================';
    RAISE NOTICE 'Backend Configuration Table Added Successfully!';
    RAISE NOTICE '============================================================================';
    RAISE NOTICE 'Default configurations created for AWS (active) and Azure (inactive)';
    RAISE NOTICE '============================================================================';
END $$;