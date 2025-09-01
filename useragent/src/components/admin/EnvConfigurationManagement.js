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
  InputAdornment,
  IconButton,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  Tooltip,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Switch,
  FormControlLabel
} from '@mui/material';
import {
  Edit as EditIcon,
  Visibility as VisibilityIcon,
  VisibilityOff as VisibilityOffIcon,
  CheckCircle as ValidIcon,
  Error as ErrorIcon,
  ExpandMore as ExpandMoreIcon,
  Security as SecurityIcon,
  Settings as SettingsIcon,
  Storage as StorageIcon,
  Refresh as RefreshIcon,
  Cloud as AWSIcon,
  Microsoft as AzureIcon
} from '@mui/icons-material';
import adminService from '../../services/adminService';

const EnvConfigurationManagement = () => {
  const [configs, setConfigs] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [validationResult, setValidationResult] = useState(null);
  
  // Dialog states
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [validationDialogOpen, setValidationDialogOpen] = useState(false);
  
  // Form states
  const [editConfig, setEditConfig] = useState(null);
  const [showPassword, setShowPassword] = useState(false);
  const [serviceFilter, setServiceFilter] = useState('All');
  
  // Backend switching states
  const [selectedBackend, setSelectedBackend] = useState('AWS');
  const [backendSwitchSuccess, setBackendSwitchSuccess] = useState('');

  useEffect(() => {
    const initializeBackendAndConfigs = async () => {
      await fetchCurrentBackend();
      // fetchConfigs will be called after selectedBackend is updated
    };
    initializeBackendAndConfigs();
  }, []);

  // Fetch configs whenever selectedBackend changes
  useEffect(() => {
    fetchConfigs();
  }, [selectedBackend]); // eslint-disable-line react-hooks/exhaustive-deps

  const fetchConfigs = async () => {
    try {
      setLoading(true);
      setError('');
      // Always fetch the standard env configs
      const response = await adminService.getEnvConfigs();
      setConfigs(response.configs || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const fetchCurrentBackend = async () => {
    try {
      const response = await adminService.getBackendStatus();
      if (response.active_backend) {
        setSelectedBackend(response.active_backend.toUpperCase());
      }
    } catch (err) {
      console.warn('Could not fetch current backend status:', err.message);
      // Default to AWS if we can't determine the current backend
    }
  };

  const handleUpdateConfig = async () => {
    try {
      setError('');
      const response = await adminService.updateEnvConfig(editConfig.key, editConfig.value);
      if (response.success) {
        setSuccess(`Configuration updated successfully! Files updated: ${response.updated_files.join(', ')}`);
        setEditDialogOpen(false);
        setEditConfig(null);
        fetchConfigs();
      } else {
        setError(response.message || 'Failed to update configuration');
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const handleValidateConfigs = async () => {
    try {
      setError('');
      const response = await adminService.validateEnvConfigs();
      setValidationResult(response);
      setValidationDialogOpen(true);
    } catch (err) {
      setError(err.message);
    }
  };

  const handleBackendSwitch = async (newBackend) => {
    try {
      setError('');
      setBackendSwitchSuccess('');
      const response = await adminService.switchBackend(newBackend.toLowerCase());
      if (response.success) {
        setSelectedBackend(newBackend);
        setBackendSwitchSuccess(`Successfully switched to ${newBackend} backend`);
        fetchConfigs(); // Refresh configs after backend switch
      } else {
        setError(response.message || `Failed to switch to ${newBackend} backend`);
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const openEditDialog = (config) => {
    setEditConfig({
      key: config.key,
      value: config.sensitive ? '' : (config.current_value || config.default_value || ''),
      service: config.service,
      description: config.description,
      sensitive: config.sensitive,
      required: config.required
    });
    setEditDialogOpen(true);
  };

  const getCategoryIcon = (category) => {
    switch (category) {
      case 'AI Provider Credentials & Models': return <SecurityIcon sx={{ color: 'error.main' }} />;
      case 'Service URLs & Connectivity': return <StorageIcon sx={{ color: 'info.main' }} />;
      case 'System Settings': return <SettingsIcon sx={{ color: 'success.main' }} />;
      case 'Other Configurations': return <SettingsIcon sx={{ color: 'grey.main' }} />;
      default: return <SettingsIcon />;
    }
  };

  const getCategoryDescription = (category) => {
    switch (category) {
      case 'AI Provider Credentials & Models': return 'AWS and Azure credentials, API keys, and AI model configurations';
      case 'Service URLs & Connectivity': return 'Service endpoints, hosts, and port configurations';
      case 'System Settings': return 'Security settings, timeouts, and system parameters';
      case 'Other Configurations': return 'Additional configuration options';
      default: return '';
    }
  };

  const getStatusChip = (config) => {
    if (config.required && !config.current_value) {
      return <Chip label="Missing" color="error" size="small" icon={<ErrorIcon />} />;
    } else if (config.current_value) {
      return <Chip label="Set" color="success" size="small" icon={<ValidIcon />} />;
    } else {
      return <Chip label="Default" color="default" size="small" />;
    }
  };

  // Filter configs based on selected backend - show backend-specific configs + all common ones
  const getBackendSpecificConfigs = (configs, selectedBackend) => {
    if (!configs || configs.length === 0) return [];
    
    return configs.filter(config => {
      const key = config.key.toLowerCase();
      
      // AWS-specific variables (complete list)
      const awsSpecificKeys = [
        'aws_access_key_id',
        'aws_secret_access_key', 
        'aws_region',
        'embedding_model', // AWS Bedrock embedding model
        'bedrock_api_url'  // Bedrock API service URL
      ];
      
      // Azure-specific variables (complete list)
      const azureSpecificKeys = [
        'azure_openai_endpoint',
        'azure_openai_api_key',
        'azure_openai_api_version',
        'azure_openai_deployment',
        'azure_openai_embedding_model'
      ];
      
      // Check if this config is AWS-specific
      const isAWSSpecific = awsSpecificKeys.includes(key);
      
      // Check if this config is Azure-specific  
      const isAzureSpecific = azureSpecificKeys.includes(key);
      
      if (selectedBackend === 'AWS') {
        // Show AWS configs and all non-Azure configs
        return !isAzureSpecific;
      } else if (selectedBackend === 'Azure') {
        // Show Azure configs and all non-AWS configs
        return !isAWSSpecific;
      }
      
      return true; // Default: show all
    });
  };

  const backendFilteredConfigs = getBackendSpecificConfigs(configs, selectedBackend);
  
  const filteredConfigs = backendFilteredConfigs.filter(config => 
    serviceFilter === 'All' || config.service === serviceFilter
  );

  // Group configurations by logical categories - standard categories, not backend-specific
  const categorizeConfig = (config) => {
    const key = config.key.toLowerCase();
    
    // AI/Cloud Provider specific credentials and models  
    if (key.includes('aws') || key.includes('azure') || key.includes('openai') || key.includes('bedrock') ||
        key.includes('key') || key.includes('secret') || key.includes('token') ||
        key.includes('model') || key.includes('embedding') || key.includes('region')) {
      return 'AI Provider Credentials & Models';
    }
    
    // Service URLs and endpoints
    if (key.includes('url') || key.includes('host') || key.includes('port') || 
        key.includes('endpoint') || key.includes('base_url')) {
      return 'Service URLs & Connectivity';
    }
    
    // System settings
    if (key.includes('cost') || key.includes('expiry') || key.includes('timeout') || 
        key.includes('log') || key.includes('debug') || key.includes('pool') || 
        key.includes('connection') || key.includes('database')) {
      return 'System Settings';
    }
    
    // Default category
    return 'Other Configurations';
  };

  const groupedConfigs = filteredConfigs.reduce((groups, config) => {
    const category = categorizeConfig(config);
    if (!groups[category]) {
      groups[category] = [];
    }
    groups[category].push(config);
    return groups;
  }, {});

  // Define the order of categories - fixed, not backend-dependent
  const categoryOrder = [
    'AI Provider Credentials & Models',
    'Service URLs & Connectivity', 
    'System Settings',
    'Other Configurations'
  ];

  // Sort the grouped configs according to the defined order
  const sortedGroupedConfigs = {};
  categoryOrder.forEach(category => {
    if (groupedConfigs[category]) {
      sortedGroupedConfigs[category] = groupedConfigs[category].sort((a, b) => a.key.localeCompare(b.key));
    }
  });
  
  // Add any remaining categories that weren't in the predefined order
  Object.keys(groupedConfigs).forEach(category => {
    if (!categoryOrder.includes(category)) {
      sortedGroupedConfigs[category] = groupedConfigs[category].sort((a, b) => a.key.localeCompare(b.key));
    }
  });

  const services = ['All', ...new Set(backendFilteredConfigs.map(config => config.service))];

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
          Environment Variables
        </Typography>
        <Box>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={fetchConfigs}
            sx={{ mr: 2 }}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            startIcon={<ValidIcon />}
            onClick={handleValidateConfigs}
            color="info"
          >
            Validate All
          </Button>
        </Box>
      </Box>

      {/* Backend Selection Toggle */}
      <Card sx={{ mb: 3 }}>
        <CardContent>
          <Box display="flex" justifyContent="space-between" alignItems="center">
            <Box display="flex" alignItems="center">
              {selectedBackend === 'AWS' ? (
                <AWSIcon sx={{ color: 'orange', mr: 1, fontSize: 28 }} />
              ) : (
                <AzureIcon sx={{ color: 'primary.main', mr: 1, fontSize: 28 }} />
              )}
              <Box>
                <Typography variant="h6">
                  AI Backend: {selectedBackend}
                </Typography>
                <Typography variant="body2" color="textSecondary">
                  {selectedBackend === 'AWS' 
                    ? 'Using AWS Bedrock for AI completions and embeddings' 
                    : 'Using Azure OpenAI for AI completions and embeddings'
                  }
                </Typography>
              </Box>
            </Box>
            <Box display="flex" alignItems="center">
              <AWSIcon sx={{ color: selectedBackend === 'AWS' ? 'orange' : 'grey.400', mr: 1 }} />
              <FormControlLabel
                control={
                  <Switch
                    checked={selectedBackend === 'Azure'}
                    onChange={(e) => handleBackendSwitch(e.target.checked ? 'Azure' : 'AWS')}
                    color="primary"
                  />
                }
                label=""
                sx={{ mr: 1 }}
              />
              <AzureIcon sx={{ color: selectedBackend === 'Azure' ? 'primary.main' : 'grey.400' }} />
            </Box>
          </Box>
        </CardContent>
      </Card>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}
      {backendSwitchSuccess && <Alert severity="success" sx={{ mb: 2 }}>{backendSwitchSuccess}</Alert>}

      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="body1" color="textSecondary">
          Manage environment variables across all microservices. Variables are grouped by type: user-specific settings, service URLs, and system settings.
        </Typography>
        <FormControl sx={{ minWidth: 120 }}>
          <InputLabel>Service</InputLabel>
          <Select
            value={serviceFilter}
            label="Service"
            onChange={(e) => setServiceFilter(e.target.value)}
            size="small"
          >
            {services.map(service => (
              <MenuItem key={service} value={service}>{service}</MenuItem>
            ))}
          </Select>
        </FormControl>
      </Box>

      {Object.entries(sortedGroupedConfigs).map(([category, categoryConfigs]) => (
        <Accordion key={category} defaultExpanded={category === 'AI Provider Credentials & Models'} sx={{ mb: 2 }}>
          <AccordionSummary expandIcon={<ExpandMoreIcon />}>
            <Box display="flex" alignItems="center" sx={{ width: '100%' }}>
              {getCategoryIcon(category)}
              <Box sx={{ ml: 1, flexGrow: 1 }}>
                <Typography variant="h6">
                  {category} ({categoryConfigs.length} variables)
                </Typography>
                <Typography variant="caption" color="textSecondary">
                  {getCategoryDescription(category)}
                </Typography>
              </Box>
              <Box sx={{ mr: 2 }}>
                {categoryConfigs.filter(c => c.required && !c.current_value).length > 0 && (
                  <Chip 
                    label={`${categoryConfigs.filter(c => c.required && !c.current_value).length} Missing`}
                    color="error" 
                    size="small" 
                  />
                )}
                {categoryConfigs.filter(c => c.sensitive).length > 0 && (
                  <Chip 
                    label={`${categoryConfigs.filter(c => c.sensitive).length} Sensitive`}
                    color="warning" 
                    size="small"
                    sx={{ ml: 1 }}
                  />
                )}
              </Box>
            </Box>
          </AccordionSummary>
          <AccordionDetails>
            <TableContainer component={Paper} elevation={0}>
              <Table size="small">
                <TableHead>
                  <TableRow>
                    <TableCell>Key</TableCell>
                    <TableCell>Service</TableCell>
                    <TableCell>Description</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Current Value</TableCell>
                    <TableCell>Default</TableCell>
                    <TableCell align="right">Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {categoryConfigs.map((config) => (
                    <TableRow key={config.key}>
                      <TableCell>
                        <Typography variant="body2" sx={{ fontFamily: 'monospace', fontWeight: 'bold' }}>
                          {config.key}
                        </Typography>
                        {config.required && (
                          <Chip label="Required" color="warning" size="small" sx={{ ml: 1 }} />
                        )}
                        {config.sensitive && (
                          <Chip label="Sensitive" color="error" size="small" sx={{ ml: 1 }} />
                        )}
                      </TableCell>
                      <TableCell>
                        <Chip 
                          label={config.service} 
                          size="small" 
                          variant="outlined"
                          color={
                            config.service === 'AuthAPI' ? 'warning' :
                            config.service === 'BedrockAPI' ? 'info' :
                            config.service === 'RAGAPI' ? 'success' :
                            config.service === 'UIConfigAPI' ? 'secondary' :
                            'default'
                          }
                        />
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2" sx={{ maxWidth: 300 }}>
                          {config.description}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        {getStatusChip(config)}
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
                          {config.current_value || 'Not set'}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Typography 
                          variant="body2" 
                          color="textSecondary"
                          sx={{ 
                            fontFamily: 'monospace',
                            maxWidth: 150,
                            overflow: 'hidden',
                            textOverflow: 'ellipsis',
                            whiteSpace: 'nowrap'
                          }}
                        >
                          {config.default_value || 'None'}
                        </Typography>
                      </TableCell>
                      <TableCell align="right">
                        <Tooltip title="Edit Configuration">
                          <IconButton onClick={() => openEditDialog(config)} size="small">
                            <EditIcon />
                          </IconButton>
                        </Tooltip>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </AccordionDetails>
        </Accordion>
      ))}

      {/* Edit Configuration Dialog */}
      <Dialog open={editDialogOpen} onClose={() => setEditDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>
          Edit Configuration: {editConfig?.key}
          <Typography variant="body2" color="textSecondary">
            Service: {editConfig?.service}
          </Typography>
        </DialogTitle>
        <DialogContent>
          <Typography variant="body2" color="textSecondary" sx={{ mb: 2 }}>
            {editConfig?.description}
          </Typography>
          
          {editConfig?.required && (
            <Alert severity="warning" sx={{ mb: 2 }}>
              This is a required configuration. Make sure to provide a valid value.
            </Alert>
          )}

          {editConfig?.sensitive && (
            <Alert severity="error" sx={{ mb: 2 }}>
              This configuration contains sensitive information. Handle with care.
            </Alert>
          )}

          <TextField
            autoFocus
            margin="dense"
            label="Value"
            fullWidth
            variant="outlined"
            type={editConfig?.sensitive && !showPassword ? 'password' : 'text'}
            value={editConfig?.value || ''}
            onChange={(e) => setEditConfig({ ...editConfig, value: e.target.value })}
            sx={{ mb: 2 }}
            InputProps={{
              endAdornment: editConfig?.sensitive && (
                <InputAdornment position="end">
                  <IconButton
                    onClick={() => setShowPassword(!showPassword)}
                    edge="end"
                  >
                    {showPassword ? <VisibilityOffIcon /> : <VisibilityIcon />}
                  </IconButton>
                </InputAdornment>
              ),
              sx: { fontFamily: 'monospace' }
            }}
            helperText={editConfig?.default_value ? `Default: ${editConfig.default_value}` : 'No default value'}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEditDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleUpdateConfig}
            variant="contained"
            disabled={editConfig?.required && !editConfig?.value}
          >
            Update Configuration
          </Button>
        </DialogActions>
      </Dialog>

      {/* Validation Dialog */}
      <Dialog open={validationDialogOpen} onClose={() => setValidationDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>
          Configuration Validation Results
        </DialogTitle>
        <DialogContent>
          {validationResult && (
            <Box>
              <Box display="flex" alignItems="center" mb={2}>
                {validationResult.valid ? (
                  <ValidIcon sx={{ color: 'success.main', mr: 1 }} />
                ) : (
                  <ErrorIcon sx={{ color: 'error.main', mr: 1 }} />
                )}
                <Typography variant="h6">
                  {validationResult.valid ? 'All configurations are valid' : 'Configuration issues found'}
                </Typography>
              </Box>

              {!validationResult.valid && validationResult.errors.length > 0 && (
                <Card>
                  <CardContent>
                    <Typography variant="subtitle1" gutterBottom>
                      Issues:
                    </Typography>
                    {validationResult.errors.map((error, index) => (
                      <Alert key={index} severity="error" sx={{ mb: 1 }}>
                        {error}
                      </Alert>
                    ))}
                  </CardContent>
                </Card>
              )}
            </Box>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setValidationDialogOpen(false)}>Close</Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default EnvConfigurationManagement;