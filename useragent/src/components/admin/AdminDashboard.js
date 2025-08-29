import React, { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  CircularProgress,
  Alert,
  Chip,
  LinearProgress,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  IconButton,
  Tooltip
} from '@mui/material';
import {
  People as PeopleIcon,
  AdminPanelSettings as AdminIcon,
  Storage as StorageIcon,
  Settings as SettingsIcon,
  CheckCircle as HealthyIcon,
  Error as ErrorIcon,
  Warning as WarningIcon,
  Folder as FolderIcon,
  Description as DocumentIcon,
  Security as SecurityIcon,
  Refresh as RefreshIcon,
  TrendingUp as TrendingUpIcon,
  Login as LoginIcon,
  Memory as MemoryIcon,
  HardDrive as DiskIcon
} from '@mui/icons-material';
import adminService from '../../services/adminService';

const AdminDashboard = () => {
  const [overview, setOverview] = useState(null);
  const [systemHealth, setSystemHealth] = useState(null);
  const [systemStats, setSystemStats] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    fetchDashboardData();
    // Set up auto-refresh every 30 seconds
    const interval = setInterval(fetchDashboardData, 30000);
    return () => clearInterval(interval);
  }, []);

  const fetchDashboardData = async () => {
    try {
      setLoading(true);
      setError('');

      const [overviewData, healthData, statsData] = await Promise.all([
        adminService.getOverview(),
        adminService.getSystemHealth(),
        adminService.getSystemStats()
      ]);

      setOverview(overviewData);
      setSystemHealth(healthData);
      setSystemStats(statsData);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const formatUptime = (seconds) => {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    return `${days}d ${hours}h ${mins}m`;
  };

  const formatBytes = (bytes) => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const getServiceStatusColor = (status) => {
    switch (status) {
      case 'healthy': return 'success';
      case 'degraded': return 'warning';
      case 'unhealthy': return 'error';
      default: return 'default';
    }
  };

  const getServiceStatusIcon = (status) => {
    switch (status) {
      case 'healthy': return <HealthyIcon />;
      case 'degraded': return <WarningIcon />;
      case 'unhealthy': return <ErrorIcon />;
      default: return <ErrorIcon />;
    }
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Alert severity="error" sx={{ mt: 2 }}>
        {error}
      </Alert>
    );
  }

  return (
    <Box sx={{ flexGrow: 1, p: 3 }}>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4" component="h1">
          Admin Dashboard
        </Typography>
        <Tooltip title="Refresh Dashboard">
          <IconButton onClick={fetchDashboardData} disabled={loading}>
            <RefreshIcon />
          </IconButton>
        </Tooltip>
      </Box>

      {/* Overview Cards - Enhanced with better styling and additional metrics */}
      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid item xs={12} sm={6} md={3}>
          <Card sx={{ 
            height: '140px', 
            background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
            color: 'white',
            position: 'relative',
            overflow: 'hidden'
          }}>
            <CardContent sx={{ position: 'relative', zIndex: 2 }}>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="body2" sx={{ opacity: 0.9, mb: 1 }}>
                    Total Users
                  </Typography>
                  <Typography variant="h3" component="div" sx={{ fontWeight: 'bold' }}>
                    {overview?.total_users || 0}
                  </Typography>
                  <Typography variant="caption" sx={{ opacity: 0.8 }}>
                    Active: {overview?.active_users || 0}
                  </Typography>
                </Box>
                <PeopleIcon sx={{ fontSize: 50, opacity: 0.8 }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card sx={{ 
            height: '140px', 
            background: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)',
            color: 'white',
            position: 'relative',
            overflow: 'hidden'
          }}>
            <CardContent sx={{ position: 'relative', zIndex: 2 }}>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="body2" sx={{ opacity: 0.9, mb: 1 }}>
                    Admin Users
                  </Typography>
                  <Typography variant="h3" component="div" sx={{ fontWeight: 'bold' }}>
                    {overview?.admin_users || 0}
                  </Typography>
                  <Typography variant="caption" sx={{ opacity: 0.8 }}>
                    Recent Logins: {systemStats?.users?.recent_logins || 0}
                  </Typography>
                </Box>
                <AdminIcon sx={{ fontSize: 50, opacity: 0.8 }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card sx={{ 
            height: '140px', 
            background: 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
            color: 'white',
            position: 'relative',
            overflow: 'hidden'
          }}>
            <CardContent sx={{ position: 'relative', zIndex: 2 }}>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="body2" sx={{ opacity: 0.9, mb: 1 }}>
                    Documents
                  </Typography>
                  <Typography variant="h3" component="div" sx={{ fontWeight: 'bold' }}>
                    {systemStats?.documents?.total_files || 0}
                  </Typography>
                  <Typography variant="caption" sx={{ opacity: 0.8 }}>
                    Size: {systemStats?.documents ? formatBytes(systemStats.documents.total_size) : '0 B'}
                  </Typography>
                </Box>
                <DocumentIcon sx={{ fontSize: 50, opacity: 0.8 }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card sx={{ 
            height: '140px', 
            background: 'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)',
            color: 'white',
            position: 'relative',
            overflow: 'hidden'
          }}>
            <CardContent sx={{ position: 'relative', zIndex: 2 }}>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="body2" sx={{ opacity: 0.9, mb: 1 }}>
                    Configurations
                  </Typography>
                  <Typography variant="h3" component="div" sx={{ fontWeight: 'bold' }}>
                    {overview?.total_configs || 0}
                  </Typography>
                  <Typography variant="caption" sx={{ opacity: 0.8 }}>
                    Encrypted: {systemStats?.configurations?.encrypted || 0}
                  </Typography>
                </Box>
                <SettingsIcon sx={{ fontSize: 50, opacity: 0.8 }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* System Health - Enhanced with more metrics and better visualization */}
      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid item xs={12} md={8}>
          <Card sx={{ height: '100%' }}>
            <CardContent>
              <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                <Typography variant="h6">
                  System Health & Services
                </Typography>
                <Chip 
                  label={systemHealth?.status || 'Unknown'}
                  color={getServiceStatusColor(systemHealth?.status)}
                  icon={getServiceStatusIcon(systemHealth?.status)}
                  size="large"
                />
              </Box>
              <Grid container spacing={2}>
                <Grid item xs={12} sm={6}>
                  <Box sx={{ p: 2, bgcolor: 'grey.50', borderRadius: 1 }}>
                    <Typography variant="body2" color="textSecondary" gutterBottom>
                      System Uptime
                    </Typography>
                    <Typography variant="h6">
                      {systemHealth?.uptime ? formatUptime(systemHealth.uptime) : 'Unknown'}
                    </Typography>
                  </Box>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Box sx={{ p: 2, bgcolor: 'grey.50', borderRadius: 1 }}>
                    <Typography variant="body2" color="textSecondary" gutterBottom>
                      Database Connection
                    </Typography>
                    <Box display="flex" alignItems="center">
                      {systemHealth?.database_connection ? (
                        <><HealthyIcon sx={{ color: 'success.main', mr: 1 }} /> Connected</>
                      ) : (
                        <><ErrorIcon sx={{ color: 'error.main', mr: 1 }} /> Disconnected</>
                      )}
                    </Box>
                  </Box>
                </Grid>
              </Grid>
              <Typography variant="subtitle1" sx={{ mt: 2, mb: 1 }}>
                Service Status
              </Typography>
              <TableContainer>
                <Table size="small">
                  <TableHead>
                    <TableRow>
                      <TableCell>Service</TableCell>
                      <TableCell>Status</TableCell>
                      <TableCell>Health Check</TableCell>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {systemHealth?.external_services && Object.entries(systemHealth.external_services).map(([service, status]) => (
                      <TableRow key={service}>
                        <TableCell>
                          <Box display="flex" alignItems="center">
                            {service === 'auth_api' && <SecurityIcon sx={{ mr: 1, color: 'warning.main' }} />}
                            {service === 'bedrock_api' && <SettingsIcon sx={{ mr: 1, color: 'info.main' }} />}
                            {service === 'rag_api' && <StorageIcon sx={{ mr: 1, color: 'success.main' }} />}
                            {service === 'database' && <StorageIcon sx={{ mr: 1, color: 'primary.main' }} />}
                            {service.replace('_', ' ').replace(/\b\w/g, l => l.toUpperCase())}
                          </Box>
                        </TableCell>
                        <TableCell>
                          <Chip 
                            label={status}
                            color={getServiceStatusColor(status)}
                            size="small"
                            icon={getServiceStatusIcon(status)}
                          />
                        </TableCell>
                        <TableCell>
                          <Typography variant="caption" color="textSecondary">
                            {status === 'healthy' ? 'Responding' : status === 'unhealthy' ? 'Error' : 'Unreachable'}
                          </Typography>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </TableContainer>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} md={4}>
          <Card sx={{ height: '100%' }}>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Quick Stats
              </Typography>
              <Box sx={{ mb: 2 }}>
                <Typography variant="body2" color="textSecondary" gutterBottom>
                  Document Folders
                </Typography>
                <Box display="flex" alignItems="center">
                  <FolderIcon sx={{ mr: 1, color: 'primary.main' }} />
                  <Typography variant="h6">
                    {systemStats?.documents?.total_folders || 0}
                  </Typography>
                </Box>
              </Box>
              <Box sx={{ mb: 2 }}>
                <Typography variant="body2" color="textSecondary" gutterBottom>
                  Recent Activity
                </Typography>
                <Box display="flex" alignItems="center">
                  <TrendingUpIcon sx={{ mr: 1, color: 'info.main' }} />
                  <Typography variant="h6">
                    {systemStats?.recent_activity?.length || 0} events
                  </Typography>
                </Box>
              </Box>
              <Box sx={{ mb: 2 }}>
                <Typography variant="body2" color="textSecondary" gutterBottom>
                  Login Activity (24h)
                </Typography>
                <Box display="flex" alignItems="center">
                  <LoginIcon sx={{ mr: 1, color: 'success.main' }} />
                  <Typography variant="h6">
                    {systemStats?.users?.recent_logins || 0}
                  </Typography>
                </Box>
              </Box>
              <Box>
                <Typography variant="body2" color="textSecondary" gutterBottom>
                  Configuration Security
                </Typography>
                <Box display="flex" alignItems="center" mb={1}>
                  <SecurityIcon sx={{ mr: 1, color: 'warning.main' }} />
                  <Typography variant="body2">
                    {systemStats?.configurations?.encrypted || 0} / {systemStats?.configurations?.total || 0} encrypted
                  </Typography>
                </Box>
                {systemStats?.configurations && systemStats.configurations.total > 0 && (
                  <LinearProgress 
                    variant="determinate" 
                    value={(systemStats.configurations.encrypted / systemStats.configurations.total) * 100}
                    sx={{ mt: 1, height: 8, borderRadius: 1 }}
                  />
                )}
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Detailed Statistics Grid */}
      <Grid container spacing={3}>
        <Grid item xs={12} md={4}>
          <Card sx={{ height: '100%' }}>
            <CardContent>
              <Box display="flex" alignItems="center" mb={2}>
                <PeopleIcon sx={{ mr: 1, color: 'primary.main' }} />
                <Typography variant="h6">
                  User Analytics
                </Typography>
              </Box>
              {systemStats?.users && (
                <Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                    <Typography variant="body2" color="textSecondary">
                      Total Users
                    </Typography>
                    <Typography variant="h6" color="primary">
                      {systemStats.users.total}
                    </Typography>
                  </Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                    <Typography variant="body2" color="textSecondary">
                      Active Users
                    </Typography>
                    <Typography variant="h6" color="success.main">
                      {systemStats.users.active}
                    </Typography>
                  </Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                    <Typography variant="body2" color="textSecondary">
                      Administrators
                    </Typography>
                    <Typography variant="h6" color="warning.main">
                      {systemStats.users.admins}
                    </Typography>
                  </Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center">
                    <Typography variant="body2" color="textSecondary">
                      Recent Logins (24h)
                    </Typography>
                    <Typography variant="h6" color="info.main">
                      {systemStats.users.recent_logins}
                    </Typography>
                  </Box>
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card sx={{ height: '100%' }}>
            <CardContent>
              <Box display="flex" alignItems="center" mb={2}>
                <DocumentIcon sx={{ mr: 1, color: 'info.main' }} />
                <Typography variant="h6">
                  Document Management
                </Typography>
              </Box>
              {systemStats?.documents && (
                <Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                    <Typography variant="body2" color="textSecondary">
                      Total Files
                    </Typography>
                    <Typography variant="h6" color="primary">
                      {systemStats.documents.total_files}
                    </Typography>
                  </Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                    <Typography variant="body2" color="textSecondary">
                      Total Folders
                    </Typography>
                    <Typography variant="h6" color="success.main">
                      {systemStats.documents.total_folders}
                    </Typography>
                  </Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center">
                    <Typography variant="body2" color="textSecondary">
                      Storage Used
                    </Typography>
                    <Typography variant="h6" color="warning.main">
                      {formatBytes(systemStats.documents.total_size)}
                    </Typography>
                  </Box>
                  <Box sx={{ mt: 2, p: 2, bgcolor: 'grey.50', borderRadius: 1 }}>
                    <Typography variant="caption" color="textSecondary" gutterBottom>
                      Average file size: {systemStats.documents.total_files > 0 ? 
                        formatBytes(systemStats.documents.total_size / systemStats.documents.total_files) : '0 B'}
                    </Typography>
                  </Box>
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card sx={{ height: '100%' }}>
            <CardContent>
              <Box display="flex" alignItems="center" mb={2}>
                <SecurityIcon sx={{ mr: 1, color: 'warning.main' }} />
                <Typography variant="h6">
                  Security Overview
                </Typography>
              </Box>
              {systemStats?.configurations && (
                <Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                    <Typography variant="body2" color="textSecondary">
                      Total Configurations
                    </Typography>
                    <Typography variant="h6" color="primary">
                      {systemStats.configurations.total}
                    </Typography>
                  </Box>
                  <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                    <Typography variant="body2" color="textSecondary">
                      Encrypted Settings
                    </Typography>
                    <Typography variant="h6" color="success.main">
                      {systemStats.configurations.encrypted}
                    </Typography>
                  </Box>
                  <Box sx={{ mb: 2 }}>
                    <Typography variant="body2" color="textSecondary" gutterBottom>
                      Encryption Rate: {systemStats.configurations.total > 0 ? 
                        ((systemStats.configurations.encrypted / systemStats.configurations.total) * 100).toFixed(1) : 0}%
                    </Typography>
                    <LinearProgress 
                      variant="determinate" 
                      value={systemStats.configurations.total > 0 ? 
                        (systemStats.configurations.encrypted / systemStats.configurations.total) * 100 : 0}
                      sx={{ height: 8, borderRadius: 1 }}
                    />
                  </Box>
                  <Box sx={{ p: 2, bgcolor: systemStats.configurations.total > 0 && 
                    systemStats.configurations.encrypted / systemStats.configurations.total < 0.7 ? 
                    'warning.light' : 'success.light', 
                    borderRadius: 1, opacity: 0.7 }}>
                    <Typography variant="caption" color="textPrimary">
                      {systemStats.configurations.total > 0 && 
                       systemStats.configurations.encrypted / systemStats.configurations.total < 0.7 ? 
                       '⚠️ Consider encrypting more configurations' : 
                       '✅ Good security coverage'}
                    </Typography>
                  </Box>
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Recent Activity - Enhanced with better styling */}
      {systemStats?.recent_activity && systemStats.recent_activity.length > 0 && (
        <Card sx={{ mt: 3 }}>
          <CardContent>
            <Box display="flex" alignItems="center" mb={2}>
              <TrendingUpIcon sx={{ mr: 1, color: 'info.main' }} />
              <Typography variant="h6">
                Recent System Activity
              </Typography>
              <Chip 
                label={`${systemStats.recent_activity.length} events`} 
                size="small" 
                sx={{ ml: 2 }} 
                color="info" 
              />
            </Box>
            <TableContainer component={Paper} elevation={0} sx={{ border: 1, borderColor: 'divider' }}>
              <Table>
                <TableHead sx={{ bgcolor: 'grey.50' }}>
                  <TableRow>
                    <TableCell sx={{ fontWeight: 'bold' }}>Timestamp</TableCell>
                    <TableCell sx={{ fontWeight: 'bold' }}>User</TableCell>
                    <TableCell sx={{ fontWeight: 'bold' }}>Action</TableCell>
                    <TableCell sx={{ fontWeight: 'bold' }}>Details</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {systemStats.recent_activity.map((activity, index) => (
                    <TableRow key={index} sx={{ '&:hover': { bgcolor: 'grey.50' } }}>
                      <TableCell>
                        <Typography variant="body2" sx={{ fontFamily: 'monospace' }}>
                          {new Date(activity.timestamp).toLocaleString()}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Box display="flex" alignItems="center">
                          <PeopleIcon sx={{ mr: 1, fontSize: 16, color: 'primary.main' }} />
                          <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
                            {activity.user}
                          </Typography>
                        </Box>
                      </TableCell>
                      <TableCell>
                        <Chip 
                          label={activity.action} 
                          size="small" 
                          variant="outlined"
                          color={activity.action.toLowerCase().includes('delete') ? 'error' : 
                                 activity.action.toLowerCase().includes('create') ? 'success' : 'default'}
                        />
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2" color="textSecondary">
                          {activity.details}
                        </Typography>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </CardContent>
        </Card>
      )}
    </Box>
  );
};

export default AdminDashboard;