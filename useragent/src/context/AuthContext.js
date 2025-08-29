import React, { createContext, useContext, useState, useEffect } from 'react';
import authService from '../services/authService';

const AuthContext = createContext({});

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

export const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) {
      // Set a timeout to prevent hanging indefinitely
      const timeoutId = setTimeout(() => {
        console.warn('Token validation timed out');
        setLoading(false);
      }, 5000);

      authService.validateToken(token)
        .then((userData) => {
          clearTimeout(timeoutId);
          setUser(userData);
          setIsAuthenticated(true);
        })
        .catch(() => {
          clearTimeout(timeoutId);
          localStorage.removeItem('token');
        })
        .finally(() => {
          setLoading(false);
        });
    } else {
      setLoading(false);
    }
  }, []);

  const login = async (username, password, isAdmin = false) => {
    try {
      const { token, user: userData } = await authService.login(username, password, isAdmin);
      localStorage.setItem('token', token);
      setUser(userData);
      setIsAuthenticated(true);
      return { success: true };
    } catch (error) {
      return { success: false, message: error.message };
    }
  };

  const register = async (username, email, password) => {
    try {
      const result = await authService.register(username, email, password);
      return result;
    } catch (error) {
      return { success: false, message: error.message };
    }
  };

  const logout = () => {
    localStorage.removeItem('token');
    setUser(null);
    setIsAuthenticated(false);
  };

  const value = {
    user,
    isAuthenticated,
    loading,
    login,
    register,
    logout,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};