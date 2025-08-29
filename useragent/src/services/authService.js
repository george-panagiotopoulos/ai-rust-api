import axios from 'axios';

const AUTH_API_URL = process.env.REACT_APP_AUTH_API_URL || 'http://localhost:9102';
const UI_CONFIG_API_URL = process.env.REACT_APP_UI_CONFIG_API_URL || 'http://localhost:9103';

class AuthService {
  async login(username, password, isAdmin = false) {
    try {
      const response = await axios.post(`${AUTH_API_URL}/login`, {
        username,
        password
      });

      if (response.data.access_token && response.data.user) {
        return {
          token: response.data.access_token,
          user: response.data.user
        };
      } else {
        throw new Error('Login failed - invalid response format');
      }
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Login failed');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async register(username, email, password) {
    try {
      const response = await axios.post(`${AUTH_API_URL}/register`, {
        username,
        email,
        password
      });

      // API returns user object on success, wrap it in success format
      if (response.data && response.data.id) {
        return {
          success: true,
          user: response.data,
          message: 'Registration successful'
        };
      } else {
        throw new Error('Registration failed - invalid response format');
      }
    } catch (error) {
      if (error.response && error.response.data) {
        throw new Error(error.response.data.error || error.response.data.message || 'Registration failed');
      }
      throw new Error('Network error. Please try again.');
    }
  }

  async validateToken(token) {
    try {
      const response = await axios.post(`${AUTH_API_URL}/validate`, 
        { token },
        {
          headers: {
            'Authorization': `Bearer ${token}`
          }
        }
      );

      if (response.data.valid && response.data.user) {
        return response.data.user;
      } else {
        throw new Error('Invalid token');
      }
    } catch (error) {
      throw new Error('Token validation failed');
    }
  }

  getAuthHeaders() {
    const token = localStorage.getItem('token');
    return token ? { Authorization: `Bearer ${token}` } : {};
  }
}

export default new AuthService();