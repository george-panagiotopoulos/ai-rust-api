import React, { useState } from 'react';
import {
  Container,
  Paper,
  Typography,
  Box,
  AppBar,
  Toolbar,
  IconButton,
  Tabs,
  Tab,
  Card,
  CardContent,
  Grid
} from '@mui/material';
import {
  Logout,
  People,
  Settings,
  FolderOpen,
  Dashboard as DashboardIcon,
  Storage as VectorIcon,
  SmartToy as RagModelIcon
} from '@mui/icons-material';
import { useAuth } from '../context/AuthContext';
import UserManagement from './admin/UserManagement';
import EnvConfigurationManagement from './admin/EnvConfigurationManagement';
import DocumentManagement from './admin/DocumentManagement';
import VectorManagement from './admin/VectorManagement';
import RagModelManagement from './admin/RagModelManagement';
import AdminDashboardOverview from './admin/AdminDashboard';

function TabPanel({ children, value, index, ...other }) {
  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`admin-tabpanel-${index}`}
      aria-labelledby={`admin-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ p: 3 }}>
          {children}
        </Box>
      )}
    </div>
  );
}

const AdminDashboard = () => {
  const { user, logout } = useAuth();
  const [tabValue, setTabValue] = useState(0);

  const handleTabChange = (event, newValue) => {
    setTabValue(newValue);
  };

  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Admin Dashboard - Welcome, {user?.username}
          </Typography>
          <IconButton
            size="large"
            edge="end"
            color="inherit"
            onClick={logout}
          >
            <Logout />
          </IconButton>
        </Toolbar>
      </AppBar>

      <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
        <Paper elevation={3}>
          <Tabs
            value={tabValue}
            onChange={handleTabChange}
            variant="scrollable"
            scrollButtons="auto"
            sx={{ borderBottom: 1, borderColor: 'divider' }}
          >
            <Tab 
              label="Overview" 
              icon={<DashboardIcon />} 
              iconPosition="start"
              id="admin-tab-0"
              aria-controls="admin-tabpanel-0"
            />
            <Tab 
              label="User Management" 
              icon={<People />} 
              iconPosition="start"
              id="admin-tab-1"
              aria-controls="admin-tabpanel-1"
            />
            <Tab 
              label="Configuration" 
              icon={<Settings />} 
              iconPosition="start"
              id="admin-tab-2"
              aria-controls="admin-tabpanel-2"
            />
            <Tab 
              label="Documents" 
              icon={<FolderOpen />} 
              iconPosition="start"
              id="admin-tab-3"
              aria-controls="admin-tabpanel-3"
            />
            <Tab 
              label="Vectors" 
              icon={<VectorIcon />} 
              iconPosition="start"
              id="admin-tab-4"
              aria-controls="admin-tabpanel-4"
            />
            <Tab 
              label="RAG Models" 
              icon={<RagModelIcon />} 
              iconPosition="start"
              id="admin-tab-5"
              aria-controls="admin-tabpanel-5"
            />
          </Tabs>

          <TabPanel value={tabValue} index={0}>
            <AdminDashboardOverview />
          </TabPanel>

          <TabPanel value={tabValue} index={1}>
            <UserManagement />
          </TabPanel>

          <TabPanel value={tabValue} index={2}>
            <EnvConfigurationManagement />
          </TabPanel>

          <TabPanel value={tabValue} index={3}>
            <DocumentManagement />
          </TabPanel>

          <TabPanel value={tabValue} index={4}>
            <VectorManagement />
          </TabPanel>

          <TabPanel value={tabValue} index={5}>
            <RagModelManagement />
          </TabPanel>
        </Paper>
      </Container>
    </Box>
  );
};

export default AdminDashboard;