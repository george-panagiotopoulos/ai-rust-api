import React, { useState, useEffect } from 'react';
import {
  Container,
  Paper,
  Typography,
  Box,
  Tabs,
  Tab,
  Button,
  Card,
  CardContent,
  Divider,
  List,
  ListItem,
  ListItemText,
  Chip,
  TextField,
  IconButton,
  Tooltip,
  Collapse,
  Alert,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  Grid,
  AppBar,
  Toolbar
} from '@mui/material';
import {
  Download as DownloadIcon,
  ExpandMore as ExpandMoreIcon,
  Code as CodeIcon,
  Api as ApiIcon,
  Description as DescriptionIcon,
  Security as SecurityIcon,
  Storage as StorageIcon,
  Psychology as PsychologyIcon,
  Settings as SettingsIcon,
  ContentCopy as ContentCopyIcon,
  Launch as LaunchIcon,
  Logout
} from '@mui/icons-material';
import { useAuth } from '../context/AuthContext';

const ApiDocumentation = () => {
  const { logout, user } = useAuth();
  const [activeTab, setActiveTab] = useState(0);
  const [expandedEndpoint, setExpandedEndpoint] = useState(null);
  const [copiedText, setCopiedText] = useState('');
  const [collectionPreview, setCollectionPreview] = useState('');

  // Load collection preview
  useEffect(() => {
    const loadCollectionPreview = async () => {
      try {
        const response = await fetch('/api-docs/AI-Rust-API.postman_collection.json');
        if (response.ok) {
          const collection = await response.text();
          setCollectionPreview(collection);
        } else {
          setCollectionPreview(generatePostmanCollection());
        }
      } catch (error) {
        setCollectionPreview(generatePostmanCollection());
      }
    };
    
    loadCollectionPreview();
  }, []);

  const handleTabChange = (event, newValue) => {
    setActiveTab(newValue);
  };

  const handleExpandEndpoint = (endpoint) => {
    setExpandedEndpoint(expandedEndpoint === endpoint ? null : endpoint);
  };

  const handleCopyToClipboard = async (text, label) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedText(label);
      setTimeout(() => setCopiedText(''), 2000);
    } catch (err) {
      console.error('Failed to copy to clipboard:', err);
    }
  };

  const downloadFile = (content, filename, mimeType) => {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  // API endpoints data structure
  const apiServices = {
    authApi: {
      name: 'Authentication API',
      baseUrl: 'http://localhost:9102',
      description: 'JWT-based authentication and user management service',
      icon: <SecurityIcon />,
      endpoints: [
        {
          method: 'GET',
          path: '/health',
          name: 'Health Check',
          description: 'Check authentication service health status',
          auth: false,
          request: null,
          response: {
            status: 'string',
            service: 'string',
            version: 'string'
          }
        },
        {
          method: 'POST',
          path: '/login',
          name: 'User Login',
          description: 'Authenticate user and receive JWT token',
          auth: false,
          request: {
            username: 'string (required, 1-50 characters)',
            password: 'string (required, min 1 character)'
          },
          response: {
            access_token: 'string (JWT token)',
            token_type: 'string ("Bearer")',
            expires_in: 'number (seconds)',
            user: 'UserResponse object'
          }
        },
        {
          method: 'POST',
          path: '/register',
          name: 'User Registration',
          description: 'Register a new user account',
          auth: false,
          request: {
            username: 'string (required, 3-50 characters)',
            email: 'string (required, valid email)',
            password: 'string (required, min 8 characters)'
          },
          response: {
            id: 'number',
            username: 'string',
            email: 'string',
            is_active: 'boolean',
            is_admin: 'boolean',
            created_at: 'string (ISO datetime)',
            updated_at: 'string (ISO datetime)',
            last_login: 'string (ISO datetime, nullable)'
          }
        },
        {
          method: 'GET',
          path: '/profile',
          name: 'Get User Profile',
          description: 'Get current user profile information',
          auth: true,
          request: null,
          response: {
            id: 'number',
            username: 'string',
            email: 'string',
            is_active: 'boolean',
            is_admin: 'boolean',
            created_at: 'string (ISO datetime)',
            updated_at: 'string (ISO datetime)',
            last_login: 'string (ISO datetime, nullable)'
          }
        },
        {
          method: 'POST',
          path: '/validate',
          name: 'Validate Token',
          description: 'Validate a JWT token',
          auth: false,
          request: {
            token: 'string (required, JWT token)'
          },
          response: {
            valid: 'boolean',
            user: 'UserResponse object (nullable)',
            expires_at: 'string (ISO datetime, nullable)'
          }
        },
        {
          method: 'POST',
          path: '/logout',
          name: 'User Logout',
          description: 'Invalidate current JWT token',
          auth: true,
          request: null,
          response: {
            message: 'string'
          }
        },
        {
          method: 'GET',
          path: '/stats',
          name: 'Authentication Stats',
          description: 'Get authentication service statistics',
          auth: false,
          request: null,
          response: {
            active_users: 'number',
            active_sessions: 'number',
            cleaned_sessions: 'number'
          }
        }
      ]
    },
    bedrockApi: {
      name: 'Bedrock AI Chat API',
      baseUrl: 'http://localhost:9100',
      description: 'AI chat completion service using AWS Bedrock',
      icon: <PsychologyIcon />,
      endpoints: [
        {
          method: 'GET',
          path: '/health',
          name: 'Health Check',
          description: 'Check service health status',
          auth: false,
          request: null,
          response: {
            status: 'string',
            service: 'string'
          }
        },
        {
          method: 'POST',
          path: '/chat',
          name: 'Standard Chat',
          description: 'Send a message and receive AI response with conversation ID',
          auth: true,
          request: {
            message: 'string (required)'
          },
          response: {
            id: 'uuid',
            response: 'string'
          }
        },
        {
          method: 'POST',
          path: '/simple-chat',
          name: 'Simple Chat with Parameters',
          description: 'Advanced chat with configurable parameters',
          auth: true,
          request: {
            prompt: 'string (required)',
            max_tokens: 'number (optional)',
            temperature: 'number (optional, 0.0-1.0)',
            top_p: 'number (optional, 0.0-1.0)'
          },
          response: {
            response: 'string',
            token_count: 'number (optional)'
          }
        }
      ]
    },
    ragApi: {
      name: 'RAG Document API',
      baseUrl: 'http://localhost:9101',
      description: 'Document retrieval and Q&A service with PostgreSQL/pgvector',
      icon: <StorageIcon />,
      endpoints: [
        {
          method: 'GET',
          path: '/health',
          name: 'Health Check',
          description: 'Check service health status',
          auth: false,
          request: null,
          response: {
            status: 'string',
            service: 'string',
            version: 'string'
          }
        },
        {
          method: 'GET',
          path: '/stats',
          name: 'RAG Statistics',
          description: 'Get RAG system statistics',
          auth: true,
          request: null,
          response: {
            document_count: 'number',
            embedding_count: 'number',
            vector_dimensions: 'number'
          }
        },
        {
          method: 'POST',
          path: '/query',
          name: 'RAG Query',
          description: 'Query documents with RAG model',
          auth: true,
          request: {
            query: 'string (required)',
            rag_model_name: 'string (optional, e.g. "Microservices" or "DistributedSystems")',
            system_prompt: 'string (optional)',
            context: 'string (optional)',
            max_tokens: 'number (optional)',
            temperature: 'number (optional)'
          },
          response: {
            answer: 'string',
            sources: 'array of source objects',
            context_used: 'string'
          }
        },
        {
          method: 'POST',
          path: '/search',
          name: 'Document Search',
          description: 'Search documents by similarity',
          auth: true,
          request: {
            query: 'string (required)',
            limit: 'number (optional)',
            similarity_threshold: 'number (optional)'
          },
          response: {
            documents: 'array of document objects'
          }
        },
        {
          method: 'POST',
          path: '/process-document',
          name: 'Process Document',
          description: 'Process and embed a document',
          auth: true,
          request: {
            filename: 'string (required)',
            content: 'string (required)'
          },
          response: {
            success: 'boolean',
            document_id: 'number',
            chunks_processed: 'number',
            message: 'string'
          }
        },
        {
          method: 'POST',
          path: '/generate-embedding',
          name: 'Generate Embedding',
          description: 'Generate vector embedding for text',
          auth: false,
          request: {
            text: 'string (required)'
          },
          response: {
            embedding: 'array of numbers',
            dimension: 'number'
          }
        },
        {
          method: 'POST',
          path: '/process-stored-document',
          name: 'Process Stored Document',
          description: 'Process a document that is already stored in the database',
          auth: false,
          request: {
            document_id: 'number (required)'
          },
          response: {
            success: 'boolean',
            document_id: 'number',
            embedding_generated: 'boolean',
            message: 'string'
          }
        },
        {
          method: 'GET',
          path: '/rag-models',
          name: 'Get RAG Models',
          description: 'Get available RAG models',
          auth: true,
          request: null,
          response: {
            rag_models: 'array of RAG model objects'
          }
        }
      ]
    },
    uiConfigApi: {
      name: 'UI Configuration API (Port 9103)',
      baseUrl: 'http://localhost:9103',
      description: 'Configuration and administration service for the AI-Rust-API system',
      icon: <SettingsIcon />,
      endpoints: [
        // Public Endpoints
        {
          method: 'GET',
          path: '/health',
          name: 'Health Check',
          description: 'Check service health status',
          auth: false,
          request: null,
          response: {
            status: 'string',
            service: 'string',
            version: 'string'
          }
        },
        {
          method: 'POST',
          path: '/auth/register',
          name: 'Register User',
          description: 'Register a new user account',
          auth: false,
          request: {
            username: 'string (required)',
            email: 'string (required)',
            password: 'string (required)'
          },
          response: {
            success: 'boolean',
            user_id: 'number'
          }
        },
        {
          method: 'POST',
          path: '/auth/chat',
          name: 'Chat',
          description: 'Chat with AI assistant',
          auth: false,
          request: {
            message: 'string (required)'
          },
          response: {
            response: 'string'
          }
        },
        // User Endpoints
        {
          method: 'GET',
          path: '/rag-models',
          name: 'Get Public RAG Models',
          description: 'Get available RAG models for users',
          auth: true,
          request: null,
          response: {
            rag_models: 'array of RAG model objects'
          }
        },
        // Admin Dashboard
        {
          method: 'GET',
          path: '/admin/overview',
          name: 'Admin Overview',
          description: 'Get admin dashboard overview data',
          auth: 'admin',
          request: null,
          response: {
            stats: 'object with system statistics'
          }
        },
        {
          method: 'GET',
          path: '/admin/system/health',
          name: 'System Health',
          description: 'Get comprehensive system health information',
          auth: 'admin',
          request: null,
          response: {
            services: 'object with service health status',
            database: 'object with database status'
          }
        },
        {
          method: 'GET',
          path: '/admin/system/stats',
          name: 'System Stats',
          description: 'Get detailed system statistics',
          auth: 'admin',
          request: null,
          response: {
            uptime: 'string',
            memory_usage: 'object',
            cpu_usage: 'number'
          }
        },
        // User Management
        {
          method: 'GET',
          path: '/admin/users',
          name: 'List Users',
          description: 'Get all users in the system',
          auth: 'admin',
          request: null,
          response: {
            users: 'array of user objects'
          }
        },
        {
          method: 'POST',
          path: '/admin/users',
          name: 'Create Admin User',
          description: 'Create a new admin user',
          auth: 'admin',
          request: {
            username: 'string (required)',
            email: 'string (required)',
            password: 'string (required)',
            is_admin: 'boolean'
          },
          response: {
            success: 'boolean',
            user_id: 'number'
          }
        },
        {
          method: 'GET',
          path: '/admin/users/:id',
          name: 'Get User by ID',
          description: 'Get specific user by ID',
          auth: 'admin',
          request: null,
          response: {
            user: 'user object'
          }
        },
        {
          method: 'PUT',
          path: '/admin/users/:id',
          name: 'Update User',
          description: 'Update user information',
          auth: 'admin',
          request: {
            is_active: 'boolean (optional)',
            is_admin: 'boolean (optional)'
          },
          response: {
            success: 'boolean',
            message: 'string'
          }
        },
        {
          method: 'DELETE',
          path: '/admin/users/:id',
          name: 'Delete User',
          description: 'Delete a user account',
          auth: 'admin',
          request: null,
          response: {
            success: 'boolean',
            message: 'string'
          }
        },
        {
          method: 'GET',
          path: '/admin/users/:id/chat-history',
          name: 'Get User Chat History',
          description: 'Get chat history for a specific user',
          auth: 'admin',
          request: null,
          response: {
            user_id: 'number',
            chat_history: 'array of chat records'
          }
        },
        // Configuration Management (Legacy)
        {
          method: 'GET',
          path: '/admin/configs',
          name: 'List Configs',
          description: 'Get all configuration entries',
          auth: 'admin',
          request: null,
          response: {
            configs: 'array of config objects'
          }
        },
        {
          method: 'POST',
          path: '/admin/configs',
          name: 'Create Config',
          description: 'Create a new configuration entry',
          auth: 'admin',
          request: {
            key: 'string (required)',
            value: 'string (required)',
            description: 'string (optional)'
          },
          response: {
            success: 'boolean',
            message: 'string',
            key: 'string'
          }
        },
        {
          method: 'GET',
          path: '/admin/configs/backup',
          name: 'Backup Configs',
          description: 'Create backup of all configurations',
          auth: 'admin',
          request: null,
          response: {
            backup_data: 'array of config objects',
            backup_timestamp: 'string (ISO datetime)',
            total_configs: 'number'
          }
        },
        {
          method: 'GET',
          path: '/admin/configs/:key',
          name: 'Get Config by Key',
          description: 'Get specific configuration by key',
          auth: 'admin',
          request: null,
          response: {
            config: 'config object'
          }
        },
        {
          method: 'PUT',
          path: '/admin/configs/:key',
          name: 'Update Config',
          description: 'Update configuration entry',
          auth: 'admin',
          request: {
            value: 'string (required)',
            description: 'string (optional)'
          },
          response: {
            success: 'boolean',
            message: 'string'
          }
        },
        {
          method: 'DELETE',
          path: '/admin/configs/:key',
          name: 'Delete Config',
          description: 'Delete configuration entry',
          auth: 'admin',
          request: null,
          response: {
            success: 'boolean',
            message: 'string'
          }
        },
        // Environment Configuration
        {
          method: 'GET',
          path: '/admin/env-configs',
          name: 'List Environment Configs',
          description: 'Get all environment configuration files',
          auth: 'admin',
          request: null,
          response: {
            configs: 'array of environment config objects'
          }
        },
        {
          method: 'GET',
          path: '/admin/env-configs/validate',
          name: 'Validate Environment Configs',
          description: 'Validate all environment configuration files',
          auth: 'admin',
          request: null,
          response: {
            validation_results: 'array of validation objects'
          }
        },
        {
          method: 'PUT',
          path: '/admin/env-configs/update',
          name: 'Update Environment Config',
          description: 'Update environment configuration',
          auth: 'admin',
          request: {
            key: 'string (required)',
            value: 'string (required)'
          },
          response: {
            success: 'boolean',
            message: 'string',
            updated_files: 'array of filenames'
          }
        },
        {
          method: 'GET',
          path: '/admin/env-configs/:key',
          name: 'Get Environment Config by Key',
          description: 'Get specific environment configuration by key',
          auth: 'admin',
          request: null,
          response: {
            key: 'string',
            value: 'string',
            files: 'array of filenames'
          }
        },
        // Document Management
        {
          method: 'GET',
          path: '/admin/documents/folders',
          name: 'List Folders',
          description: 'Get all document folders',
          auth: 'admin',
          request: null,
          response: {
            folders: 'array of folder objects'
          }
        },
        {
          method: 'POST',
          path: '/admin/documents/folders',
          name: 'Create Folder',
          description: 'Create a new document folder',
          auth: 'admin',
          request: {
            name: 'string (required)'
          },
          response: {
            success: 'boolean',
            message: 'string',
            folder_name: 'string',
            folder_path: 'string'
          }
        },
        {
          method: 'GET',
          path: '/admin/documents/folders/:folder/documents',
          name: 'List Documents in Folder',
          description: 'Get all documents in a specific folder',
          auth: 'admin',
          request: null,
          response: {
            documents: 'array of document objects'
          }
        },
        {
          method: 'POST',
          path: '/admin/documents/folders/:folder/upload',
          name: 'Upload Document',
          description: 'Upload a document to a specific folder',
          auth: 'admin',
          request: {
            file: 'multipart form data (required)'
          },
          response: {
            success: 'boolean',
            message: 'string',
            uploaded_files: 'array of uploaded file objects'
          }
        },
        {
          method: 'DELETE',
          path: '/admin/documents/folders/:folder/documents/:filename',
          name: 'Delete Document',
          description: 'Delete a specific document',
          auth: 'admin',
          request: null,
          response: {
            success: 'boolean',
            message: 'string'
          }
        },
        {
          method: 'DELETE',
          path: '/admin/documents/folders/:folder',
          name: 'Delete Folder',
          description: 'Delete a document folder and all its contents',
          auth: 'admin',
          request: null,
          response: {
            success: 'boolean',
            message: 'string'
          }
        },
        // Vector Management
        {
          method: 'GET',
          path: '/admin/vectors',
          name: 'List Vectors',
          description: 'Get all vector configurations',
          auth: 'admin',
          request: null,
          response: {
            vectors: 'array of vector objects'
          }
        },
        {
          method: 'POST',
          path: '/admin/vectors',
          name: 'Create Vector',
          description: 'Create a new vector configuration',
          auth: 'admin',
          request: {
            name: 'string (required)',
            folder_name: 'string (required)'
          },
          response: {
            success: 'boolean',
            vector_id: 'number',
            message: 'string'
          }
        },
        {
          method: 'GET',
          path: '/admin/vectors/:id',
          name: 'Get Vector by ID',
          description: 'Get specific vector configuration by ID',
          auth: 'admin',
          request: null,
          response: {
            vector: 'vector object'
          }
        },
        {
          method: 'DELETE',
          path: '/admin/vectors/:id',
          name: 'Delete Vector',
          description: 'Delete a vector configuration',
          auth: 'admin',
          request: null,
          response: {
            success: 'boolean',
            message: 'string'
          }
        },
        // RAG Model Management
        {
          method: 'GET',
          path: '/admin/rag-models',
          name: 'List RAG Models (Admin)',
          description: 'Get all RAG models for admin management',
          auth: 'admin',
          request: null,
          response: {
            rag_models: 'array of RAG model objects'
          }
        },
        {
          method: 'POST',
          path: '/admin/rag-models',
          name: 'Create RAG Model',
          description: 'Create a new RAG model',
          auth: 'admin',
          request: {
            name: 'string (required)',
            vector_id: 'number (required)',
            system_prompt: 'string (required)',
            context: 'string (optional)'
          },
          response: {
            success: 'boolean',
            rag_model_id: 'number',
            message: 'string'
          }
        },
        {
          method: 'GET',
          path: '/admin/rag-models/:id',
          name: 'Get RAG Model by ID',
          description: 'Get specific RAG model by ID',
          auth: 'admin',
          request: null,
          response: {
            rag_model: 'RAG model object'
          }
        },
        {
          method: 'PUT',
          path: '/admin/rag-models/:id',
          name: 'Update RAG Model',
          description: 'Update RAG model configuration',
          auth: 'admin',
          request: {
            name: 'string (optional)',
            system_prompt: 'string (optional)',
            context: 'string (optional)',
            is_active: 'boolean (optional)'
          },
          response: {
            success: 'boolean',
            message: 'string'
          }
        },
        {
          method: 'DELETE',
          path: '/admin/rag-models/:id',
          name: 'Delete RAG Model',
          description: 'Delete a RAG model',
          auth: 'admin',
          request: null,
          response: {
            success: 'boolean',
            message: 'string'
          }
        }
      ]
    }
  };

  // Generate Postman Collection
  const generatePostmanCollection = () => {
    const collection = {
      info: {
        name: 'AI Rust API Collection',
        description: 'Complete API collection for AI Rust microservices',
        version: '1.0.0',
        schema: 'https://schema.getpostman.com/json/collection/v2.1.0/collection.json'
      },
      auth: {
        type: 'bearer',
        bearer: [
          {
            key: 'token',
            value: '{{auth_token}}',
            type: 'string'
          }
        ]
      },
      variable: [
        {
          key: 'bedrock_base_url',
          value: 'http://localhost:9100'
        },
        {
          key: 'rag_base_url',
          value: 'http://localhost:9101'
        },
        {
          key: 'ui_config_base_url',
          value: 'http://localhost:9103'
        },
        {
          key: 'auth_token',
          value: 'your-jwt-token-here'
        }
      ],
      item: []
    };

    Object.entries(apiServices).forEach(([serviceKey, service]) => {
      const serviceFolder = {
        name: service.name,
        item: []
      };

      service.endpoints.forEach(endpoint => {
        const request = {
          name: endpoint.name,
          request: {
            method: endpoint.method,
            header: [],
            url: {
              raw: `{{${serviceKey.replace('Api', '').toLowerCase()}_base_url}}${endpoint.path}`,
              host: [`{{${serviceKey.replace('Api', '').toLowerCase()}_base_url}}`],
              path: endpoint.path.split('/').filter(p => p)
            }
          }
        };

        if (endpoint.auth) {
          request.request.auth = {
            type: 'bearer',
            bearer: [
              {
                key: 'token',
                value: '{{auth_token}}',
                type: 'string'
              }
            ]
          };
        }

        if (endpoint.request && endpoint.method === 'POST') {
          request.request.header.push({
            key: 'Content-Type',
            value: 'application/json'
          });
          request.request.body = {
            mode: 'raw',
            raw: JSON.stringify(endpoint.request, null, 2),
            options: {
              raw: {
                language: 'json'
              }
            }
          };
        }

        serviceFolder.item.push(request);
      });

      collection.item.push(serviceFolder);
    });

    return JSON.stringify(collection, null, 2);
  };

  // Generate OpenAPI Specification
  const generateSwaggerSpec = () => {
    const swagger = {
      openapi: '3.0.0',
      info: {
        title: 'AI Rust API',
        description: 'Comprehensive API documentation for AI Rust microservices architecture',
        version: '1.0.0',
        contact: {
          name: 'API Support',
          email: 'support@example.com'
        }
      },
      servers: [
        {
          url: 'http://localhost:9102',
          description: 'Authentication API Server'
        },
        {
          url: 'http://localhost:9100',
          description: 'Bedrock AI Chat API Server'
        },
        {
          url: 'http://localhost:9101',
          description: 'RAG Document API Server'
        },
        {
          url: 'http://localhost:9103',
          description: 'UI Configuration API Server'
        }
      ],
      components: {
        securitySchemes: {
          bearerAuth: {
            type: 'http',
            scheme: 'bearer',
            bearerFormat: 'JWT'
          }
        },
        schemas: {
          Error: {
            type: 'object',
            properties: {
              error: {
                type: 'string'
              },
              message: {
                type: 'string'
              }
            }
          }
        }
      },
      paths: {}
    };

    Object.entries(apiServices).forEach(([serviceKey, service]) => {
      service.endpoints.forEach(endpoint => {
        const pathKey = endpoint.path;
        const method = endpoint.method.toLowerCase();

        if (!swagger.paths[pathKey]) {
          swagger.paths[pathKey] = {};
        }

        const operation = {
          summary: endpoint.name,
          description: endpoint.description,
          tags: [service.name],
          responses: {
            '200': {
              description: 'Success',
              content: {
                'application/json': {
                  schema: {
                    type: 'object',
                    properties: endpoint.response || {}
                  }
                }
              }
            },
            '400': {
              description: 'Bad Request',
              content: {
                'application/json': {
                  schema: {
                    $ref: '#/components/schemas/Error'
                  }
                }
              }
            },
            '401': {
              description: 'Unauthorized',
              content: {
                'application/json': {
                  schema: {
                    $ref: '#/components/schemas/Error'
                  }
                }
              }
            },
            '500': {
              description: 'Internal Server Error',
              content: {
                'application/json': {
                  schema: {
                    $ref: '#/components/schemas/Error'
                  }
                }
              }
            }
          }
        };

        if (endpoint.auth) {
          operation.security = [{ bearerAuth: [] }];
        }

        if (endpoint.request && method === 'post') {
          operation.requestBody = {
            required: true,
            content: {
              'application/json': {
                schema: {
                  type: 'object',
                  properties: Object.fromEntries(
                    Object.entries(endpoint.request).map(([key, value]) => [
                      key,
                      { type: typeof value === 'string' ? 'string' : 'number' }
                    ])
                  )
                }
              }
            }
          };
        }

        swagger.paths[pathKey][method] = operation;
      });
    });

    return JSON.stringify(swagger, null, 2);
  };

  // Generate cURL examples
  const generateCurlExample = (service, endpoint) => {
    const baseUrl = service.baseUrl;
    let curl = `curl -X ${endpoint.method} "${baseUrl}${endpoint.path}"`;
    
    if (endpoint.auth) {
      curl += ' \\\n  -H "Authorization: Bearer YOUR_JWT_TOKEN"';
    }

    if (endpoint.request && endpoint.method === 'POST') {
      curl += ' \\\n  -H "Content-Type: application/json"';
      curl += ` \\\n  -d '${JSON.stringify(endpoint.request, null, 2)}'`;
    }

    return curl;
  };

  return (
    <Box sx={{ flexGrow: 1 }}>
      <Container maxWidth="xl" sx={{ p: 3 }}>
        <Paper elevation={3}>
          <Tabs
            value={activeTab}
            onChange={handleTabChange}
            sx={{ borderBottom: 1, borderColor: 'divider' }}
          >
            <Tab icon={<DescriptionIcon />} label="API Documentation" />
            <Tab icon={<CodeIcon />} label="Postman Collection" />
            <Tab icon={<ApiIcon />} label="Swagger/OpenAPI" />
          </Tabs>

          {/* API Documentation Tab */}
          {activeTab === 0 && (
            <Box sx={{ p: 3 }}>
              <Typography variant="h4" gutterBottom>
                AI Rust API Documentation
              </Typography>
              <Typography variant="body1" color="textSecondary" paragraph>
                Comprehensive REST API documentation for the AI Rust microservices architecture.
              </Typography>

              <Alert severity="info" sx={{ mb: 3 }}>
                <Typography variant="body2">
                  <strong>Authentication:</strong> Most endpoints require a Bearer token in the Authorization header.
                  Get your token by authenticating through the UI or using the login endpoint.
                </Typography>
              </Alert>

              {Object.entries(apiServices).map(([serviceKey, service]) => (
                <Card key={serviceKey} sx={{ mb: 3 }}>
                  <CardContent>
                    <Box display="flex" alignItems="center" mb={2}>
                      {service.icon}
                      <Typography variant="h5" sx={{ ml: 1 }}>
                        {service.name}
                      </Typography>
                      <Chip label={service.baseUrl} sx={{ ml: 2 }} />
                    </Box>
                    <Typography variant="body2" color="textSecondary" paragraph>
                      {service.description}
                    </Typography>

                    {service.endpoints.map((endpoint, index) => (
                      <Accordion key={index} sx={{ mb: 1 }}>
                        <AccordionSummary
                          expandIcon={<ExpandMoreIcon />}
                          sx={{
                            bgcolor: endpoint.method === 'GET' ? 'success.light' : 'info.light',
                            '&:hover': { bgcolor: endpoint.method === 'GET' ? 'success.main' : 'info.main' }
                          }}
                        >
                          <Box display="flex" alignItems="center" gap={2}>
                            <Chip
                              label={endpoint.method}
                              size="small"
                              color={endpoint.method === 'GET' ? 'success' : 'primary'}
                            />
                            <Typography variant="body1" fontWeight="medium">
                              {endpoint.path}
                            </Typography>
                            <Typography variant="body2" color="textSecondary">
                              {endpoint.name}
                            </Typography>
                            {endpoint.auth && <SecurityIcon fontSize="small" />}
                            {endpoint.adminOnly && <Chip label="Admin Only" size="small" color="warning" />}
                          </Box>
                        </AccordionSummary>
                        <AccordionDetails>
                          <Grid container spacing={3}>
                            <Grid item xs={12}>
                              <Typography variant="body1" paragraph>
                                {endpoint.description}
                              </Typography>
                            </Grid>

                            {endpoint.request && (
                              <Grid item xs={12} md={6}>
                                <Typography variant="h6" gutterBottom>
                                  Request Body
                                </Typography>
                                <Paper sx={{ p: 2, bgcolor: 'grey.50' }}>
                                  <pre style={{ margin: 0, fontSize: '0.875rem' }}>
                                    {JSON.stringify(endpoint.request, null, 2)}
                                  </pre>
                                </Paper>
                              </Grid>
                            )}

                            <Grid item xs={12} md={endpoint.request ? 6 : 12}>
                              <Typography variant="h6" gutterBottom>
                                Response
                              </Typography>
                              <Paper sx={{ p: 2, bgcolor: 'grey.50' }}>
                                <pre style={{ margin: 0, fontSize: '0.875rem' }}>
                                  {JSON.stringify(endpoint.response, null, 2)}
                                </pre>
                              </Paper>
                            </Grid>

                            <Grid item xs={12}>
                              <Typography variant="h6" gutterBottom>
                                cURL Example
                              </Typography>
                              <Paper sx={{ p: 2, bgcolor: 'grey.900', color: 'white' }}>
                                <pre style={{ margin: 0, fontSize: '0.875rem' }}>
                                  {generateCurlExample(service, endpoint)}
                                </pre>
                                <Tooltip title={copiedText === `curl-${serviceKey}-${index}` ? 'Copied!' : 'Copy cURL'}>
                                  <IconButton
                                    size="small"
                                    sx={{ color: 'white', float: 'right' }}
                                    onClick={() => handleCopyToClipboard(
                                      generateCurlExample(service, endpoint),
                                      `curl-${serviceKey}-${index}`
                                    )}
                                  >
                                    <ContentCopyIcon fontSize="small" />
                                  </IconButton>
                                </Tooltip>
                              </Paper>
                            </Grid>
                          </Grid>
                        </AccordionDetails>
                      </Accordion>
                    ))}
                  </CardContent>
                </Card>
              ))}
            </Box>
          )}

          {/* Postman Collection Tab */}
          {activeTab === 1 && (
            <Box sx={{ p: 3 }}>
              <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
                <Typography variant="h4">
                  Postman Collection
                </Typography>
                <Button
                  variant="contained"
                  startIcon={<DownloadIcon />}
                  onClick={async () => {
                    try {
                      const response = await fetch('/api-docs/AI-Rust-API.postman_collection.json');
                      if (response.ok) {
                        const collection = await response.text();
                        downloadFile(collection, 'AI-Rust-API.postman_collection.json', 'application/json');
                      } else {
                        downloadFile(generatePostmanCollection(), 'AI-Rust-API-Collection.postman_collection.json', 'application/json');
                      }
                    } catch (error) {
                      downloadFile(generatePostmanCollection(), 'AI-Rust-API-Collection.postman_collection.json', 'application/json');
                    }
                  }}
                >
                  Download Collection
                </Button>
              </Box>

              <Alert severity="info" sx={{ mb: 3 }}>
                <Typography variant="body2">
                  This Postman collection includes all API endpoints with pre-configured variables.
                  Import into Postman and set the <code>auth_token</code> variable with your JWT token.
                </Typography>
              </Alert>

              <Typography variant="h6" gutterBottom>
                Collection Overview
              </Typography>
              <List>
                {Object.entries(apiServices).map(([serviceKey, service]) => (
                  <ListItem key={serviceKey}>
                    <ListItemText
                      primary={service.name}
                      secondary={`${service.endpoints.length} endpoints - ${service.baseUrl}`}
                    />
                    <Chip label={`${service.endpoints.length} endpoints`} />
                  </ListItem>
                ))}
              </List>

              <Typography variant="h6" gutterBottom sx={{ mt: 3 }}>
                Setup Instructions
              </Typography>
              <Paper sx={{ p: 2 }}>
                <ol>
                  <li>Download the Postman collection using the button above</li>
                  <li>Import the collection into Postman</li>
                  <li>Set up environment variables:
                    <ul>
                      <li><code>auth_token</code>: Your JWT token from login</li>
                      <li><code>bedrock_base_url</code>: http://localhost:9100</li>
                      <li><code>rag_base_url</code>: http://localhost:9101</li>
                      <li><code>ui_config_base_url</code>: http://localhost:9103</li>
                    </ul>
                  </li>
                  <li>Start testing the APIs!</li>
                </ol>
              </Paper>

              <TextField
                fullWidth
                multiline
                rows={15}
                value={collectionPreview}
                variant="outlined"
                sx={{ mt: 3 }}
                InputProps={{
                  readOnly: true,
                  style: { fontSize: '0.75rem', fontFamily: 'monospace' }
                }}
              />
            </Box>
          )}

          {/* Swagger/OpenAPI Tab */}
          {activeTab === 2 && (
            <Box sx={{ p: 3 }}>
              <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
                <Typography variant="h4">
                  OpenAPI 3.0 Specification
                </Typography>
                <Box>
                  <Button
                    variant="contained"
                    startIcon={<DownloadIcon />}
                    onClick={() => downloadFile(
                      generateSwaggerSpec(),
                      'ai-rust-api-swagger.json',
                      'application/json'
                    )}
                    sx={{ mr: 2 }}
                  >
                    Download Swagger JSON
                  </Button>
                  <Button
                    variant="outlined"
                    startIcon={<LaunchIcon />}
                    href={`https://editor.swagger.io/?url=${encodeURIComponent(window.location.origin)}/swagger.json`}
                    target="_blank"
                  >
                    Open in Swagger Editor
                  </Button>
                </Box>
              </Box>

              <Alert severity="info" sx={{ mb: 3 }}>
                <Typography variant="body2">
                  OpenAPI specification compatible with Swagger UI and other API tools.
                  Download the JSON file or view it in the Swagger Editor for interactive documentation.
                </Typography>
              </Alert>

              <Typography variant="h6" gutterBottom>
                Specification Features
              </Typography>
              <Grid container spacing={2} sx={{ mb: 3 }}>
                <Grid item xs={12} sm={6} md={3}>
                  <Card>
                    <CardContent sx={{ textAlign: 'center' }}>
                      <ApiIcon sx={{ fontSize: 40, color: 'primary.main' }} />
                      <Typography variant="h6">
                        {Object.values(apiServices).reduce((sum, service) => sum + service.endpoints.length, 0)}
                      </Typography>
                      <Typography variant="body2" color="textSecondary">
                        Total Endpoints
                      </Typography>
                    </CardContent>
                  </Card>
                </Grid>
                <Grid item xs={12} sm={6} md={3}>
                  <Card>
                    <CardContent sx={{ textAlign: 'center' }}>
                      <SettingsIcon sx={{ fontSize: 40, color: 'secondary.main' }} />
                      <Typography variant="h6">
                        {Object.keys(apiServices).length}
                      </Typography>
                      <Typography variant="body2" color="textSecondary">
                        Microservices
                      </Typography>
                    </CardContent>
                  </Card>
                </Grid>
                <Grid item xs={12} sm={6} md={3}>
                  <Card>
                    <CardContent sx={{ textAlign: 'center' }}>
                      <SecurityIcon sx={{ fontSize: 40, color: 'warning.main' }} />
                      <Typography variant="h6">JWT</Typography>
                      <Typography variant="body2" color="textSecondary">
                        Authentication
                      </Typography>
                    </CardContent>
                  </Card>
                </Grid>
                <Grid item xs={12} sm={6} md={3}>
                  <Card>
                    <CardContent sx={{ textAlign: 'center' }}>
                      <CodeIcon sx={{ fontSize: 40, color: 'success.main' }} />
                      <Typography variant="h6">3.0</Typography>
                      <Typography variant="body2" color="textSecondary">
                        OpenAPI Version
                      </Typography>
                    </CardContent>
                  </Card>
                </Grid>
              </Grid>

              <TextField
                fullWidth
                multiline
                rows={20}
                value={generateSwaggerSpec()}
                variant="outlined"
                InputProps={{
                  readOnly: true,
                  style: { fontSize: '0.75rem', fontFamily: 'monospace' }
                }}
              />
            </Box>
          )}
        </Paper>
      </Container>
    </Box>
  );
};

export default ApiDocumentation;