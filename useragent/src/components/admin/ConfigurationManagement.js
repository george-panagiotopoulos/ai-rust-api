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
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Switch,
  FormControlLabel,
  Chip,
  Alert,
  CircularProgress,
  Menu,
  MenuItem,
  Tooltip,
  InputAdornment
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  MoreVert as MoreVertIcon,
  Visibility as VisibilityIcon,
  VisibilityOff as VisibilityOffIcon,
  Lock as LockIcon,
  LockOpen as LockOpenIcon,
  Download as DownloadIcon,
  Settings as SettingsIcon
} from '@mui/icons-material';
import { format } from 'date-fns';
import adminService from '../../services/adminService';

const ConfigurationManagement = () => {
  const [configs, setConfigs] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  
  // Dialog states
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [viewDialogOpen, setViewDialogOpen] = useState(false);
  
  // Form states
  const [newConfig, setNewConfig] = useState({
    key: '',
    value: '',
    is_encrypted: false,
    description: ''
  });
  const [editConfig, setEditConfig] = useState(null);
  const [selectedConfig, setSelectedConfig] = useState(null);
  const [viewedConfig, setViewedConfig] = useState(null);
  
  // Menu and visibility states
  const [anchorEl, setAnchorEl] = useState(null);
  const [menuConfig, setMenuConfig] = useState(null);
  const [showPassword, setShowPassword] = useState(false);

  useEffect(() => {
    fetchConfigs();
  }, []);

  const fetchConfigs = async () => {
    try {
      setLoading(true);
      setError('');
      const response = await adminService.getConfigs();
      setConfigs(response.configs || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateConfig = async () => {
    try {
      setError('');
      await adminService.createConfig(newConfig);
      setSuccess('Configuration created successfully!');
      setNewConfig({ key: '', value: '', is_encrypted: false, description: '' });
      setCreateDialogOpen(false);
      fetchConfigs();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleUpdateConfig = async () => {
    try {
      setError('');
      const updateData = {
        value: editConfig.value
      };
      await adminService.updateConfig(editConfig.key, updateData);
      setSuccess('Configuration updated successfully!');
      setEditDialogOpen(false);
      setEditConfig(null);
      fetchConfigs();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleDeleteConfig = async () => {
    try {
      setError('');
      await adminService.deleteConfig(selectedConfig.key);
      setSuccess('Configuration deleted successfully!');
      setDeleteDialogOpen(false);
      setSelectedConfig(null);
      fetchConfigs();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleViewConfig = async (config) => {
    try {
      setError('');
      const response = await adminService.getConfig(config.key);
      setViewedConfig(response);
      setViewDialogOpen(true);
    } catch (err) {
      setError(err.message);
    }
  };

  const handleBackupConfigs = async () => {
    try {
      setError('');
      const response = await adminService.backupConfigs();
      
      // Create and download backup file
      const dataStr = JSON.stringify(response, null, 2);
      const dataBlob = new Blob([dataStr], { type: 'application/json' });
      const url = URL.createObjectURL(dataBlob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `config-backup-${new Date().toISOString().split('T')[0]}.json`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);
      
      setSuccess('Configuration backup downloaded successfully!');
    } catch (err) {
      setError(err.message);
    }
  };

  const handleMenuClick = (event, config) => {
    setAnchorEl(event.currentTarget);
    setMenuConfig(config);
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
    setMenuConfig(null);
  };

  const openEditDialog = (config) => {
    setEditConfig({
      key: config.key,
      value: config.value === '********' ? '' : config.value,
      is_encrypted: config.is_encrypted,
      description: config.description
    });
    setEditDialogOpen(true);
    handleMenuClose();
  };

  const openDeleteDialog = (config) => {
    setSelectedConfig(config);
    setDeleteDialogOpen(true);
    handleMenuClose();
  };

  const openViewDialog = (config) => {
    handleViewConfig(config);
    handleMenuClose();
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
          Configuration Management
        </Typography>
        <Box>
          <Button
            variant="outlined"
            startIcon={<DownloadIcon />}
            onClick={handleBackupConfigs}
            sx={{ mr: 2 }}
          >
            Backup Configs
          </Button>
          <Button
            variant="contained"
            startIcon={<AddIcon />}
            onClick={() => setCreateDialogOpen(true)}
          >
            Add Configuration
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Card>
        <CardContent>
          <TableContainer component={Paper}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Key</TableCell>
                  <TableCell>Value</TableCell>
                  <TableCell>Encryption</TableCell>
                  <TableCell>Description</TableCell>
                  <TableCell>Created</TableCell>
                  <TableCell>Updated</TableCell>
                  <TableCell align="right">Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {configs.map((config) => (
                  <TableRow key={config.key}>
                    <TableCell>
                      <Box display="flex" alignItems="center">
                        <SettingsIcon sx={{ mr: 1, color: 'primary.main' }} />
                        <Typography variant="body2" sx={{ fontFamily: 'monospace' }}>
                          {config.key}
                        </Typography>
                      </Box>
                    </TableCell>
                    <TableCell>
                      <Typography 
                        variant="body2" 
                        sx={{ 
                          fontFamily: 'monospace',
                          maxWidth: 200,
                          overflow: 'hidden',
                          textOverflow: 'ellipsis',
                          whiteSpace: 'nowrap'
                        }}
                      >
                        {config.value}
                      </Typography>
                    </TableCell>
                    <TableCell>
                      <Chip
                        label={config.is_encrypted ? 'Encrypted' : 'Plain Text'}
                        icon={config.is_encrypted ? <LockIcon /> : <LockOpenIcon />}
                        color={config.is_encrypted ? 'warning' : 'default'}
                        size="small"
                      />
                    </TableCell>
                    <TableCell>
                      <Typography 
                        variant="body2" 
                        sx={{ 
                          maxWidth: 200,
                          overflow: 'hidden',
                          textOverflow: 'ellipsis',
                          whiteSpace: 'nowrap'
                        }}
                      >
                        {config.description || 'No description'}
                      </Typography>
                    </TableCell>
                    <TableCell>
                      {config.created_at ? format(new Date(config.created_at), 'MMM dd, yyyy') : 'N/A'}
                    </TableCell>
                    <TableCell>
                      {config.updated_at ? format(new Date(config.updated_at), 'MMM dd, yyyy HH:mm') : 'N/A'}
                    </TableCell>
                    <TableCell align="right">
                      <Tooltip title="More actions">
                        <IconButton onClick={(e) => handleMenuClick(e, config)}>
                          <MoreVertIcon />
                        </IconButton>
                      </Tooltip>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </CardContent>
      </Card>

      {/* Actions Menu */}
      <Menu
        anchorEl={anchorEl}
        open={Boolean(anchorEl)}
        onClose={handleMenuClose}
      >
        <MenuItem onClick={() => openViewDialog(menuConfig)}>
          <VisibilityIcon sx={{ mr: 1 }} />
          View Value
        </MenuItem>
        <MenuItem onClick={() => openEditDialog(menuConfig)}>
          <EditIcon sx={{ mr: 1 }} />
          Edit Configuration
        </MenuItem>
        <MenuItem onClick={() => openDeleteDialog(menuConfig)} sx={{ color: 'error.main' }}>
          <DeleteIcon sx={{ mr: 1 }} />
          Delete Configuration
        </MenuItem>
      </Menu>

      {/* Create Configuration Dialog */}
      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Create Configuration</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Key"
            fullWidth
            variant="outlined"
            value={newConfig.key}
            onChange={(e) => setNewConfig({ ...newConfig, key: e.target.value })}
            sx={{ mb: 2 }}
            helperText="Unique identifier for this configuration"
          />
          <TextField
            margin="dense"
            label="Value"
            fullWidth
            multiline
            rows={4}
            variant="outlined"
            type={newConfig.is_encrypted && !showPassword ? 'password' : 'text'}
            value={newConfig.value}
            onChange={(e) => setNewConfig({ ...newConfig, value: e.target.value })}
            sx={{ mb: 2 }}
            InputProps={{
              endAdornment: newConfig.is_encrypted && (
                <InputAdornment position="end">
                  <IconButton
                    onClick={() => setShowPassword(!showPassword)}
                    edge="end"
                  >
                    {showPassword ? <VisibilityOffIcon /> : <VisibilityIcon />}
                  </IconButton>
                </InputAdornment>
              ),
            }}
          />
          <TextField
            margin="dense"
            label="Description"
            fullWidth
            multiline
            rows={2}
            variant="outlined"
            value={newConfig.description}
            onChange={(e) => setNewConfig({ ...newConfig, description: e.target.value })}
            sx={{ mb: 2 }}
            helperText="Optional description for this configuration"
          />
          <FormControlLabel
            control={
              <Switch
                checked={newConfig.is_encrypted}
                onChange={(e) => setNewConfig({ ...newConfig, is_encrypted: e.target.checked })}
              />
            }
            label="Encrypt this configuration value"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleCreateConfig}
            variant="contained"
            disabled={!newConfig.key || !newConfig.value}
          >
            Create Configuration
          </Button>
        </DialogActions>
      </Dialog>

      {/* Edit Configuration Dialog */}
      <Dialog open={editDialogOpen} onClose={() => setEditDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Edit Configuration: {editConfig?.key}</DialogTitle>
        <DialogContent>
          <TextField
            margin="dense"
            label="Value"
            fullWidth
            multiline
            rows={4}
            variant="outlined"
            type={editConfig?.is_encrypted && !showPassword ? 'password' : 'text'}
            value={editConfig?.value || ''}
            onChange={(e) => setEditConfig({ ...editConfig, value: e.target.value })}
            sx={{ mb: 2 }}
            InputProps={{
              endAdornment: editConfig?.is_encrypted && (
                <InputAdornment position="end">
                  <IconButton
                    onClick={() => setShowPassword(!showPassword)}
                    edge="end"
                  >
                    {showPassword ? <VisibilityOffIcon /> : <VisibilityIcon />}
                  </IconButton>
                </InputAdornment>
              ),
            }}
          />
          {editConfig?.description && (
            <Typography variant="body2" color="textSecondary" sx={{ mb: 1 }}>
              Description: {editConfig.description}
            </Typography>
          )}
          <Typography variant="body2" color="textSecondary">
            Encryption: {editConfig?.is_encrypted ? 'Enabled' : 'Disabled'}
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEditDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleUpdateConfig}
            variant="contained"
            disabled={!editConfig?.value}
          >
            Update Configuration
          </Button>
        </DialogActions>
      </Dialog>

      {/* Delete Configuration Dialog */}
      <Dialog open={deleteDialogOpen} onClose={() => setDeleteDialogOpen(false)}>
        <DialogTitle>Delete Configuration</DialogTitle>
        <DialogContent>
          <Typography>
            Are you sure you want to delete configuration "{selectedConfig?.key}"? This action cannot be undone.
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDeleteDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleDeleteConfig} variant="contained" color="error">
            Delete
          </Button>
        </DialogActions>
      </Dialog>

      {/* View Configuration Dialog */}
      <Dialog open={viewDialogOpen} onClose={() => setViewDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Configuration Details: {viewedConfig?.key}</DialogTitle>
        <DialogContent>
          <Box sx={{ mb: 2 }}>
            <Typography variant="subtitle2" gutterBottom>
              Key:
            </Typography>
            <Typography variant="body2" sx={{ fontFamily: 'monospace', mb: 2 }}>
              {viewedConfig?.key}
            </Typography>

            <Typography variant="subtitle2" gutterBottom>
              Value:
            </Typography>
            <TextField
              fullWidth
              multiline
              rows={6}
              variant="outlined"
              value={viewedConfig?.value || ''}
              InputProps={{
                readOnly: true,
                sx: { fontFamily: 'monospace' }
              }}
              sx={{ mb: 2 }}
            />

            {viewedConfig?.description && (
              <>
                <Typography variant="subtitle2" gutterBottom>
                  Description:
                </Typography>
                <Typography variant="body2" sx={{ mb: 2 }}>
                  {viewedConfig.description}
                </Typography>
              </>
            )}

            <Box display="flex" justifyContent="space-between">
              <Typography variant="body2" color="textSecondary">
                Encrypted: {viewedConfig?.is_encrypted ? 'Yes' : 'No'}
              </Typography>
              <Typography variant="body2" color="textSecondary">
                Last Updated: {viewedConfig?.updated_at ? 
                  format(new Date(viewedConfig.updated_at), 'MMM dd, yyyy HH:mm') : 'N/A'}
              </Typography>
            </Box>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setViewDialogOpen(false)}>Close</Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default ConfigurationManagement;