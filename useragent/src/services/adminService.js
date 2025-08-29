import axios from 'axios';

const UI_CONFIG_API_URL = process.env.REACT_APP_UI_CONFIG_API_URL || 'http://localhost:9103';

class AdminService {
  constructor() {
    this.baseURL = UI_CONFIG_API_URL;
  }

  getAuthHeaders() {
    const token = localStorage.getItem('token');
    return {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    };
  }

  // Dashboard Overview
  async getOverview() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/overview`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch overview');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async getSystemHealth() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/system/health`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch system health');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async getSystemStats() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/system/stats`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch system stats');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  // User Management
  async getUsers() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/users`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch users');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async getUser(userId) {
    try {
      const response = await axios.get(`${this.baseURL}/admin/users/${userId}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch user');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async updateUser(userId, userData) {
    try {
      const response = await axios.put(`${this.baseURL}/admin/users/${userId}`, userData, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to update user');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async deleteUser(userId) {
    try {
      const response = await axios.delete(`${this.baseURL}/admin/users/${userId}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to delete user');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async createAdmin(adminData) {
    try {
      const response = await axios.post(`${this.baseURL}/admin/users`, adminData, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to create admin user');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async getUserChatHistory(userId) {
    try {
      const response = await axios.get(`${this.baseURL}/admin/users/${userId}/chat-history`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch chat history');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  // Configuration Management
  async getConfigs() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/configs`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch configurations');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async getConfig(key) {
    try {
      const response = await axios.get(`${this.baseURL}/admin/configs/${encodeURIComponent(key)}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch configuration');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async createConfig(configData) {
    try {
      const response = await axios.post(`${this.baseURL}/admin/configs`, configData, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to create configuration');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async updateConfig(key, configData) {
    try {
      const response = await axios.put(`${this.baseURL}/admin/configs/${encodeURIComponent(key)}`, configData, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to update configuration');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async deleteConfig(key) {
    try {
      const response = await axios.delete(`${this.baseURL}/admin/configs/${encodeURIComponent(key)}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to delete configuration');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async backupConfigs() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/configs/backup`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to backup configurations');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  // Document Management
  async getFolders() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/documents/folders`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch folders');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async createFolder(folderData) {
    try {
      const response = await axios.post(`${this.baseURL}/admin/documents/folders`, folderData, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to create folder');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async getDocuments(folderName) {
    try {
      const response = await axios.get(`${this.baseURL}/admin/documents/folders/${encodeURIComponent(folderName)}/documents`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch documents');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async uploadDocument(folderName, file) {
    try {
      const formData = new FormData();
      formData.append('file', file);

      const response = await axios.post(`${this.baseURL}/admin/documents/folders/${encodeURIComponent(folderName)}/upload`, formData, {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
          'Content-Type': 'multipart/form-data'
        }
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to upload document');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async deleteDocument(folderName, filename) {
    try {
      const response = await axios.delete(`${this.baseURL}/admin/documents/folders/${encodeURIComponent(folderName)}/documents/${encodeURIComponent(filename)}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to delete document');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async deleteFolder(folderName) {
    try {
      const response = await axios.delete(`${this.baseURL}/admin/documents/folders/${encodeURIComponent(folderName)}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to delete folder');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  // New .env configuration management methods
  async getEnvConfigs() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/env-configs`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch .env configurations');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async getEnvConfig(key) {
    try {
      const response = await axios.get(`${this.baseURL}/admin/env-configs/${encodeURIComponent(key)}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch .env configuration');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async updateEnvConfig(key, value) {
    try {
      const response = await axios.put(`${this.baseURL}/admin/env-configs/update`, {
        key: key,
        value: value
      }, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to update .env configuration');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async validateEnvConfigs() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/env-configs/validate`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to validate .env configurations');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  // Vector Management
  async getVectors() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/vectors`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch vectors');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async createVector(vectorData) {
    try {
      const response = await axios.post(`${this.baseURL}/admin/vectors`, vectorData, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to create vector');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async deleteVector(vectorId) {
    try {
      const response = await axios.delete(`${this.baseURL}/admin/vectors/${vectorId}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to delete vector');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  // RAG Model Management
  async getRagModels() {
    try {
      const response = await axios.get(`${this.baseURL}/admin/rag-models`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to fetch RAG models');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async createRagModel(ragModelData) {
    try {
      const response = await axios.post(`${this.baseURL}/admin/rag-models`, ragModelData, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to create RAG model');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async updateRagModel(modelId, ragModelData) {
    try {
      const response = await axios.put(`${this.baseURL}/admin/rag-models/${modelId}`, ragModelData, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to update RAG model');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async deleteRagModel(modelId) {
    try {
      const response = await axios.delete(`${this.baseURL}/admin/rag-models/${modelId}`, {
        headers: this.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Failed to delete RAG model');
      }
      throw new Error('Network error. Please try again.');
    }
  }
}

const adminService = new AdminService();
export default adminService;