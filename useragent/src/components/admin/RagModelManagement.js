import React, { useState, useEffect } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Button,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Chip,
  Alert,
  CircularProgress,
  IconButton,
  Tooltip,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  Refresh as RefreshIcon,
  SmartToy as ModelIcon,
  Storage as VectorIcon,
} from '@mui/icons-material';
import adminService from '../../services/adminService';

const RagModelManagement = () => {
  const [ragModels, setRagModels] = useState([]);
  const [vectors, setVectors] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  
  // Dialog states
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  
  // Form states
  const [newRagModel, setNewRagModel] = useState({
    name: '',
    vector_id: '',
    system_prompt: '',
    context: '',
  });

  const [editRagModel, setEditRagModel] = useState({
    id: null,
    name: '',
    vector_id: '',
    system_prompt: '',
    context: '',
  });

  useEffect(() => {
    fetchRagModels();
    fetchVectors();
  }, []);

  const fetchRagModels = async () => {
    try {
      setLoading(true);
      setError('');
      const response = await adminService.getRagModels();
      setRagModels(response.rag_models || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const fetchVectors = async () => {
    try {
      const response = await adminService.getVectors();
      setVectors(response.vectors || []);
    } catch (err) {
      console.warn('Failed to fetch vectors:', err.message);
    }
  };

  const handleCreateRagModel = async () => {
    try {
      setError('');
      if (!newRagModel.name || !newRagModel.vector_id || !newRagModel.system_prompt) {
        setError('Name, vector, and system prompt are required');
        return;
      }

      const response = await adminService.createRagModel({
        ...newRagModel,
        vector_id: parseInt(newRagModel.vector_id)
      });
      
      if (response.success) {
        setSuccess(`RAG model '${newRagModel.name}' created successfully!`);
        setCreateDialogOpen(false);
        setNewRagModel({ name: '', vector_id: '', system_prompt: '', context: '' });
        fetchRagModels();
      } else {
        setError(response.message || 'Failed to create RAG model');
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const handleUpdateRagModel = async () => {
    try {
      setError('');
      if (!editRagModel.name || !editRagModel.vector_id || !editRagModel.system_prompt) {
        setError('Name, vector, and system prompt are required');
        return;
      }

      const response = await adminService.updateRagModel(editRagModel.id, {
        name: editRagModel.name,
        vector_id: parseInt(editRagModel.vector_id),
        system_prompt: editRagModel.system_prompt,
        context: editRagModel.context,
      });
      
      if (response.success) {
        setSuccess(`RAG model '${editRagModel.name}' updated successfully!`);
        setEditDialogOpen(false);
        setEditRagModel({ id: null, name: '', vector_id: '', system_prompt: '', context: '' });
        fetchRagModels();
      } else {
        setError(response.message || 'Failed to update RAG model');
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const handleDeleteRagModel = async (modelId, modelName) => {
    if (!window.confirm(`Are you sure you want to delete the RAG model "${modelName}"? This action cannot be undone.`)) {
      return;
    }

    try {
      setError('');
      const response = await adminService.deleteRagModel(modelId);
      if (response.success) {
        setSuccess(`RAG model "${modelName}" deleted successfully`);
        fetchRagModels();
      } else {
        setError(response.message || 'Failed to delete RAG model');
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const openCreateDialog = () => {
    setCreateDialogOpen(true);
  };

  const openEditDialog = (ragModel) => {
    setEditRagModel({
      id: ragModel.id,
      name: ragModel.name,
      vector_id: ragModel.vector_id.toString(),
      system_prompt: ragModel.system_prompt,
      context: ragModel.context || '',
    });
    setEditDialogOpen(true);
  };

  const getStatusChip = (ragModel) => {
    return ragModel.is_active ? (
      <Chip label="Active" color="success" size="small" />
    ) : (
      <Chip label="Inactive" color="default" size="small" />
    );
  };

  const formatDate = (dateString) => {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleString();
  };

  const truncateText = (text, maxLength = 50) => {
    if (!text) return 'N/A';
    return text.length > maxLength ? text.substring(0, maxLength) + '...' : text;
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
      </Box>
    );
  }

  return (
    <Box sx={{ p: 3 }}>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4" component="h1">
          RAG Models
        </Typography>
        <Box>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={fetchRagModels}
            sx={{ mr: 2 }}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            startIcon={<AddIcon />}
            onClick={openCreateDialog}
          >
            Create RAG Model
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Typography variant="body1" color="textSecondary" sx={{ mb: 3 }}>
        RAG models combine vectors with system prompts to create intelligent chat assistants with domain-specific knowledge.
      </Typography>

      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell>Vector</TableCell>
              <TableCell>System Prompt</TableCell>
              <TableCell>Context</TableCell>
              <TableCell>Status</TableCell>
              <TableCell>Created</TableCell>
              <TableCell>Updated</TableCell>
              <TableCell align="right">Actions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {ragModels.length === 0 ? (
              <TableRow>
                <TableCell colSpan={8} align="center">
                  <Typography color="textSecondary">
                    No RAG models found. Create a RAG model to enable intelligent chat with your documents.
                  </Typography>
                </TableCell>
              </TableRow>
            ) : (
              ragModels.map((ragModel) => (
                <TableRow key={ragModel.id}>
                  <TableCell>
                    <Box display="flex" alignItems="center">
                      <ModelIcon sx={{ mr: 1, color: 'primary.main' }} />
                      <Typography variant="body2" sx={{ fontWeight: 'bold' }}>
                        {ragModel.name}
                      </Typography>
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Box display="flex" alignItems="center">
                      <VectorIcon sx={{ mr: 1, color: 'text.secondary' }} />
                      {ragModel.vector_name}
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Tooltip title={ragModel.system_prompt}>
                      <Typography variant="body2" sx={{ maxWidth: 200, overflow: 'hidden', textOverflow: 'ellipsis' }}>
                        {truncateText(ragModel.system_prompt, 50)}
                      </Typography>
                    </Tooltip>
                  </TableCell>
                  <TableCell>
                    <Tooltip title={ragModel.context || 'No context'}>
                      <Typography variant="body2" sx={{ maxWidth: 150, overflow: 'hidden', textOverflow: 'ellipsis' }}>
                        {truncateText(ragModel.context, 30)}
                      </Typography>
                    </Tooltip>
                  </TableCell>
                  <TableCell>{getStatusChip(ragModel)}</TableCell>
                  <TableCell>{formatDate(ragModel.created_at)}</TableCell>
                  <TableCell>{formatDate(ragModel.updated_at)}</TableCell>
                  <TableCell align="right">
                    <Tooltip title="Edit RAG Model">
                      <IconButton onClick={() => openEditDialog(ragModel)} size="small">
                        <EditIcon />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title="Delete RAG Model">
                      <IconButton 
                        onClick={() => handleDeleteRagModel(ragModel.id, ragModel.name)} 
                        size="small"
                        color="error"
                      >
                        <DeleteIcon />
                      </IconButton>
                    </Tooltip>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </TableContainer>

      {/* Create RAG Model Dialog */}
      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Create New RAG Model</DialogTitle>
        <DialogContent>
          <Typography variant="body2" color="textSecondary" sx={{ mb: 3 }}>
            Create a RAG model by combining a vector with custom prompts and context.
          </Typography>
          
          <TextField
            autoFocus
            margin="dense"
            label="Model Name"
            fullWidth
            variant="outlined"
            value={newRagModel.name}
            onChange={(e) => setNewRagModel({ ...newRagModel, name: e.target.value })}
            sx={{ mb: 2 }}
            helperText="A unique name for this RAG model"
          />

          <FormControl fullWidth sx={{ mb: 2 }}>
            <InputLabel>Vector</InputLabel>
            <Select
              value={newRagModel.vector_id}
              label="Vector"
              onChange={(e) => setNewRagModel({ ...newRagModel, vector_id: e.target.value })}
            >
              {vectors.map((vector) => (
                <MenuItem key={vector.id} value={vector.id}>
                  {vector.name} ({vector.embedding_count} embeddings)
                </MenuItem>
              ))}
            </Select>
          </FormControl>

          <TextField
            margin="dense"
            label="System Prompt"
            fullWidth
            variant="outlined"
            multiline
            rows={4}
            value={newRagModel.system_prompt}
            onChange={(e) => setNewRagModel({ ...newRagModel, system_prompt: e.target.value })}
            sx={{ mb: 2 }}
            helperText="Instructions for the AI on how to behave and respond"
          />

          <TextField
            margin="dense"
            label="Context (Optional)"
            fullWidth
            variant="outlined"
            multiline
            rows={3}
            value={newRagModel.context}
            onChange={(e) => setNewRagModel({ ...newRagModel, context: e.target.value })}
            helperText="Additional context or instructions for the AI"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleCreateRagModel}
            variant="contained"
            disabled={!newRagModel.name || !newRagModel.vector_id || !newRagModel.system_prompt}
          >
            Create RAG Model
          </Button>
        </DialogActions>
      </Dialog>

      {/* Edit RAG Model Dialog */}
      <Dialog open={editDialogOpen} onClose={() => setEditDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Edit RAG Model</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Model Name"
            fullWidth
            variant="outlined"
            value={editRagModel.name}
            onChange={(e) => setEditRagModel({ ...editRagModel, name: e.target.value })}
            sx={{ mb: 2 }}
          />

          <FormControl fullWidth sx={{ mb: 2 }}>
            <InputLabel>Vector</InputLabel>
            <Select
              value={editRagModel.vector_id}
              label="Vector"
              onChange={(e) => setEditRagModel({ ...editRagModel, vector_id: e.target.value })}
            >
              {vectors.map((vector) => (
                <MenuItem key={vector.id} value={vector.id}>
                  {vector.name} ({vector.embedding_count} embeddings)
                </MenuItem>
              ))}
            </Select>
          </FormControl>

          <TextField
            margin="dense"
            label="System Prompt"
            fullWidth
            variant="outlined"
            multiline
            rows={4}
            value={editRagModel.system_prompt}
            onChange={(e) => setEditRagModel({ ...editRagModel, system_prompt: e.target.value })}
            sx={{ mb: 2 }}
          />

          <TextField
            margin="dense"
            label="Context (Optional)"
            fullWidth
            variant="outlined"
            multiline
            rows={3}
            value={editRagModel.context}
            onChange={(e) => setEditRagModel({ ...editRagModel, context: e.target.value })}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEditDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleUpdateRagModel}
            variant="contained"
            disabled={!editRagModel.name || !editRagModel.vector_id || !editRagModel.system_prompt}
          >
            Update RAG Model
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default RagModelManagement;