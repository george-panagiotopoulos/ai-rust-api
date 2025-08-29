import React, { useState } from 'react';
import {
  Container,
  Paper,
  TextField,
  Button,
  Typography,
  Box,
  Alert,
  Link,
  ToggleButton,
  ToggleButtonGroup
} from '@mui/material';
import { useNavigate, Link as RouterLink } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';

const Login = () => {
  const navigate = useNavigate();
  const { login } = useAuth();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loginType, setLoginType] = useState('user');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    const result = await login(username, password, loginType === 'admin');
    
    if (result.success) {
      navigate(loginType === 'admin' ? '/admin' : '/dashboard');
    } else {
      setError(result.message || 'Login failed');
    }
    
    setLoading(false);
  };

  const handleLoginTypeChange = (event, newType) => {
    if (newType !== null) {
      setLoginType(newType);
      setError('');
    }
  };

  return (
    <Container component="main" maxWidth="sm">
      <Box
        sx={{
          marginTop: 8,
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
        }}
      >
        <Paper elevation={3} sx={{ padding: 4, width: '100%' }}>
          <Typography component="h1" variant="h4" align="center" gutterBottom>
            AI Rust API
          </Typography>
          
          <Typography component="h2" variant="h5" align="center" gutterBottom>
            Sign In
          </Typography>

          <Box sx={{ display: 'flex', justifyContent: 'center', mb: 3 }}>
            <ToggleButtonGroup
              value={loginType}
              exclusive
              onChange={handleLoginTypeChange}
              aria-label="login type"
            >
              <ToggleButton value="user" aria-label="user login">
                User Login
              </ToggleButton>
              <ToggleButton value="admin" aria-label="admin login">
                Admin Login
              </ToggleButton>
            </ToggleButtonGroup>
          </Box>

          {error && (
            <Alert severity="error" sx={{ mb: 2 }}>
              {error}
            </Alert>
          )}

          <Box component="form" onSubmit={handleSubmit} sx={{ mt: 1 }}>
            <TextField
              margin="normal"
              required
              fullWidth
              id="username"
              label="Username"
              name="username"
              autoComplete="username"
              autoFocus
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
            <TextField
              margin="normal"
              required
              fullWidth
              name="password"
              label="Password"
              type="password"
              id="password"
              autoComplete="current-password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
            <Button
              type="submit"
              fullWidth
              variant="contained"
              sx={{ mt: 3, mb: 2 }}
              disabled={loading}
            >
              {loading ? 'Signing In...' : `Sign In as ${loginType === 'admin' ? 'Admin' : 'User'}`}
            </Button>
            
            {loginType === 'user' && (
              <Box textAlign="center">
                <Link component={RouterLink} to="/register" variant="body2">
                  {"Don't have an account? Sign Up"}
                </Link>
              </Box>
            )}
          </Box>
        </Paper>
      </Box>
    </Container>
  );
};

export default Login;