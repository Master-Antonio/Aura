import React, { useState, useEffect } from "react";
import {
  Box,
  Chip,
  Divider,
  Grid,
  Typography,
  Alert,
  CircularProgress,
  Tooltip,
  IconButton,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { SvgIconComponent } from "@mui/icons-material";
import RefreshIcon from "@mui/icons-material/Refresh";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import ErrorIcon from "@mui/icons-material/Error";
import WarningIcon from "@mui/icons-material/Warning";
import BaseCard from "./BaseCard";
import { invoke } from "@tauri-apps/api/core";

const MetricBox = styled(Box)(({ theme }) => ({
  padding: theme.spacing(1),
  borderRadius: theme.shape.borderRadius,
  backgroundColor: theme.palette.background.default,
  border: `1px solid ${theme.palette.divider}`,
}));

interface MonitorHealth {
  cpu_healthy: boolean;
  memory_healthy: boolean;
  storage_healthy: boolean;
  network_healthy: boolean;
  gpu_healthy: boolean;
  system_healthy: boolean;
  last_health_check: number;
  error_counts: Record<string, number>;
}

export interface MonitorHealthCardProps {
  icon: SvgIconComponent;
  title: string;
}

const MonitorHealthCard: React.FC<MonitorHealthCardProps> = ({
  icon,
  title,
}) => {
  const [health, setHealth] = useState<MonitorHealth | null>(null);
  const [loading, setLoading] = useState(true);
  const fetchHealth = async () => {
    try {
      setLoading(true);
      const healthData = await invoke<MonitorHealth>("get_monitor_health");
      setHealth(healthData);
    } catch (error) {
      console.error("Failed to fetch monitor health:", error);
    } finally {
      setLoading(false);
    }
  };

  const resetHealth = async () => {
    try {
      await invoke("reset_monitor_health");
      await fetchHealth(); // Refresh after reset
    } catch (error) {
      console.error("Failed to reset monitor health:", error);
    }
  };

  useEffect(() => {
    fetchHealth();
    
    // Update health every 10 seconds
    const interval = setInterval(fetchHealth, 10000);
    return () => clearInterval(interval);
  }, []);

  if (!health) {
    return (
      <BaseCard icon={icon} title={title}>
        <Box display="flex" justifyContent="center" p={2}>
          <CircularProgress size={24} />
        </Box>
      </BaseCard>
    );
  }

  const components = [
    { name: "CPU", healthy: health.cpu_healthy, errors: health.error_counts.cpu || 0 },
    { name: "Memory", healthy: health.memory_healthy, errors: health.error_counts.memory || 0 },
    { name: "Storage", healthy: health.storage_healthy, errors: health.error_counts.storage || 0 },
    { name: "Network", healthy: health.network_healthy, errors: health.error_counts.network || 0 },
    { name: "GPU", healthy: health.gpu_healthy, errors: health.error_counts.gpu || 0 },
    { name: "System", healthy: health.system_healthy, errors: health.error_counts.system || 0 },
  ];

  const overallHealthy = components.every(c => c.healthy);
  const totalErrors = components.reduce((sum, c) => sum + c.errors, 0);

  const getHealthIcon = (healthy: boolean, errors: number) => {
    if (healthy && errors === 0) return <CheckCircleIcon color="success" fontSize="small" />;
    if (healthy && errors > 0) return <WarningIcon color="warning" fontSize="small" />;
    return <ErrorIcon color="error" fontSize="small" />;
  };

  const getHealthColor = (healthy: boolean, errors: number) => {
    if (healthy && errors === 0) return "success";
    if (healthy && errors > 0) return "warning";
    return "error";
  };

  return (
    <BaseCard
      icon={icon}
      title={title}
      headerActions={
        <Box display="flex" alignItems="center" gap={1}>
          <Chip 
            label={overallHealthy ? "Healthy" : "Issues"}
            size="small"
            color={overallHealthy ? "success" : "error"}
          />
          <Tooltip title="Reset Health Status">
            <IconButton size="small" onClick={resetHealth} disabled={loading}>
              <RefreshIcon fontSize="small" />
            </IconButton>
          </Tooltip>
        </Box>
      }
    >
      {/* Overall Status */}
      <Typography variant="body2" color="text.secondary" mb={2}>
        System Monitor Health Status
      </Typography>

      {/* Health Summary */}
      <Box mb={2}>
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={1}>
          <Typography variant="caption" color="text.secondary">
            Overall Health
          </Typography>
          <Typography variant="body2" fontWeight="bold" color={overallHealthy ? "success.main" : "error.main"}>
            {overallHealthy ? "All Systems Operational" : `${components.filter(c => !c.healthy).length} Issues Detected`}
          </Typography>
        </Box>

        {totalErrors > 0 && (
          <Alert severity={overallHealthy ? "warning" : "error"} sx={{ mb: 2 }}>
            Total Errors: {totalErrors} | Last Check: {new Date(health.last_health_check * 1000).toLocaleTimeString()}
          </Alert>
        )}
      </Box>

      <Divider sx={{ my: 2 }} />      {/* Component Health Grid */}
      <Grid container spacing={1}>
        {components.map((component) => (
          <Grid key={component.name} size={6}>
            <MetricBox>
              <Box display="flex" alignItems="center" justifyContent="space-between" mb={0.5}>
                <Box display="flex" alignItems="center" gap={0.5}>
                  {getHealthIcon(component.healthy, component.errors)}
                  <Typography variant="caption" color="text.secondary">
                    {component.name}
                  </Typography>
                </Box>
                <Chip
                  size="small"
                  label={component.healthy ? "OK" : "ERROR"}
                  color={getHealthColor(component.healthy, component.errors) as any}
                />
              </Box>
              {component.errors > 0 && (
                <Typography variant="caption" color="text.secondary">
                  {component.errors} error{component.errors !== 1 ? 's' : ''}
                </Typography>
              )}
            </MetricBox>
          </Grid>
        ))}
      </Grid>

      {/* Performance Tips */}
      <Divider sx={{ my: 2 }} />
      <Typography variant="caption" color="text.secondary">
        Monitoring refreshes every 10 seconds. High error counts may indicate system stress.
      </Typography>
    </BaseCard>
  );
};

export default MonitorHealthCard;
