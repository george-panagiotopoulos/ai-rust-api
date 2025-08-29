import React, { useState, useEffect, useCallback } from 'react';
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
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Chip,
  Alert,
  CircularProgress,
  Menu,
  MenuItem,
  Tooltip,
  Grid,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  ListItemSecondaryAction,
  Breadcrumbs,
  Link,
  LinearProgress
} from '@mui/material';
import {
  CreateNewFolder as CreateFolderIcon,
  Upload as UploadIcon,
  Delete as DeleteIcon,
  MoreVert as MoreVertIcon,
  Folder as FolderIcon,
  InsertDriveFile as FileIcon,
  Description as DocIcon,
  PictureAsPdf as PdfIcon,
  Archive as ArchiveIcon,
  ArrowBack as BackIcon,
  Timeline as CreateVectorIcon
} from '@mui/icons-material';
import { format } from 'date-fns';
import adminService from '../../services/adminService';

const DocumentManagement = () => {
  const [folders, setFolders] = useState([]);
  const [documents, setDocuments] = useState([]);
  const [selectedFolder, setSelectedFolder] = useState(null);
  const [loading, setLoading] = useState(true);
  const [uploadLoading, setUploadLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  
  // Dialog states
  const [createFolderDialogOpen, setCreateFolderDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [uploadDialogOpen, setUploadDialogOpen] = useState(false);
  const [createVectorDialogOpen, setCreateVectorDialogOpen] = useState(false);
  
  // Form states
  const [newFolder, setNewFolder] = useState({ name: '' });
  const [selectedItem, setSelectedItem] = useState(null);
  const [selectedFiles, setSelectedFiles] = useState([]);
  const [newVector, setNewVector] = useState({
    name: '',
    folder_name: '',
    description: '',
  });
  
  // Menu state
  const [anchorEl, setAnchorEl] = useState(null);
  const [menuItem, setMenuItem] = useState(null);

  useEffect(() => {
    if (selectedFolder) {
      fetchDocuments(selectedFolder.name);
    } else {
      fetchFolders();
    }
  }, [selectedFolder]);

  const fetchFolders = async () => {
    try {
      setLoading(true);
      setError('');
      const response = await adminService.getFolders();
      setFolders(response.folders || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const fetchDocuments = async (folderName) => {
    try {
      setLoading(true);
      setError('');
      const response = await adminService.getDocuments(folderName);
      setDocuments(response.documents || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateFolder = async () => {
    try {
      setError('');
      await adminService.createFolder(newFolder);
      setSuccess('Folder created successfully!');
      setNewFolder({ name: '' });
      setCreateFolderDialogOpen(false);
      fetchFolders();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleDeleteItem = async () => {
    try {
      setError('');
      if (selectedItem.type === 'folder') {
        await adminService.deleteFolder(selectedItem.name);
        setSuccess('Folder deleted successfully!');
        fetchFolders();
      } else {
        await adminService.deleteDocument(selectedFolder.name, selectedItem.name);
        setSuccess('Document deleted successfully!');
        fetchDocuments(selectedFolder.name);
      }
      setDeleteDialogOpen(false);
      setSelectedItem(null);
    } catch (err) {
      setError(err.message);
    }
  };

  const handleFileUpload = async () => {
    if (!selectedFolder || selectedFiles.length === 0) return;

    try {
      setUploadLoading(true);
      setError('');
      
      for (const file of selectedFiles) {
        await adminService.uploadDocument(selectedFolder.name, file);
      }
      
      setSuccess(`${selectedFiles.length} file(s) uploaded successfully!`);
      setSelectedFiles([]);
      setUploadDialogOpen(false);
      fetchDocuments(selectedFolder.name);
    } catch (err) {
      setError(err.message);
    } finally {
      setUploadLoading(false);
    }
  };

  const handleCreateVector = async () => {
    try {
      setError('');
      if (!newVector.name) {
        setError('Vector name is required');
        return;
      }

      const vectorData = {
        ...newVector,
        folder_name: selectedFolder.name,
      };

      const response = await adminService.createVector(vectorData);
      if (response.success) {
        setSuccess(`Vector '${newVector.name}' created successfully! ${response.message}`);
        setCreateVectorDialogOpen(false);
        setNewVector({ name: '', folder_name: '', description: '' });
      } else {
        setError(response.message || 'Failed to create vector');
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const openCreateVectorDialog = () => {
    setNewVector({
      name: selectedFolder.name + '_vector',
      folder_name: selectedFolder.name,
      description: `Vector created from ${selectedFolder.name} folder`,
    });
    setCreateVectorDialogOpen(true);
  };

  const handleFileSelect = (event) => {
    const files = Array.from(event.target.files);
    setSelectedFiles(prev => [...prev, ...files]);
  };

  const handleMenuClick = (event, item, type) => {
    setAnchorEl(event.currentTarget);
    setMenuItem({ ...item, type });
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
    setMenuItem(null);
  };

  const openDeleteDialog = (item, type) => {
    setSelectedItem({ ...item, type });
    setDeleteDialogOpen(true);
    handleMenuClose();
  };

  const getFileIcon = (filename) => {
    const extension = filename.split('.').pop()?.toLowerCase();
    switch (extension) {
      case 'pdf':
        return <PdfIcon sx={{ color: 'error.main' }} />;
      case 'doc':
      case 'docx':
        return <DocIcon sx={{ color: 'primary.main' }} />;
      case 'txt':
      case 'md':
        return <FileIcon sx={{ color: 'text.secondary' }} />;
      case 'json':
      case 'xml':
      case 'csv':
        return <ArchiveIcon sx={{ color: 'warning.main' }} />;
      default:
        return <FileIcon sx={{ color: 'text.secondary' }} />;
    }
  };

  const formatBytes = (bytes) => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const removeSelectedFile = (index) => {
    setSelectedFiles(prev => prev.filter((_, i) => i !== index));
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
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" component="h1" gutterBottom>
            Document Management
          </Typography>
          <Breadcrumbs aria-label="breadcrumb">
            <Link
              component="button"
              variant="body1"
              onClick={() => setSelectedFolder(null)}
              sx={{ textDecoration: selectedFolder ? 'underline' : 'none' }}
            >
              Folders
            </Link>
            {selectedFolder && (
              <Typography color="textPrimary">{selectedFolder.name}</Typography>
            )}
          </Breadcrumbs>
        </Box>
        <Box>
          {selectedFolder && (
            <>
              <Button
                variant="outlined"
                startIcon={<BackIcon />}
                onClick={() => setSelectedFolder(null)}
                sx={{ mr: 2 }}
              >
                Back to Folders
              </Button>
              <Button
                variant="outlined"
                startIcon={<UploadIcon />}
                onClick={() => setUploadDialogOpen(true)}
                sx={{ mr: 2 }}
              >
                Upload Documents
              </Button>
              <Button
                variant="contained"
                startIcon={<CreateVectorIcon />}
                onClick={openCreateVectorDialog}
                color="primary"
              >
                Create Vector
              </Button>
            </>
          )}
          {!selectedFolder && (
            <Button
              variant="contained"
              startIcon={<CreateFolderIcon />}
              onClick={() => setCreateFolderDialogOpen(true)}
            >
              Create Folder
            </Button>
          )}
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      {/* Content */}
      <Card>
        <CardContent>
          {!selectedFolder ? (
            // Folders view
            <Grid container spacing={3}>
              {folders.map((folder) => (
                <Grid item xs={12} sm={6} md={4} key={folder.name}>
                  <Card 
                    sx={{ 
                      cursor: 'pointer',
                      '&:hover': { elevation: 4 },
                      position: 'relative'
                    }}
                    onClick={() => setSelectedFolder(folder)}
                  >
                    <CardContent>
                      <Box display="flex" alignItems="center" justifyContent="space-between">
                        <Box display="flex" alignItems="center">
                          <FolderIcon sx={{ fontSize: 40, color: 'primary.main', mr: 2 }} />
                          <Box>
                            <Typography variant="h6" gutterBottom>
                              {folder.name}
                            </Typography>
                            <Typography variant="body2" color="textSecondary">
                              {folder.document_count} documents
                            </Typography>
                            <Typography variant="caption" color="textSecondary">
                              Created: {folder.created_at ? 
                                format(new Date(folder.created_at), 'MMM dd, yyyy') : 'N/A'}
                            </Typography>
                          </Box>
                        </Box>
                        <IconButton
                          onClick={(e) => {
                            e.stopPropagation();
                            handleMenuClick(e, folder, 'folder');
                          }}
                        >
                          <MoreVertIcon />
                        </IconButton>
                      </Box>
                    </CardContent>
                  </Card>
                </Grid>
              ))}
            </Grid>
          ) : (
            // Documents view
            <TableContainer component={Paper}>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Document</TableCell>
                    <TableCell>Size</TableCell>
                    <TableCell>Type</TableCell>
                    <TableCell>Created</TableCell>
                    <TableCell align="right">Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {documents.map((doc) => (
                    <TableRow key={doc.name}>
                      <TableCell>
                        <Box display="flex" alignItems="center">
                          {getFileIcon(doc.name)}
                          <Typography variant="body2" sx={{ ml: 1 }}>
                            {doc.name}
                          </Typography>
                        </Box>
                      </TableCell>
                      <TableCell>{formatBytes(doc.size)}</TableCell>
                      <TableCell>
                        <Chip
                          label={doc.content_type || 'Unknown'}
                          size="small"
                          variant="outlined"
                        />
                      </TableCell>
                      <TableCell>
                        {doc.created_at ? format(new Date(doc.created_at), 'MMM dd, yyyy HH:mm') : 'N/A'}
                      </TableCell>
                      <TableCell align="right">
                        <Tooltip title="More actions">
                          <IconButton onClick={(e) => handleMenuClick(e, doc, 'document')}>
                            <MoreVertIcon />
                          </IconButton>
                        </Tooltip>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          )}
        </CardContent>
      </Card>

      {/* Actions Menu */}
      <Menu
        anchorEl={anchorEl}
        open={Boolean(anchorEl)}
        onClose={handleMenuClose}
      >
        <MenuItem 
          onClick={() => openDeleteDialog(menuItem, menuItem?.type)} 
          sx={{ color: 'error.main' }}
        >
          <DeleteIcon sx={{ mr: 1 }} />
          Delete {menuItem?.type === 'folder' ? 'Folder' : 'Document'}
        </MenuItem>
      </Menu>

      {/* Create Folder Dialog */}
      <Dialog open={createFolderDialogOpen} onClose={() => setCreateFolderDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Create New Folder</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Folder Name"
            fullWidth
            variant="outlined"
            value={newFolder.name}
            onChange={(e) => setNewFolder({ ...newFolder, name: e.target.value })}
            helperText="Enter a name for the new folder"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateFolderDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleCreateFolder}
            variant="contained"
            disabled={!newFolder.name.trim()}
          >
            Create Folder
          </Button>
        </DialogActions>
      </Dialog>

      {/* Upload Documents Dialog */}
      <Dialog open={uploadDialogOpen} onClose={() => setUploadDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Upload Documents to {selectedFolder?.name}</DialogTitle>
        <DialogContent>
          <Box
            sx={{
              border: '2px dashed',
              borderColor: 'grey.300',
              borderRadius: 2,
              p: 4,
              textAlign: 'center',
              mb: 2,
              position: 'relative'
            }}
          >
            <input
              type="file"
              multiple
              accept=".pdf,.txt,.md,.docx,.json,.csv,.xml,.html,.rtf"
              onChange={handleFileSelect}
              style={{
                position: 'absolute',
                top: 0,
                left: 0,
                width: '100%',
                height: '100%',
                opacity: 0,
                cursor: 'pointer'
              }}
            />
            <UploadIcon sx={{ fontSize: 48, color: 'text.secondary', mb: 1 }} />
            <Typography variant="h6" gutterBottom>
              Click to select files
            </Typography>
            <Typography variant="caption" color="textSecondary" sx={{ mt: 1, display: 'block' }}>
              Supported formats: PDF, TXT, MD, DOCX, JSON, CSV, XML, HTML, RTF
            </Typography>
          </Box>

          {selectedFiles.length > 0 && (
            <Box>
              <Typography variant="subtitle1" gutterBottom>
                Selected Files ({selectedFiles.length}):
              </Typography>
              <List dense>
                {selectedFiles.map((file, index) => (
                  <ListItem key={index}>
                    <ListItemIcon>
                      {getFileIcon(file.name)}
                    </ListItemIcon>
                    <ListItemText
                      primary={file.name}
                      secondary={formatBytes(file.size)}
                    />
                    <ListItemSecondaryAction>
                      <IconButton onClick={() => removeSelectedFile(index)}>
                        <DeleteIcon />
                      </IconButton>
                    </ListItemSecondaryAction>
                  </ListItem>
                ))}
              </List>
            </Box>
          )}

          {uploadLoading && (
            <Box sx={{ mt: 2 }}>
              <Typography variant="body2" gutterBottom>
                Uploading files...
              </Typography>
              <LinearProgress />
            </Box>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setUploadDialogOpen(false)} disabled={uploadLoading}>
            Cancel
          </Button>
          <Button 
            onClick={handleFileUpload}
            variant="contained"
            disabled={selectedFiles.length === 0 || uploadLoading}
          >
            Upload {selectedFiles.length} File{selectedFiles.length !== 1 ? 's' : ''}
          </Button>
        </DialogActions>
      </Dialog>

      {/* Delete Confirmation Dialog */}
      <Dialog open={deleteDialogOpen} onClose={() => setDeleteDialogOpen(false)}>
        <DialogTitle>
          Delete {selectedItem?.type === 'folder' ? 'Folder' : 'Document'}
        </DialogTitle>
        <DialogContent>
          <Typography>
            Are you sure you want to delete {selectedItem?.type === 'folder' ? 'folder' : 'document'} "{selectedItem?.name}"? 
            {selectedItem?.type === 'folder' && ' This will also delete all documents in the folder.'} 
            This action cannot be undone.
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDeleteDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleDeleteItem} variant="contained" color="error">
            Delete
          </Button>
        </DialogActions>
      </Dialog>

      {/* Create Vector Dialog */}
      <Dialog open={createVectorDialogOpen} onClose={() => setCreateVectorDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Create Vector from Folder</DialogTitle>
        <DialogContent>
          <Typography variant="body2" color="textSecondary" sx={{ mb: 3 }}>
            Create a vector by processing all documents in the "{selectedFolder?.name}" folder. 
            The system will generate embeddings that can be used for RAG functionality.
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

          <Typography variant="body2" color="textSecondary" sx={{ mt: 2 }}>
            <strong>Folder:</strong> {selectedFolder?.name}<br/>
            <strong>Documents:</strong> {selectedFolder?.document_count || 0} files<br/>
            Vector processing will begin immediately after creation.
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateVectorDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleCreateVector}
            variant="contained"
            disabled={!newVector.name}
          >
            Create Vector
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default DocumentManagement;