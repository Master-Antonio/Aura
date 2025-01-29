import React, { useState } from "react";
import {
  Box,
  Chip,
  Divider,
  Grid,
  IconButton,
  LinearProgress,
  Typography,
  Tooltip,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { SvgIconComponent } from "@mui/icons-material";
import DownloadIcon from "@mui/icons-material/Download";
import UploadIcon from "@mui/icons-material/Upload";
import NetworkCheckIcon from "@mui/icons-material/NetworkCheck";
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

export interface NetworkCardProps {
  icon: SvgIconComponent;
  title: string;
  percentage?: number;
  progressData?: ProgressData[];
  genericData?: any;
}

const NetworkCard: React.FC<NetworkCardProps> = ({
  icon,
  progressData,
  genericData,
}) => {
  const [selectedInterface, setSelectedInterface] = useState(0);
  // Extract network interfaces from progressData (individual interfaces from backend)
  const allInterfaces = progressData || [];

  // If no interfaces found, create a default interface
  const networkInterfaces = allInterfaces.length > 0 ? allInterfaces : [{
    title: "Ethernet",
    value: 15, // Default activity level
    temperature: undefined
  }];

  const currentInterface = networkInterfaces[selectedInterface] || networkInterfaces[0];    // Extract real-time data from genericData and convert to KB/s
  const downloadSpeed = genericData?.find((item: any) => item.title === "Download Speed")?.value || "0 B/s";
  const uploadSpeed = genericData?.find((item: any) => item.title === "Upload Speed")?.value || "0 B/s";
  
  // Helper function to convert bytes to KB/s for display
  const formatSpeedInKB = (speedStr: string) => {
    const match = speedStr.match(/([\d.]+)\s*([KMGT]?B)/);
    if (!match) return "0 KB/s";
    
    const value = parseFloat(match[1]);
    const unit = match[2];
    
    let kbValue = value;
    switch (unit) {
      case 'GB': kbValue = value * 1024 * 1024; break;
      case 'MB': kbValue = value * 1024; break;
      case 'KB': kbValue = value; break;
      case 'B': kbValue = value / 1024; break;
      default: kbValue = value / 1024; break;
    }
    
    return `${kbValue.toFixed(1)} KB/s`;
  };
  
  // Calculate network usage based on current interface
  const calculateNetworkUsage = () => {
    // Use interface-specific value if available, otherwise calculate from speeds
    if (currentInterface?.value !== undefined) {
      return currentInterface.value;
    }
    
    const downMbps = parseFloat(downloadSpeed.replace(/[^\d.]/g, '')) || 0;
    const upMbps = parseFloat(uploadSpeed.replace(/[^\d.]/g, '')) || 0;
    const maxSpeed = 1000; // Assume Gigabit connection
    const totalSpeed = downMbps + upMbps;
    
    return Math.min((totalSpeed / maxSpeed) * 100, 100);
  };

  const overallUsage = calculateNetworkUsage();
  // Determine network interface specifications
  const getInterfaceSpecs = (interfaceTitle: string) => {
    let interfaceName = interfaceTitle;
    let interfaceType = "Ethernet";
    let connectionSpeed = "1 Gbps";
    let status = "Disconnected";
    
    // Parse interface title format: "InterfaceName (Type) - Speed Mbps [Status]"
    const interfaceMatch = interfaceTitle.match(/^(.+?)\s*\(([^)]+)\)(?:\s*-\s*(\d+)\s*Mbps)?(?:\s*\[([^\]]+)\])?/);
    if (interfaceMatch) {
      interfaceName = interfaceMatch[1].trim();
      interfaceType = interfaceMatch[2].trim();
      if (interfaceMatch[3]) {
        connectionSpeed = `${interfaceMatch[3]} Mbps`;
      }
      if (interfaceMatch[4]) {
        status = interfaceMatch[4].trim();
      }
    } else {
      // Fallback: determine type from name
      if (interfaceTitle.toLowerCase().includes("wi-fi") || interfaceTitle.toLowerCase().includes("wireless")) {
        interfaceType = "Wi-Fi";
        connectionSpeed = "300 Mbps";
        status = "Connected";
      } else if (interfaceTitle.toLowerCase().includes("bluetooth")) {
        interfaceType = "Bluetooth";
        connectionSpeed = "3 Mbps";
        status = "Paired";
      }
    }
    
    return { interfaceName, interfaceType, connectionSpeed, status };
  };

  const { interfaceName, interfaceType, connectionSpeed, status } = getInterfaceSpecs(currentInterface?.title || "");
  return (
    <BaseCard
      icon={icon}
      title="Network"
      headerActions={
        <Chip 
          label={interfaceType}
          size="small"
          color="info"
        />
      }
    >      {/* Interface Name */}
      <Typography
        variant="body2"
        color="text.secondary"
        mb={1}
        noWrap
        title={interfaceName || "Primary Interface"}
      >
        {interfaceName || "Primary Interface"}
      </Typography>
      
      {/* Connection Speed */}
      <Typography
        variant="caption"
        color="text.secondary"
        mb={2}
        display="block"
      >
        {connectionSpeed} • {status}
      </Typography>

      {/* Network Activity */}
      <Box mb={2}>
        <Box
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mb={1}
        >
          <Typography variant="body2" fontWeight="medium">
            Network Activity
          </Typography>
          <Typography variant="body2">
            {Math.round(overallUsage)}%
          </Typography>
        </Box>
        <LinearProgress
          variant="determinate"
          value={overallUsage}
          sx={{ height: 6, borderRadius: 3 }}
          color={overallUsage > 70 ? "warning" : "info"}
        />
      </Box>

      {/* Network Speed Info */}
      <Box mb={2}>
        <Typography variant="caption" color="text.secondary">
          Download: {downloadSpeed} • Upload: {uploadSpeed}
        </Typography>
      </Box>

      <Divider sx={{ my: 2 }} />

      {/* Metrics Grid */}
      <Grid container spacing={1}>
        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <DownloadIcon
                sx={{ fontSize: 16, mr: 0.5, color: "success.main" }}
              />
              <Typography variant="caption" color="text.secondary">
                Speed
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {connectionSpeed}
            </Typography>
          </MetricBox>
        </Grid>

        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <NetworkCheckIcon sx={{ fontSize: 16, mr: 0.5, color: "info.main" }} />
              <Typography variant="caption" color="text.secondary">
                Status
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold" color="success.main">
              {status}
            </Typography>
          </MetricBox>
        </Grid>        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <DownloadIcon sx={{ fontSize: 16, mr: 0.5, color: "success.main" }} />
              <Typography variant="caption" color="text.secondary">
                Received
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {formatSpeedInKB(downloadSpeed)}
            </Typography>
          </MetricBox>
        </Grid>

        <Grid size={6}>
          <MetricBox>
            <Box display="flex" alignItems="center" mb={0.5}>
              <UploadIcon
                sx={{ fontSize: 16, mr: 0.5, color: "secondary.main" }}
              />
              <Typography variant="caption" color="text.secondary">
                Transmitted
              </Typography>
            </Box>
            <Typography variant="body2" fontWeight="bold">
              {formatSpeedInKB(uploadSpeed)}
            </Typography>
          </MetricBox>
        </Grid>
      </Grid>

      {/* Navigation for multiple interfaces */}
      {networkInterfaces.length > 1 && (
        <>
          <Divider sx={{ my: 1.5 }} />
          <Box display="flex" justifyContent="center" alignItems="center">
            <Tooltip title="Previous Interface">
              <IconButton
                size="small"
                onClick={() => setSelectedInterface((prev) => Math.max(prev - 1, 0))}
                disabled={selectedInterface === 0}
              >
                <ChevronLeftIcon fontSize="small" />
              </IconButton>
            </Tooltip>
            <Typography variant="caption" color="text.secondary" mx={1}>
              Interface {selectedInterface + 1} of {networkInterfaces.length}
            </Typography>
            <Tooltip title="Next Interface">
              <IconButton
                size="small"
                onClick={() =>
                  setSelectedInterface((prev) =>
                    Math.min(prev + 1, networkInterfaces.length - 1),
                  )
                }
                disabled={selectedInterface === networkInterfaces.length - 1}
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

export default NetworkCard;
