import React, { useState } from "react";
import {
  Box,
  Chip,
  Divider,
  Grid,
  IconButton,
  LinearProgress,
  Tooltip,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import MemoryIcon from "@mui/icons-material/Memory";
import ThermostatIcon from "@mui/icons-material/Thermostat";
import BoltIcon from "@mui/icons-material/Bolt";
import SpeedIcon from "@mui/icons-material/Speed";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import { GpuStats } from "../data/GpuStats";
import BaseCard from "./BaseCard";

const MetricBox = styled(Box)(({ theme }) => ({
  padding: theme.spacing(1),
  borderRadius: theme.shape.borderRadius,
  backgroundColor: theme.palette.background.default,
  border: `1px solid ${theme.palette.divider}`,
}));

interface GpuCardProps {
  gpuStats: GpuStats | null;
  loading?: boolean;
}

const GpuCard: React.FC<GpuCardProps> = ({ gpuStats }) => {
  const [selectedGpu, setSelectedGpu] = useState(0);
  if (!gpuStats || !gpuStats.gpus || gpuStats.gpus.length === 0) {
    return (
      <BaseCard icon={MemoryIcon} title="GPU Status">
        <Typography variant="body2" color="text.secondary">
          No GPU detected or data unavailable
        </Typography>
      </BaseCard>
    );
  }
  // Use the first GPU for display (could be enhanced to show multiple GPUs)
  const gpu = gpuStats.gpus[selectedGpu];
  const {
    name,
    vendor,
    utilization,
    memory_used,
    memory_total,
    memory_usage_percentage,
    temperature,
    power_usage,
    clock_speed,
    memory_clock,
  } = gpu;
  const formatBytes = (bytes: number): string => {
    const gb = bytes / (1024 * 1024 * 1024);
    return `${gb.toFixed(1)} GB`;
  };
  const formatMHz = (mhz?: number): string => {
    if (!mhz) return "N/A";
    return mhz >= 1000 ? `${(mhz / 1000).toFixed(1)} GHz` : `${mhz} MHz`;
  };
  return (
    <BaseCard
      icon={MemoryIcon}
      title="GPU"
      headerActions={
        <Chip
          label={vendor.toUpperCase()}
          size="small"
          color={
            vendor.toLowerCase().includes("nvidia") ? "success" : "warning"
          }
        />
      }
    >
      {/* GPU Name */}
      <Typography
        variant="body2"
        color="text.secondary"
        mb={2}
        noWrap
        title={name}
      >
        {name}
      </Typography>
      {/* Utilization */}
      <Box mb={2}>
        <Box
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mb={1}
        >
          <Typography variant="body2" fontWeight="medium">
            GPU Utilization
          </Typography>
          <Typography variant="body2">
            {utilization ? `${utilization.toFixed(1)}%` : "N/A"}
          </Typography>
        </Box>
        <LinearProgress
          variant="determinate"
          value={utilization || 0}
          sx={{ height: 6, borderRadius: 3 }}
          color={utilization && utilization > 80 ? "warning" : "primary"}
        />
      </Box>
      {/* Memory Usage */}
      <Box mb={2}>
        <Box
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mb={1}
        >
          <Typography variant="body2" fontWeight="medium">
            Memory Usage
          </Typography>
          <Typography variant="body2">
            {formatBytes(memory_used)} / {formatBytes(memory_total)}
          </Typography>
        </Box>
        <LinearProgress
          variant="determinate"
          value={memory_usage_percentage}
          sx={{ height: 6, borderRadius: 3 }}
          color={memory_usage_percentage > 80 ? "error" : "info"}
        />
      </Box>
      <Divider sx={{ my: 2 }} /> {/* Metrics Grid */}
      <Grid container spacing={1}>
        {temperature && (
          <Grid size={6}>
            <MetricBox>
              <Box display="flex" alignItems="center" mb={0.5}>
                <ThermostatIcon
                  sx={{ fontSize: 16, mr: 0.5, color: "warning.main" }}
                />
                <Typography variant="caption" color="text.secondary">
                  Temp
                </Typography>
              </Box>
              <Typography variant="body2" fontWeight="bold">
                {temperature.toFixed(1)}°C
              </Typography>
            </MetricBox>
          </Grid>
        )}

        {power_usage && (
          <Grid size={6}>
            <MetricBox>
              <Box display="flex" alignItems="center" mb={0.5}>
                <BoltIcon sx={{ fontSize: 16, mr: 0.5, color: "error.main" }} />
                <Typography variant="caption" color="text.secondary">
                  Power
                </Typography>
              </Box>
              <Typography variant="body2" fontWeight="bold">
                {power_usage.toFixed(1)}W
              </Typography>
            </MetricBox>
          </Grid>
        )}

        {clock_speed && (
          <Grid size={6}>
            <MetricBox>
              <Box display="flex" alignItems="center" mb={0.5}>
                <SpeedIcon sx={{ fontSize: 16, mr: 0.5, color: "info.main" }} />
                <Typography variant="caption" color="text.secondary">
                  GPU Clock
                </Typography>
              </Box>
              <Typography variant="body2" fontWeight="bold">
                {formatMHz(clock_speed)}
              </Typography>
            </MetricBox>
          </Grid>
        )}

        {memory_clock && (
          <Grid size={6}>
            <MetricBox>
              <Box display="flex" alignItems="center" mb={0.5}>
                <MemoryIcon
                  sx={{ fontSize: 16, mr: 0.5, color: "secondary.main" }}
                />
                <Typography variant="caption" color="text.secondary">
                  Mem Clock
                </Typography>
              </Box>
              <Typography variant="body2" fontWeight="bold">
                {formatMHz(memory_clock)}
              </Typography>
            </MetricBox>
          </Grid>
        )}
      </Grid>
      {/* Navigation for multiple GPUs */}
      {gpuStats.gpus.length > 1 && (
        <>
          <Divider sx={{ my: 1.5 }} />
          <Box display="flex" justifyContent="center" alignItems="center">
            <Tooltip title="Previous GPU">
              <IconButton
                size="small"
                onClick={() => setSelectedGpu((prev) => Math.max(prev - 1, 0))}
                disabled={selectedGpu === 0}
              >
                <ChevronLeftIcon fontSize="small" />
              </IconButton>
            </Tooltip>
            <Typography variant="caption" color="text.secondary" mx={1}>
              Showing GPU {selectedGpu + 1} of {gpuStats.gpus.length}
            </Typography>
            <Tooltip title="Next GPU">
              <IconButton
                size="small"
                onClick={() =>
                  setSelectedGpu((prev) =>
                    Math.min(prev + 1, gpuStats.gpus.length - 1),
                  )
                }
                disabled={selectedGpu === gpuStats.gpus.length - 1}
              >
                <ChevronRightIcon fontSize="small" />
              </IconButton>
            </Tooltip>
          </Box>
        </>
      )}
    </BaseCard>
  );
};
export default GpuCard;
