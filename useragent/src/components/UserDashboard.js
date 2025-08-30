import React, { useState, useEffect, useRef } from 'react';
import {
  Container,
  Paper,
  TextField,
  Button,
  Typography,
  Box,
  AppBar,
  Toolbar,
  IconButton,
  Alert,
  CircularProgress,
  List,
  ListItem,
  ListItemText,
  Divider,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Switch,
  FormControlLabel,
  Chip,
  Tooltip,
  Tabs,
  Tab
} from '@mui/material';
import { Logout, Send, SmartToy, Chat, Psychology, Api as ApiIcon } from '@mui/icons-material';
import { useAuth } from '../context/AuthContext';
import ApiDocumentation from './ApiDocumentation';
import axios from 'axios';

function TabPanel({ children, value, index, ...other }) {
  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`user-tabpanel-${index}`}
      aria-labelledby={`user-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box>
          {children}
        </Box>
      )}
    </div>
  );
}

const UserDashboard = () => {
  const { user, logout } = useAuth();
  const [activeTab, setActiveTab] = useState(0);
  const [message, setMessage] = useState('');
  const [chatHistory, setChatHistory] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [ragMode, setRagMode] = useState(false);
  const [ragModels, setRagModels] = useState([]);
  const [selectedRagModel, setSelectedRagModel] = useState('');
  const [ragModelsLoading, setRagModelsLoading] = useState(false);
  const messagesEndRef = useRef(null);

  const BEDROCK_API_URL = process.env.REACT_APP_BEDROCK_API_URL || 'http://localhost:9100';
  const RAG_API_URL = process.env.REACT_APP_RAG_API_URL || 'http://localhost:9101';
  const UI_CONFIG_API_URL = process.env.REACT_APP_UI_CONFIG_API_URL || 'http://localhost:9103';

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [chatHistory]);

  useEffect(() => {
    fetchRagModels();
  }, []);

  const fetchRagModels = async () => {
    try {
      setRagModelsLoading(true);
      const token = localStorage.getItem('token');
      const response = await axios.get(`${UI_CONFIG_API_URL}/rag-models`, {
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        }
      });
      
      setRagModels(response.data.rag_models || []);
    } catch (error) {
      console.error('Failed to fetch RAG models:', error);
      setRagModels([]);
    } finally {
      setRagModelsLoading(false);
    }
  };

  const sendMessage = async () => {
    if (!message.trim() || loading) return;

    // Check if RAG mode is enabled but no model is selected
    if (ragMode && !selectedRagModel) {
      setError('Please select a RAG model to continue.');
      return;
    }

    const userMessage = message;
    setMessage('');
    setError('');
    setLoading(true);

    // Add user message to chat immediately
    const chatMode = ragMode && selectedRagModel ? 'rag' : 'regular';
    setChatHistory(prev => [...prev, {
      type: 'user',
      content: userMessage,
      timestamp: new Date(),
      mode: chatMode,
      ragModel: ragMode ? ragModels.find(m => m.id === parseInt(selectedRagModel))?.name : null
    }]);

    try {
      const token = localStorage.getItem('token');
      let response;

      if (ragMode && selectedRagModel) {
        // Use RAG API
        response = await axios.post(
          `${RAG_API_URL}/query`,
          { 
            query: userMessage,
            rag_model_id: parseInt(selectedRagModel)
          },
          {
            headers: {
              'Authorization': `Bearer ${token}`,
              'Content-Type': 'application/json'
            }
          }
        );

        if (response.data.answer) {
          setChatHistory(prev => [...prev, {
            type: 'assistant',
            content: response.data.answer,
            timestamp: new Date(),
            mode: 'rag',
            sources: response.data.sources || [],
            contextUsed: response.data.context_used
          }]);
        } else {
          throw new Error('No response from RAG API');
        }
      } else {
        // Use regular Bedrock API
        response = await axios.post(
          `${BEDROCK_API_URL}/chat`,
          { message: userMessage },
          {
            headers: {
              'Authorization': `Bearer ${token}`,
              'Content-Type': 'application/json'
            }
          }
        );

        if (response.data.response) {
          setChatHistory(prev => [...prev, {
            type: 'assistant',
            content: response.data.response,
            timestamp: new Date(),
            mode: 'regular'
          }]);
        } else {
          throw new Error('No response from server');
        }
      }
    } catch (error) {
      console.error('Chat error:', error);
      setError('Failed to send message. Please try again.');
      setChatHistory(prev => [...prev, {
        type: 'error',
        content: 'Sorry, I encountered an error processing your message.',
        timestamp: new Date()
      }]);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  const handleTabChange = (event, newValue) => {
    setActiveTab(newValue);
  };

  const formatTime = (timestamp) => {
    return timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            User Dashboard - Welcome, {user?.username}
          </Typography>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
            {activeTab === 0 && (
              <Chip 
                icon={ragMode ? <Psychology /> : <Chat />}
                label={ragMode ? 'RAG Mode' : 'Regular Chat'}
                color={ragMode ? 'secondary' : 'default'}
                variant={ragMode ? 'filled' : 'outlined'}
              />
            )}
            <IconButton
              size="large"
              edge="end"
              color="inherit"
              onClick={logout}
            >
              <Logout />
            </IconButton>
          </Box>
        </Toolbar>
      </AppBar>

      <Container maxWidth="xl" sx={{ mt: 4, mb: 4 }}>
        <Paper elevation={3}>
          <Tabs
            value={activeTab}
            onChange={handleTabChange}
            sx={{ borderBottom: 1, borderColor: 'divider' }}
          >
            <Tab 
              label="Chat" 
              icon={<Chat />} 
              iconPosition="start"
              id="user-tab-0"
              aria-controls="user-tabpanel-0"
            />
            <Tab 
              label="API Documentation" 
              icon={<ApiIcon />} 
              iconPosition="start"
              id="user-tab-1"
              aria-controls="user-tabpanel-1"
            />
          </Tabs>

          <TabPanel value={activeTab} index={0}>
            <Container maxWidth="md" sx={{ p: 0 }}>
        {/* RAG Configuration Panel */}
        <Paper elevation={2} sx={{ p: 2, mb: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, flexWrap: 'wrap' }}>
            <FormControlLabel
              control={
                <Switch
                  checked={ragMode}
                  onChange={(e) => {
                    setRagMode(e.target.checked);
                    if (!e.target.checked) {
                      setSelectedRagModel('');
                    }
                    setError('');
                  }}
                  color="secondary"
                />
              }
              label={
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                  {ragMode ? <Psychology /> : <Chat />}
                  <Typography>RAG Mode</Typography>
                </Box>
              }
            />
            
            {ragMode && (
              <FormControl sx={{ minWidth: 200 }} size="small" disabled={!ragMode || ragModelsLoading}>
                <InputLabel>Select RAG Model</InputLabel>
                <Select
                  value={selectedRagModel}
                  onChange={(e) => setSelectedRagModel(e.target.value)}
                  label="Select RAG Model"
                >
                  {ragModels.map((model) => (
                    <MenuItem key={model.id} value={model.id}>
                      <Box>
                        <Typography variant="body2">{model.name}</Typography>
                        <Typography variant="caption" color="textSecondary">
                          Vector: {model.vector_name}
                        </Typography>
                      </Box>
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            )}

            {ragModelsLoading && (
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <CircularProgress size={16} />
                <Typography variant="body2">Loading models...</Typography>
              </Box>
            )}

            {ragMode && ragModels.length === 0 && !ragModelsLoading && (
              <Typography variant="body2" color="textSecondary">
                No RAG models available. Contact admin to create RAG models.
              </Typography>
            )}
          </Box>
        </Paper>

        <Paper elevation={3} sx={{ height: '70vh', display: 'flex', flexDirection: 'column' }}>
          {/* Chat Messages */}
          <Box sx={{ flexGrow: 1, overflow: 'auto', p: 2 }}>
            {chatHistory.length === 0 ? (
              <Box sx={{ textAlign: 'center', mt: 4 }}>
                <Typography variant="h6" color="textSecondary">
                  Welcome to AI Chat!
                </Typography>
                <Typography variant="body2" color="textSecondary">
                  Start a conversation by typing a message below.
                </Typography>
              </Box>
            ) : (
              <List sx={{ width: '100%' }}>
                {chatHistory.map((msg, index) => (
                  <React.Fragment key={index}>
                    <ListItem
                      sx={{
                        flexDirection: msg.type === 'user' ? 'row-reverse' : 'row',
                        alignItems: 'flex-start'
                      }}
                    >
                      <Box
                        sx={{
                          backgroundColor: 
                            msg.type === 'user' ? 'primary.main' : 
                            msg.type === 'error' ? 'error.main' : 'grey.200',
                          color: msg.type === 'user' || msg.type === 'error' ? 'white' : 'text.primary',
                          borderRadius: 2,
                          p: 2,
                          maxWidth: '70%',
                          ml: msg.type === 'user' ? 'auto' : 0,
                          mr: msg.type === 'user' ? 0 : 'auto'
                        }}
                      >
                        {/* Message mode indicator */}
                        {(msg.mode === 'rag' || msg.ragModel) && (
                          <Box sx={{ mb: 1 }}>
                            <Chip
                              icon={<Psychology />}
                              label={msg.ragModel ? `RAG: ${msg.ragModel}` : 'RAG Mode'}
                              size="small"
                              color="secondary"
                              variant="outlined"
                              sx={{ 
                                bgcolor: msg.type === 'user' ? 'rgba(255,255,255,0.2)' : 'rgba(156,39,176,0.1)',
                                color: msg.type === 'user' ? 'white' : 'secondary.main'
                              }}
                            />
                          </Box>
                        )}

                        <Typography variant="body1" sx={{ whiteSpace: 'pre-wrap' }}>
                          {msg.content}
                        </Typography>

                        {/* RAG Sources */}
                        {msg.sources && msg.sources.length > 0 && (
                          <Box sx={{ mt: 2, p: 1, bgcolor: 'rgba(0,0,0,0.05)', borderRadius: 1 }}>
                            <Typography variant="caption" sx={{ fontWeight: 'bold', display: 'block', mb: 1 }}>
                              Sources:
                            </Typography>
                            {msg.sources.slice(0, 3).map((source, idx) => (
                              <Tooltip key={idx} title={source.snippet} placement="top">
                                <Chip
                                  label={`${source.filename} (${(source.similarity * 100).toFixed(1)}%)`}
                                  size="small"
                                  variant="outlined"
                                  sx={{ mr: 0.5, mb: 0.5, fontSize: '0.7rem' }}
                                />
                              </Tooltip>
                            ))}
                          </Box>
                        )}

                        <Typography variant="caption" sx={{ opacity: 0.8, mt: 1, display: 'block' }}>
                          {formatTime(msg.timestamp)}
                        </Typography>
                      </Box>
                    </ListItem>
                    {index < chatHistory.length - 1 && <Divider sx={{ my: 1 }} />}
                  </React.Fragment>
                ))}
                {loading && (
                  <ListItem>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                      <CircularProgress size={20} />
                      <Typography variant="body2" color="textSecondary">
                        AI is thinking...
                      </Typography>
                    </Box>
                  </ListItem>
                )}
                <div ref={messagesEndRef} />
              </List>
            )}
          </Box>

          {/* Message Input */}
          <Box sx={{ p: 2, borderTop: 1, borderColor: 'divider' }}>
            {error && (
              <Alert severity="error" sx={{ mb: 2 }}>
                {error}
              </Alert>
            )}
            
            <Box sx={{ display: 'flex', gap: 1 }}>
              <TextField
                fullWidth
                multiline
                maxRows={4}
                value={message}
                onChange={(e) => setMessage(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder="Type your message here..."
                disabled={loading}
              />
              <Button
                variant="contained"
                endIcon={<Send />}
                onClick={sendMessage}
                disabled={!message.trim() || loading}
                sx={{ alignSelf: 'flex-end' }}
              >
                Send
              </Button>
            </Box>
          </Box>
        </Paper>
            </Container>
          </TabPanel>

          <TabPanel value={activeTab} index={1}>
            <ApiDocumentation />
          </TabPanel>
        </Paper>
      </Container>
    </Box>
  );
};

export default UserDashboard;