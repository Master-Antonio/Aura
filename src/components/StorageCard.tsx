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
import StorageIcon from "@mui/icons-material/Storage";
import SpeedIcon from "@mui/icons-material/Speed";
import DevicesIcon from "@mui/icons-material/Devices";
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

export interface StorageCardProps {
  icon: SvgIconComponent;
  title: string;
  percentage?: number;
  progressData?: ProgressData[];
  genericData?: any;
}

const StorageCard: React.FC<StorageCardProps> = ({
  icon,
  percentage,
  progressData,
  genericData,
}) => {
  const [selectedDrive, setSelectedDrive] = useState(0);
  // Use progressData for drive navigation (from backend)
  const storageDrives =
    progressData && progressData.length > 0
      ? progressData
      : [
          {
            title: "System Drive (C:)",
            value: 50,
            temperature: 42.0,
          },
        ];
  const currentDrive = storageDrives[selectedDrive] || storageDrives[0];
  // Parse drive usage from progress data value
  const overallUsage = currentDrive?.value || percentage || 0;
  // Extract drive specifications from drive name and backend data
  const getDriveSpecs = (driveData: any) => {
    const driveName = driveData.title || "";
    // Find matching generic data for this drive
    const matchingGenericData = genericData?.find(
      (item: any) =>
        item.title.includes("Drive") || item.title.includes("Disk"),
    );
    // Extract drive letter and model from title (e.g., "C: - Samsung 980 PRO (1TB)")
    const driveMatch = driveName.match(
      /^([A-Z]:)\s*-\s*(.+?)(?:\s*\(([^)]+)\))?$/,
    );
    const driveLetter = driveMatch ? driveMatch[1] : "C:";
    const driveModel = driveMatch ? driveMatch[2] : "System Drive";
    const capacity = driveMatch && driveMatch[3] ? driveMatch[3] : "Unknown";
    // Parse additional info from generic data value if available
    let driveType = "SSD";
    let driveInterface = "SATA";
    let fileSystem = "NTFS";
    if (matchingGenericData?.value) {
      // Parse value string format: "used / total (percentage) | drive_type | interface"
      const valueMatch = matchingGenericData.value.match(
        /^([^|]*)\|\s*([^|]+)\|\s*(.+)$/,
      );
      if (valueMatch) {
        driveType = valueMatch[2].trim();
        driveInterface = valueMatch[3].trim();
      }
    }
    // Fallback: determine drive type based on typical patterns
    if (driveName.toLowerCase().includes("nvme")) {
      driveType = "NVMe SSD";
      driveInterface = "PCIe 4.0";
    } else if (driveName.toLowerCase().includes("ssd")) {
      driveType = "SATA SSD";
      driveInterface = "SATA III";
    } else if (driveLetter === "C:") {
      driveType = "NVMe SSD";
      driveInterface = "PCIe 4.0";
    }
    return {
      driveType,
      driveInterface,
      driveModel,
      fileSystem,
      capacity,
      driveLetter,
    };
  };
  const { driveType, driveInterface, driveModel, capacity } =
    getDriveSpecs(currentDrive);
  const driveTemp = currentDrive?.temperature || 42.5;
  return (
    <BaseCard
      icon={icon}
      title="Storage"
      headerActions={<Chip label={driveType} size="small" color="info" />}
    >
      {" "}
      {/* Drive Name */}
      <Typography
        variant="body2"
        color="text.secondary"
        mb={1}
        noWrap
        title={currentDrive?.title || "System Drive"}
      >
        {currentDrive?.title || "System Drive"}
      </Typography>
      {/* Drive Model */}
      <Typography
        variant="caption"
        color="text.secondary"
        mb={2}
        display="block"
      >
        {driveModel} • {capacity}
      </Typography>
      {/* Drive Usage */}
      <Box mb={2}>
        <Box
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mb={1}
        >
          <Typography variant="body2" fontWeight="medium">
            Storage Usage
          </Typography>
          <Typography variant="body2">{Math.round(overallUsage)}%</Typography>
        </Box>
        <LinearProgress
          variant="determinate"
          value={overallUsage}
          sx={{ height: 6, borderRadius: 3 }}
          color={
            overallUsage > 80 ? "error" : overallUsage > 60 ? "warning" : "info"
          }
        />
      </Box>{" "}
      {/* Storage Info */}
      <Box mb={2}>
        <Typography variant="caption" color="text.secondary">
          {genericData?.find((item: any) => item.title.includes("Total"))
            ?.value || "Total capacity information"}
        </Typography>
      </Box>
      <Divider sx={{ my: 2 }} />
      {/* Metrics Grid */}
      <Grid container spacing={1}>
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
              {driveTemp.toFixed(1)}°C
            </Typography>
          </MetricBox>
        </Grid>

        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <StorageIcon sx={{ fontSize: 16, mr: 0.5, color: "info.main" }} />
              <Typography variant="caption" color="text.secondary">
                Type
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {driveType}
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
                Interface
              </Typography>
            </Box>{" "}
            <Typography variant="body2" fontWeight="bold">
              {driveInterface}
            </Typography>
          </MetricBox>
        </Grid>

        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <DevicesIcon
                sx={{ fontSize: 16, mr: 0.5, color: "secondary.main" }}
              />
              <Typography variant="caption" color="text.secondary">
                Health
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold" color="success.main">
              Good
            </Typography>
          </MetricBox>
        </Grid>
      </Grid>
      {/* Navigation for multiple drives */}
      {storageDrives.length > 1 && (
        <>
          <Divider sx={{ my: 1.5 }} />
          <Box display="flex" justifyContent="center" alignItems="center">
            <Tooltip title="Previous Drive">
              <IconButton
                size="small"
                onClick={() =>
                  setSelectedDrive((prev) => Math.max(prev - 1, 0))
                }
                disabled={selectedDrive === 0}
              >
                <ChevronLeftIcon fontSize="small" />
              </IconButton>
            </Tooltip>
            <Typography variant="caption" color="text.secondary" mx={1}>
              Drive {selectedDrive + 1} of {storageDrives.length}
            </Typography>
            <Tooltip title="Next Drive">
              <IconButton
                size="small"
                onClick={() =>
                  setSelectedDrive((prev) =>
                    Math.min(prev + 1, storageDrives.length - 1),
                  )
                }
                disabled={selectedDrive === storageDrives.length - 1}
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
export default StorageCard;
