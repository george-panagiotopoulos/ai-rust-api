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
  Tooltip
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  MoreVert as MoreVertIcon,
  History as HistoryIcon,
  Person as PersonIcon,
  AdminPanelSettings as AdminIcon
} from '@mui/icons-material';
import { format } from 'date-fns';
import adminService from '../../services/adminService';

const UserManagement = () => {
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  
  // Dialog states
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [chatHistoryDialogOpen, setChatHistoryDialogOpen] = useState(false);
  
  // Form states
  const [newAdmin, setNewAdmin] = useState({
    username: '',
    email: '',
    password: ''
  });
  const [editUser, setEditUser] = useState(null);
  const [selectedUser, setSelectedUser] = useState(null);
  const [chatHistory, setChatHistory] = useState([]);
  
  // Menu state
  const [anchorEl, setAnchorEl] = useState(null);
  const [menuUser, setMenuUser] = useState(null);

  useEffect(() => {
    fetchUsers();
  }, []);

  const fetchUsers = async () => {
    try {
      setLoading(true);
      setError('');
      const response = await adminService.getUsers();
      setUsers(response.users || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateAdmin = async () => {
    try {
      setError('');
      await adminService.createAdmin(newAdmin);
      setSuccess('Admin user created successfully!');
      setNewAdmin({ username: '', email: '', password: '' });
      setCreateDialogOpen(false);
      fetchUsers();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleUpdateUser = async () => {
    try {
      setError('');
      const updateData = {
        is_active: editUser.is_active,
        is_admin: editUser.is_admin
      };
      await adminService.updateUser(editUser.id, updateData);
      setSuccess('User updated successfully!');
      setEditDialogOpen(false);
      setEditUser(null);
      fetchUsers();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleDeleteUser = async () => {
    try {
      setError('');
      await adminService.deleteUser(selectedUser.id);
      setSuccess('User deleted successfully!');
      setDeleteDialogOpen(false);
      setSelectedUser(null);
      fetchUsers();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleViewChatHistory = async (user) => {
    try {
      setError('');
      setSelectedUser(user);
      const response = await adminService.getUserChatHistory(user.id);
      setChatHistory(response.chat_history || []);
      setChatHistoryDialogOpen(true);
    } catch (err) {
      setError(err.message);
    }
  };

  const handleMenuClick = (event, user) => {
    setAnchorEl(event.currentTarget);
    setMenuUser(user);
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
    setMenuUser(null);
  };

  const openEditDialog = (user) => {
    setEditUser({
      id: user.id,
      username: user.username,
      email: user.email,
      is_active: user.is_active || false,
      is_admin: user.is_admin || false
    });
    setEditDialogOpen(true);
    handleMenuClose();
  };

  const openDeleteDialog = (user) => {
    setSelectedUser(user);
    setDeleteDialogOpen(true);
    handleMenuClose();
  };

  const openChatHistoryDialog = (user) => {
    handleViewChatHistory(user);
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
          User Management
        </Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => setCreateDialogOpen(true)}
        >
          Create Admin User
        </Button>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Card>
        <CardContent>
          <TableContainer component={Paper}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>User</TableCell>
                  <TableCell>Email</TableCell>
                  <TableCell>Role</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Created</TableCell>
                  <TableCell>Last Login</TableCell>
                  <TableCell align="right">Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {users.map((user) => (
                  <TableRow key={user.id}>
                    <TableCell>
                      <Box display="flex" alignItems="center">
                        {user.is_admin ? (
                          <AdminIcon sx={{ mr: 1, color: 'warning.main' }} />
                        ) : (
                          <PersonIcon sx={{ mr: 1, color: 'grey.500' }} />
                        )}
                        <Typography variant="body2">
                          {user.username}
                        </Typography>
                      </Box>
                    </TableCell>
                    <TableCell>{user.email}</TableCell>
                    <TableCell>
                      <Chip
                        label={user.is_admin ? 'Admin' : 'User'}
                        color={user.is_admin ? 'warning' : 'default'}
                        size="small"
                      />
                    </TableCell>
                    <TableCell>
                      <Chip
                        label={user.is_active ? 'Active' : 'Inactive'}
                        color={user.is_active ? 'success' : 'default'}
                        size="small"
                      />
                    </TableCell>
                    <TableCell>
                      {user.created_at ? format(new Date(user.created_at), 'MMM dd, yyyy') : 'N/A'}
                    </TableCell>
                    <TableCell>
                      {user.last_login ? format(new Date(user.last_login), 'MMM dd, yyyy HH:mm') : 'Never'}
                    </TableCell>
                    <TableCell align="right">
                      <Tooltip title="More actions">
                        <IconButton onClick={(e) => handleMenuClick(e, user)}>
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
        <MenuItem onClick={() => openEditDialog(menuUser)}>
          <EditIcon sx={{ mr: 1 }} />
          Edit User
        </MenuItem>
        <MenuItem onClick={() => openChatHistoryDialog(menuUser)}>
          <HistoryIcon sx={{ mr: 1 }} />
          View Chat History
        </MenuItem>
        <MenuItem onClick={() => openDeleteDialog(menuUser)} sx={{ color: 'error.main' }}>
          <DeleteIcon sx={{ mr: 1 }} />
          Delete User
        </MenuItem>
      </Menu>

      {/* Create Admin Dialog */}
      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Create Admin User</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Username"
            fullWidth
            variant="outlined"
            value={newAdmin.username}
            onChange={(e) => setNewAdmin({ ...newAdmin, username: e.target.value })}
            sx={{ mb: 2 }}
          />
          <TextField
            margin="dense"
            label="Email"
            type="email"
            fullWidth
            variant="outlined"
            value={newAdmin.email}
            onChange={(e) => setNewAdmin({ ...newAdmin, email: e.target.value })}
            sx={{ mb: 2 }}
          />
          <TextField
            margin="dense"
            label="Password"
            type="password"
            fullWidth
            variant="outlined"
            value={newAdmin.password}
            onChange={(e) => setNewAdmin({ ...newAdmin, password: e.target.value })}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleCreateAdmin}
            variant="contained"
            disabled={!newAdmin.username || !newAdmin.email || !newAdmin.password}
          >
            Create Admin
          </Button>
        </DialogActions>
      </Dialog>

      {/* Edit User Dialog */}
      <Dialog open={editDialogOpen} onClose={() => setEditDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Edit User: {editUser?.username}</DialogTitle>
        <DialogContent>
          <Box sx={{ py: 2 }}>
            <FormControlLabel
              control={
                <Switch
                  checked={editUser?.is_active || false}
                  onChange={(e) => setEditUser({ ...editUser, is_active: e.target.checked })}
                />
              }
              label="Active"
              sx={{ mb: 2, display: 'block' }}
            />
            <FormControlLabel
              control={
                <Switch
                  checked={editUser?.is_admin || false}
                  onChange={(e) => setEditUser({ ...editUser, is_admin: e.target.checked })}
                />
              }
              label="Admin"
              sx={{ display: 'block' }}
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEditDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleUpdateUser} variant="contained">
            Update User
          </Button>
        </DialogActions>
      </Dialog>

      {/* Delete User Dialog */}
      <Dialog open={deleteDialogOpen} onClose={() => setDeleteDialogOpen(false)}>
        <DialogTitle>Delete User</DialogTitle>
        <DialogContent>
          <Typography>
            Are you sure you want to delete user "{selectedUser?.username}"? This action cannot be undone.
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDeleteDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleDeleteUser} variant="contained" color="error">
            Delete
          </Button>
        </DialogActions>
      </Dialog>

      {/* Chat History Dialog */}
      <Dialog 
        open={chatHistoryDialogOpen} 
        onClose={() => setChatHistoryDialogOpen(false)} 
        maxWidth="md" 
        fullWidth
        maxHeight="80vh"
      >
        <DialogTitle>Chat History: {selectedUser?.username}</DialogTitle>
        <DialogContent>
          {chatHistory.length === 0 ? (
            <Typography>No chat history found for this user.</Typography>
          ) : (
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Timestamp</TableCell>
                    <TableCell>Conversation ID</TableCell>
                    <TableCell>User Message</TableCell>
                    <TableCell>Assistant Response</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {chatHistory.map((chat) => (
                    <TableRow key={chat.id}>
                      <TableCell>
                        {chat.created_at ? format(new Date(chat.created_at), 'MMM dd, yyyy HH:mm') : 'N/A'}
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2" sx={{ fontFamily: 'monospace' }}>
                          {chat.conversation_id.substring(0, 8)}...
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2" sx={{ maxWidth: 300, overflow: 'hidden', textOverflow: 'ellipsis' }}>
                          {chat.user_message}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2" sx={{ maxWidth: 300, overflow: 'hidden', textOverflow: 'ellipsis' }}>
                          {chat.assistant_response}
                        </Typography>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setChatHistoryDialogOpen(false)}>Close</Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default UserManagement;