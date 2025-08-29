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
  Paper
} from '@mui/material';
import {
  People as PeopleIcon,
  AdminPanelSettings as AdminIcon,
  Storage as StorageIcon,
  Settings as SettingsIcon,
  CheckCircle as HealthyIcon,
  Error as ErrorIcon,
  Warning as WarningIcon
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
      <Typography variant="h4" component="h1" gutterBottom>
        Admin Dashboard
      </Typography>

      {/* Overview Cards */}
      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid item xs={12} sm={6} md={3}>
          <Card sx={{ height: '120px', display: 'flex', alignItems: 'center' }}>
            <CardContent sx={{ width: '100%' }}>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography color="textSecondary" gutterBottom variant="body2">
                    Total Users
                  </Typography>
                  <Typography variant="h4" component="div">
                    {overview?.total_users || 0}
                  </Typography>
                </Box>
                <PeopleIcon sx={{ fontSize: 40, color: 'primary.main' }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card sx={{ height: '120px', display: 'flex', alignItems: 'center' }}>
            <CardContent sx={{ width: '100%' }}>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography color="textSecondary" gutterBottom variant="body2">
                    Active Users
                  </Typography>
                  <Typography variant="h4" component="div">
                    {overview?.active_users || 0}
                  </Typography>
                </Box>
                <PeopleIcon sx={{ fontSize: 40, color: 'success.main' }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card sx={{ height: '120px', display: 'flex', alignItems: 'center' }}>
            <CardContent sx={{ width: '100%' }}>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography color="textSecondary" gutterBottom variant="body2">
                    Admin Users
                  </Typography>
                  <Typography variant="h4" component="div">
                    {overview?.admin_users || 0}
                  </Typography>
                </Box>
                <AdminIcon sx={{ fontSize: 40, color: 'warning.main' }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card sx={{ height: '120px', display: 'flex', alignItems: 'center' }}>
            <CardContent sx={{ width: '100%' }}>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography color="textSecondary" gutterBottom variant="body2">
                    Configurations
                  </Typography>
                  <Typography variant="h4" component="div">
                    {overview?.total_configs || 0}
                  </Typography>
                </Box>
                <SettingsIcon sx={{ fontSize: 40, color: 'info.main' }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* System Health */}
      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                System Health
              </Typography>
              <Box sx={{ mb: 2 }}>
                <Typography variant="subtitle1" gutterBottom>
                  Overall Status:
                  <Chip 
                    label={systemHealth?.status || 'Unknown'}
                    color={getServiceStatusColor(systemHealth?.status)}
                    icon={getServiceStatusIcon(systemHealth?.status)}
                    sx={{ ml: 1 }}
                  />
                </Typography>
              </Box>
              <Typography variant="body2" gutterBottom>
                Uptime: {systemHealth?.uptime ? formatUptime(systemHealth.uptime) : 'Unknown'}
              </Typography>
              <Typography variant="body2" gutterBottom>
                Database Connection: {systemHealth?.database_connection ? 'Connected' : 'Disconnected'}
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Services Status
              </Typography>
              <TableContainer>
                <Table size="small">
                  <TableHead>
                    <TableRow>
                      <TableCell>Service</TableCell>
                      <TableCell>Status</TableCell>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {overview?.service_status && Object.entries(overview.service_status).map(([service, status]) => (
                      <TableRow key={service}>
                        <TableCell>
                          {service.replace('_', ' ').replace(/\b\w/g, l => l.toUpperCase())}
                        </TableCell>
                        <TableCell>
                          <Chip 
                            label={status}
                            color={getServiceStatusColor(status)}
                            size="small"
                            icon={getServiceStatusIcon(status)}
                          />
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </TableContainer>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* System Statistics */}
      <Grid container spacing={3}>
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                User Statistics
              </Typography>
              {systemStats?.users && (
                <Box>
                  <Typography variant="body2">Total: {systemStats.users.total}</Typography>
                  <Typography variant="body2">Active: {systemStats.users.active}</Typography>
                  <Typography variant="body2">Admins: {systemStats.users.admins}</Typography>
                  <Typography variant="body2">Recent Logins (24h): {systemStats.users.recent_logins}</Typography>
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Document Statistics
              </Typography>
              {systemStats?.documents && (
                <Box>
                  <Typography variant="body2">Total Files: {systemStats.documents.total_files}</Typography>
                  <Typography variant="body2">Total Folders: {systemStats.documents.total_folders}</Typography>
                  <Typography variant="body2">Total Size: {formatBytes(systemStats.documents.total_size)}</Typography>
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Configuration Statistics
              </Typography>
              {systemStats?.configurations && (
                <Box>
                  <Typography variant="body2">Total Configs: {systemStats.configurations.total}</Typography>
                  <Typography variant="body2">Encrypted: {systemStats.configurations.encrypted}</Typography>
                  <Typography variant="body2">
                    Encryption Rate: {systemStats.configurations.total > 0 ? 
                      ((systemStats.configurations.encrypted / systemStats.configurations.total) * 100).toFixed(1) : 0}%
                  </Typography>
                  {systemStats.configurations.total > 0 && (
                    <LinearProgress 
                      variant="determinate" 
                      value={(systemStats.configurations.encrypted / systemStats.configurations.total) * 100}
                      sx={{ mt: 1 }}
                    />
                  )}
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Recent Activity */}
      {systemStats?.recent_activity && systemStats.recent_activity.length > 0 && (
        <Card sx={{ mt: 3 }}>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Recent Activity
            </Typography>
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Timestamp</TableCell>
                    <TableCell>User</TableCell>
                    <TableCell>Action</TableCell>
                    <TableCell>Details</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {systemStats.recent_activity.map((activity, index) => (
                    <TableRow key={index}>
                      <TableCell>{new Date(activity.timestamp).toLocaleString()}</TableCell>
                      <TableCell>{activity.user}</TableCell>
                      <TableCell>{activity.action}</TableCell>
                      <TableCell>{activity.details}</TableCell>
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