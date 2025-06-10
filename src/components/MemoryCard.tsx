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
import { SvgIconComponent } from "@mui/icons-material";
import ThermostatIcon from "@mui/icons-material/Thermostat";
import SpeedIcon from "@mui/icons-material/Speed";
import MemoryIcon from "@mui/icons-material/Memory";
import StorageIcon from "@mui/icons-material/Storage";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import { ProgressData } from "../data/SystemStats";
import BaseCard from "./BaseCard";

const MetricBox = styled(Box)(({ theme }) => ({
  padding: theme.spacing(1),
  borderRadius: theme.shape.borderRadius,
  backgroundColor: theme.palette.background.default,
  border: `1px solid ${theme.palette.divider}`,
}));

export interface MemoryCardProps {
  icon: SvgIconComponent;
  title: string;
  percentage?: number;
  progressData?: ProgressData[];
  genericData?: any;
}

const MemoryCard: React.FC<MemoryCardProps> = ({
  icon,
  percentage,
  genericData,
}) => {
  const [selectedModule, setSelectedModule] = useState(0);
  // Extract memory modules from genericData
  const memoryModules =
    genericData?.filter(
      (data: any) =>
        data.title.includes("Slot") ||
        data.title.includes("Module") ||
        data.title.includes("DIMM"),
    ) || [];
  // If no modules found, create a default overall memory entry
  if (memoryModules.length === 0) {
    const totalMemory =
      genericData?.find((data: any) => data.title === "Total Memory")?.value ||
      "0 GB";
    const usedMemory =
      genericData?.find((data: any) => data.title === "Used Memory")?.value ||
      "0 GB";
    memoryModules.push({
      title: "System Memory",
      value: `${usedMemory} / ${totalMemory}`,
      manufacturer: "System",
      speed: "N/A",
      type: "DDR4",
    });
  }
  const currentModule = memoryModules[selectedModule] || memoryModules[0];
  const overallUsage = percentage || 0;
  // Extract memory specifications
  const getMemorySpecs = (moduleValue: string) => {
    // Parse memory module info (e.g., "16 GB @ 3200 MHz - Kingston HyperX")
    const parts = moduleValue.split(" - ");
    const capacityPart = parts[0] || moduleValue;
    const brandPart = parts[1] || "Unknown";
    const capacity = capacityPart.match(/\d+\s*GB/)?.[0] || "N/A";
    const speed = capacityPart.match(/\d+\s*MHz/)?.[0] || "N/A";
    return { capacity, speed, brand: brandPart };
  };
  const { capacity, speed, brand } = getMemorySpecs(currentModule?.value || "");
  const memoryTemp = 45.2; // Simulated temperature
  // Determine memory type
  const getMemoryType = () => {
    if (currentModule?.value?.includes("DDR5")) return "DDR5";
    if (currentModule?.value?.includes("DDR4")) return "DDR4";
    if (currentModule?.value?.includes("DDR3")) return "DDR3";
    return "DDR4"; // Default
  };
  const memoryType = getMemoryType();
  return (
    <BaseCard
      icon={icon}
      title="Memory"
      headerActions={<Chip label={memoryType} size="small" color="primary" />}
    >
      {/* Memory Module Name */}
      <Typography
        variant="body2"
        color="text.secondary"
        mb={2}
        noWrap
        title={currentModule?.title || "System Memory"}
      >
        {currentModule?.title || "System Memory"}
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
            Memory Usage
          </Typography>
          <Typography variant="body2">{Math.round(overallUsage)}%</Typography>
        </Box>
        <LinearProgress
          variant="determinate"
          value={overallUsage}
          sx={{ height: 6, borderRadius: 3 }}
          color={overallUsage > 80 ? "warning" : "primary"}
        />
      </Box>

      <Divider sx={{ my: 2 }} />

      {/* Metrics Grid */}
      <Grid container spacing={1}>
        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <StorageIcon sx={{ fontSize: 16, mr: 0.5, color: "info.main" }} />
              <Typography variant="caption" color="text.secondary">
                Capacity
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {capacity}
            </Typography>
          </MetricBox>
        </Grid>

        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <SpeedIcon
                sx={{ fontSize: 16, mr: 0.5, color: "success.main" }}
              />
              <Typography variant="caption" color="text.secondary">
                Speed
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {speed}
            </Typography>
          </MetricBox>
        </Grid>

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
              {memoryTemp}°C
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
                Brand
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold" noWrap>
              {brand}
            </Typography>
          </MetricBox>
        </Grid>
      </Grid>

      {/* Navigation for multiple memory modules */}
      {memoryModules.length > 1 && (
        <>
          <Divider sx={{ my: 1.5 }} />
          <Box display="flex" justifyContent="center" alignItems="center">
            <Tooltip title="Previous Module">
              <IconButton
                size="small"
                onClick={() =>
                  setSelectedModule((prev) => Math.max(prev - 1, 0))
                }
                disabled={selectedModule === 0}
              >
                <ChevronLeftIcon fontSize="small" />
              </IconButton>
            </Tooltip>
            <Typography variant="caption" color="text.secondary" mx={1}>
              Module {selectedModule + 1} of {memoryModules.length}
            </Typography>
            <Tooltip title="Next Module">
              <IconButton
                size="small"
                onClick={() =>
                  setSelectedModule((prev) =>
                    Math.min(prev + 1, memoryModules.length - 1),
                  )
                }
                disabled={selectedModule === memoryModules.length - 1}
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
export default MemoryCard;
