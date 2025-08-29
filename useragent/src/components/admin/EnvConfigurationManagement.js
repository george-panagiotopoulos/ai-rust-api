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
  MenuItem
} from '@mui/material';
import {
  Edit as EditIcon,
  Visibility as VisibilityIcon,
  VisibilityOff as VisibilityOffIcon,
  CheckCircle as ValidIcon,
  Error as ErrorIcon,
  Warning as WarningIcon,
  ExpandMore as ExpandMoreIcon,
  Security as SecurityIcon,
  Settings as SettingsIcon,
  Storage as StorageIcon,
  Refresh as RefreshIcon
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

  useEffect(() => {
    fetchConfigs();
  }, []);

  const fetchConfigs = async () => {
    try {
      setLoading(true);
      setError('');
      const response = await adminService.getEnvConfigs();
      setConfigs(response.configs || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
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

  const getServiceIcon = (service) => {
    switch (service) {
      case 'Generic': return <StorageIcon sx={{ color: 'primary.main' }} />;
      case 'AuthAPI': return <SecurityIcon sx={{ color: 'warning.main' }} />;
      case 'BedrockAPI': return <SettingsIcon sx={{ color: 'info.main' }} />;
      case 'RAGAPI': return <StorageIcon sx={{ color: 'success.main' }} />;
      case 'UIConfigAPI': return <SettingsIcon sx={{ color: 'secondary.main' }} />;
      default: return <SettingsIcon />;
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

  const filteredConfigs = configs.filter(config => 
    serviceFilter === 'All' || config.service === serviceFilter
  );

  const groupedConfigs = filteredConfigs.reduce((groups, config) => {
    const service = config.service;
    if (!groups[service]) {
      groups[service] = [];
    }
    groups[service].push(config);
    return groups;
  }, {});

  const services = ['All', ...new Set(configs.map(config => config.service))];

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
          Environment Configuration
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

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="body1" color="textSecondary">
          Manage configuration values across all microservices. Changes are applied to .env files automatically.
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

      {Object.entries(groupedConfigs).map(([service, serviceConfigs]) => (
        <Accordion key={service} defaultExpanded={service === 'Generic'} sx={{ mb: 2 }}>
          <AccordionSummary expandIcon={<ExpandMoreIcon />}>
            <Box display="flex" alignItems="center" sx={{ width: '100%' }}>
              {getServiceIcon(service)}
              <Typography variant="h6" sx={{ ml: 1, flexGrow: 1 }}>
                {service} ({serviceConfigs.length} configs)
              </Typography>
              <Box sx={{ mr: 2 }}>
                {serviceConfigs.filter(c => c.required && !c.current_value).length > 0 && (
                  <Chip 
                    label={`${serviceConfigs.filter(c => c.required && !c.current_value).length} Missing`}
                    color="error" 
                    size="small" 
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
                    <TableCell>Description</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Current Value</TableCell>
                    <TableCell>Default</TableCell>
                    <TableCell align="right">Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {serviceConfigs.map((config) => (
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