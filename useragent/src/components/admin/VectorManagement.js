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
  Delete as DeleteIcon,
  Refresh as RefreshIcon,
  Storage as StorageIcon,
  Description as DocumentIcon,
  VectorIcon,
} from '@mui/icons-material';
import adminService from '../../services/adminService';

const VectorManagement = () => {
  const [vectors, setVectors] = useState([]);
  const [folders, setFolders] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  
  // Dialog states
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  
  // Form states
  const [newVector, setNewVector] = useState({
    name: '',
    folder_name: '',
    description: '',
  });

  useEffect(() => {
    fetchVectors();
    fetchFolders();
  }, []);

  const fetchVectors = async () => {
    try {
      setLoading(true);
      setError('');
      const response = await adminService.getVectors();
      setVectors(response.vectors || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const fetchFolders = async () => {
    try {
      const response = await adminService.getFolders();
      setFolders(response.folders || []);
    } catch (err) {
      console.warn('Failed to fetch folders:', err.message);
    }
  };

  const handleCreateVector = async () => {
    try {
      setError('');
      if (!newVector.name || !newVector.folder_name) {
        setError('Name and folder are required');
        return;
      }

      const response = await adminService.createVector(newVector);
      if (response.success) {
        setSuccess(`Vector '${newVector.name}' created successfully! ${response.message}`);
        setCreateDialogOpen(false);
        setNewVector({ name: '', folder_name: '', description: '' });
        fetchVectors();
      } else {
        setError(response.message || 'Failed to create vector');
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const handleDeleteVector = async (vectorId, vectorName) => {
    if (!window.confirm(`Are you sure you want to delete the vector "${vectorName}"? This action cannot be undone.`)) {
      return;
    }

    try {
      setError('');
      const response = await adminService.deleteVector(vectorId);
      if (response.success) {
        setSuccess(`Vector "${vectorName}" deleted successfully`);
        fetchVectors();
      } else {
        setError(response.message || 'Failed to delete vector');
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const openCreateDialog = () => {
    setCreateDialogOpen(true);
  };

  const getStatusChip = (vector) => {
    if (vector.embedding_count > 0) {
      return <Chip label="Ready" color="success" size="small" />;
    } else if (vector.document_count > 0) {
      return <Chip label="Processing" color="warning" size="small" />;
    } else {
      return <Chip label="Empty" color="default" size="small" />;
    }
  };

  const formatDate = (dateString) => {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleString();
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
          Vector Management
        </Typography>
        <Box>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={fetchVectors}
            sx={{ mr: 2 }}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            startIcon={<AddIcon />}
            onClick={openCreateDialog}
          >
            Create Vector
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Typography variant="body1" color="textSecondary" sx={{ mb: 3 }}>
        Vectors contain embeddings generated from document folders. Create vectors to enable RAG functionality.
      </Typography>

      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell>Folder</TableCell>
              <TableCell>Description</TableCell>
              <TableCell>Documents</TableCell>
              <TableCell>Embeddings</TableCell>
              <TableCell>Status</TableCell>
              <TableCell>Created</TableCell>
              <TableCell>Updated</TableCell>
              <TableCell align="right">Actions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {vectors.length === 0 ? (
              <TableRow>
                <TableCell colSpan={9} align="center">
                  <Typography color="textSecondary">
                    No vectors found. Create a vector from a document folder to get started.
                  </Typography>
                </TableCell>
              </TableRow>
            ) : (
              vectors.map((vector) => (
                <TableRow key={vector.id}>
                  <TableCell>
                    <Box display="flex" alignItems="center">
                      <StorageIcon sx={{ mr: 1, color: 'primary.main' }} />
                      <Typography variant="body2" sx={{ fontWeight: 'bold' }}>
                        {vector.name}
                      </Typography>
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Box display="flex" alignItems="center">
                      <DocumentIcon sx={{ mr: 1, color: 'text.secondary' }} />
                      {vector.folder_name}
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Typography variant="body2" sx={{ maxWidth: 200, overflow: 'hidden', textOverflow: 'ellipsis' }}>
                      {vector.description || 'No description'}
                    </Typography>
                  </TableCell>
                  <TableCell>{vector.document_count || 0}</TableCell>
                  <TableCell>{vector.embedding_count || 0}</TableCell>
                  <TableCell>{getStatusChip(vector)}</TableCell>
                  <TableCell>{formatDate(vector.created_at)}</TableCell>
                  <TableCell>{formatDate(vector.updated_at)}</TableCell>
                  <TableCell align="right">
                    <Tooltip title="Delete Vector">
                      <IconButton 
                        onClick={() => handleDeleteVector(vector.id, vector.name)} 
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

      {/* Create Vector Dialog */}
      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Create New Vector</DialogTitle>
        <DialogContent>
          <Typography variant="body2" color="textSecondary" sx={{ mb: 3 }}>
            Create a vector from a document folder. The system will process all documents in the folder and generate embeddings.
          </Typography>
          
          <TextField
            autoFocus
            margin="dense"
            label="Vector Name"
            fullWidth
            variant="outlined"
            value={newVector.name}
            onChange={(e) => setNewVector({ ...newVector, name: e.target.value })}
            sx={{ mb: 2 }}
            helperText="A unique name for this vector"
          />

          <FormControl fullWidth sx={{ mb: 2 }}>
            <InputLabel>Document Folder</InputLabel>
            <Select
              value={newVector.folder_name}
              label="Document Folder"
              onChange={(e) => setNewVector({ ...newVector, folder_name: e.target.value })}
            >
              {folders.map((folder) => (
                <MenuItem key={folder.name} value={folder.name}>
                  {folder.name} ({folder.document_count} documents)
                </MenuItem>
              ))}
            </Select>
          </FormControl>

          <TextField
            margin="dense"
            label="Description (Optional)"
            fullWidth
            variant="outlined"
            multiline
            rows={3}
            value={newVector.description}
            onChange={(e) => setNewVector({ ...newVector, description: e.target.value })}
            helperText="Optional description for this vector"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleCreateVector}
            variant="contained"
            disabled={!newVector.name || !newVector.folder_name}
          >
            Create Vector
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default VectorManagement;