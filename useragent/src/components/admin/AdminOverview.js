import React from 'react';
import {
  Grid,
  Card,
  CardContent,
  Typography,
  Box,
  Chip
} from '@mui/material';
import {
  People,
  Settings,
  FolderOpen,
  Security
} from '@mui/icons-material';

const StatCard = ({ title, value, icon, color = 'primary' }) => (
  <Card sx={{ height: '100%' }}>
    <CardContent>
      <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
        {icon}
        <Typography variant="h6" component="div" sx={{ ml: 1 }}>
          {title}
        </Typography>
      </Box>
      <Typography variant="h3" color={`${color}.main`} gutterBottom>
        {value}
      </Typography>
    </CardContent>
  </Card>
);

const AdminOverview = () => {
  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        System Overview
      </Typography>
      <Typography variant="body1" color="textSecondary" paragraph>
        Welcome to the AI Rust API Admin Dashboard. Here you can manage users, configure system settings, and handle document uploads.
      </Typography>

      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Active Users"
            value="N/A"
            icon={<People color="primary" />}
            color="primary"
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Configurations"
            value="N/A"
            icon={<Settings color="secondary" />}
            color="secondary"
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Documents"
            value="N/A"
            icon={<FolderOpen color="success" />}
            color="success"
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Security"
            value="OK"
            icon={<Security color="info" />}
            color="info"
          />
        </Grid>
      </Grid>

      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                System Status
              </Typography>
              <Box sx={{ display: 'flex', gap: 1, flexWrap: 'wrap' }}>
                <Chip label="AuthAPI" color="success" variant="outlined" />
                <Chip label="BedrockAPI" color="success" variant="outlined" />
                <Chip label="RAGAPI" color="success" variant="outlined" />
                <Chip label="UIConfigAPI" color="warning" variant="outlined" />
              </Box>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Quick Actions
              </Typography>
              <Typography variant="body2" color="textSecondary">
                • Manage user accounts and permissions
              </Typography>
              <Typography variant="body2" color="textSecondary">
                • Configure API keys and system settings
              </Typography>
              <Typography variant="body2" color="textSecondary">
                • Upload and organize documents for RAG
              </Typography>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
};

export default AdminOverview;