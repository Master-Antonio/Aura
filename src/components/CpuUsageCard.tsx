import React, { useState } from "react";
import {
  Box,
  Chip,
  Divider,
  Grid,
  IconButton,
  LinearProgress,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { SvgIconComponent } from "@mui/icons-material";
import ThermostatIcon from "@mui/icons-material/Thermostat";
import SpeedIcon from "@mui/icons-material/Speed";
import MemoryIcon from "@mui/icons-material/Memory";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";
import { ProgressData } from "../data/SystemStats";
import BaseCard from "./BaseCard";

const MetricBox = styled(Box)(({ theme }) => ({
  padding: theme.spacing(1),
  borderRadius: theme.shape.borderRadius,
  backgroundColor: theme.palette.background.default,
  border: `1px solid ${theme.palette.divider}`,
}));

export interface CpuUsageCardProps {
  icon: SvgIconComponent;
  title: string;
  percentage?: number;
  progressData?: ProgressData[];
  genericData?: any;
}

const CpuUsageCard: React.FC<CpuUsageCardProps> = ({
  icon,
  percentage,
  progressData,
  genericData,
}) => {
  const [expanded, setExpanded] = useState(false);

  // Determina il brand della CPU
  const getCpuBrand = (cpuName: string) => {
    if (cpuName.toLowerCase().includes('amd')) return 'AMD';
    if (cpuName.toLowerCase().includes('intel')) return 'Intel';
    return 'CPU';
  };

  const cpuName = genericData?.cpu_name || "AMD Ryzen 7 7800X3D 8-Core Processor";
  const cpuBrand = getCpuBrand(cpuName);
  const overallUsage = percentage || 0;
  
  // Dati simulati per temperatura e frequenze (da implementare nel backend)
  const temperature = 50.8;
  const baseClock = 4.2;
  const maxClock = 4.7;
  const cores = 8;
  const threads = 8;
  return (
    <BaseCard
      icon={icon}
      title="CPU"
      headerActions={
        <Box display="flex" gap={1} alignItems="center">
          <Chip 
            label={cpuBrand} 
            size="small"
            color={cpuBrand === 'AMD' ? "error" : "info"}
          />
          <Chip 
            label={`${Math.round(overallUsage)}%`} 
            size="small"
            color="success"
          />
          <IconButton
            size="small"
            onClick={() => setExpanded(!expanded)}
            sx={{ ml: 1 }}
          >
            {expanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
          </IconButton>
        </Box>
      }
    >
      {/* CPU Name */}
      <Typography
        variant="body2"
        color="text.secondary"
        mb={2}
        noWrap
        title={cpuName}
      >
        {cpuName}
      </Typography>

      {/* Overall Usage */}
      <Box mb={2}>
        <Box
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mb={1}
        >
          <Typography variant="body2" fontWeight="medium">
            Overall Usage
          </Typography>
          <Typography variant="body2">
            {Math.round(overallUsage)}%
          </Typography>
        </Box>
        <LinearProgress
          variant="determinate"
          value={overallUsage}
          sx={{ height: 6, borderRadius: 3 }}
          color={overallUsage > 80 ? "warning" : "primary"}
        />
      </Box>

      <Divider sx={{ my: 2 }} />      {/* Metrics Grid */}
      <Grid container spacing={1} mb={2}>
        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <ThermostatIcon
                sx={{ fontSize: 16, mr: 0.5, color: "success.main" }}
              />
              <Typography variant="caption" color="text.secondary">
                Temp
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {temperature}°C
            </Typography>
          </MetricBox>
        </Grid>

        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <SpeedIcon sx={{ fontSize: 16, mr: 0.5, color: "info.main" }} />
              <Typography variant="caption" color="text.secondary">
                Base Clock
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {baseClock} GHz
            </Typography>
          </MetricBox>
        </Grid>

        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <SpeedIcon sx={{ fontSize: 16, mr: 0.5, color: "warning.main" }} />
              <Typography variant="caption" color="text.secondary">
                Max Clock
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {maxClock} GHz
            </Typography>
          </MetricBox>
        </Grid>

        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <MemoryIcon
                sx={{ fontSize: 16, mr: 0.5, color: "secondary.main" }}
              />
              <Typography variant="caption" color="text.secondary">
                Cores/Threads
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {cores}/{threads}
            </Typography>
          </MetricBox>
        </Grid>
      </Grid>      <Divider sx={{ my: 2 }} />      {/* Core Usage - solo se espanso */}
      {expanded && (
        <>
          <Typography variant="body2" fontWeight="medium" mb={1}>
            Core Usage
          </Typography>
          <Grid container spacing={1}>
            {progressData?.map((core) => {
              const coreTemp = 48 + Math.random() * 15;
              return (
                <Grid size={6} key={core.title}>
                  <Box mb={1}>
                    <Box
                      display="flex"
                      justifyContent="space-between"
                      alignItems="center"
                      mb={0.5}
                    >
                      <Typography variant="caption" fontWeight="medium">
                        {core.title}
                      </Typography>
                      <Typography variant="caption" fontWeight="medium">
                        {Math.round(core.value)}%
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={core.value}
                      sx={{ height: 4, borderRadius: 2, mb: 0.5 }}
                      color="primary"
                    />
                    <Typography variant="caption" color="success.main" sx={{ fontSize: '0.7rem' }}>
                      🌡️ {coreTemp.toFixed(1)}°C
                    </Typography>
                  </Box>
                </Grid>
              );
            })}
          </Grid>
        </>
      )}
    </BaseCard>
  );
};

export default CpuUsageCard;
