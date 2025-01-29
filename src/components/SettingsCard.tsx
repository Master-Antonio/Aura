import React, { useState } from "react";
import {
  Alert,
  Box,
  Chip,
  Collapse,
  Divider,
  FormControl,
  FormControlLabel,
  IconButton,
  InputLabel,
  MenuItem,
  Select,
  Slider,
  Switch,
  Typography,
} from "@mui/material";
import SettingsIcon from "@mui/icons-material/Settings";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";
import SpeedIcon from "@mui/icons-material/Speed";
import UpdateIcon from "@mui/icons-material/Update";
import VisibilityIcon from "@mui/icons-material/Visibility";
import BaseCard from "./BaseCard";

interface SettingsCardProps {
  refreshInterval: number;
  onRefreshIntervalChange: (interval: number) => void;
  updating: boolean;
  onToggleUpdating: () => void;
  processPerPage: number;
  onProcessPerPageChange: (perPage: number) => void;
}

const SettingsCard: React.FC<SettingsCardProps> = ({
  refreshInterval,
  onRefreshIntervalChange,
  updating,
  onToggleUpdating,
  processPerPage,
  onProcessPerPageChange,
}) => {
  const [expanded, setExpanded] = useState(false);
  const refreshIntervalOptions = [
    { value: 500, label: "0.5s (Very Fast)", color: "error" as const },
    { value: 1000, label: "1s (Fast)", color: "warning" as const },
    { value: 2000, label: "2s (Normal)", color: "primary" as const },
    { value: 5000, label: "5s (Slow)", color: "info" as const },
    { value: 10000, label: "10s (Very Slow)", color: "success" as const },
  ];
  const processPerPageOptions = [10, 25, 50, 100, 200];
  const getCurrentRefreshOption = () => {
    return (
      refreshIntervalOptions.find(
        (option) => option.value === refreshInterval,
      ) || refreshIntervalOptions[2]
    ); // Default to Normal
  };
  const handleRefreshIntervalChange = (event: any) => {
    const newInterval = event.target.value as number;
    onRefreshIntervalChange(newInterval);
  };
  const handleSliderChange = (_: Event, newValue: number | number[]) => {
    const value = Array.isArray(newValue) ? newValue[0] : newValue;
    onRefreshIntervalChange(value);
  };
  return (
    <BaseCard
      icon={SettingsIcon}
      title="Settings"
      headerActions={
        <Box display="flex" alignItems="center" gap={1}>
          <Chip
            label={getCurrentRefreshOption().label}
            size="small"
            color={getCurrentRefreshOption().color}
          />
          <IconButton onClick={() => setExpanded(!expanded)} size="small">
            {expanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
          </IconButton>
        </Box>
      }
    >
      {/* Quick Controls */}
      <Box
        display="flex"
        alignItems="center"
        justifyContent="space-between"
        mb={1}
      >
        <FormControlLabel
          control={
            <Switch
              checked={updating}
              onChange={onToggleUpdating}
              color="primary"
            />
          }
          label={
            <Box display="flex" alignItems="center" gap={0.5}>
              <UpdateIcon fontSize="small" />
              <Typography variant="body2">Real-time Updates</Typography>
            </Box>
          }
        />

        <Typography variant="body2" color="text.secondary">
          {updating ? "Active" : "Paused"}
        </Typography>
      </Box>

      {/* Expanded Settings */}
      <Collapse in={expanded}>
        <Divider sx={{ my: 2 }} />

        {/* Performance Warning */}
        {refreshInterval < 1000 && (
          <Alert severity="warning" sx={{ mb: 2 }}>
            Very fast refresh rates may impact system performance
          </Alert>
        )}

        {/* Refresh Interval Controls */}
        <Box mb={3}>
          <Box display="flex" alignItems="center" gap={1} mb={2}>
            <SpeedIcon fontSize="small" color="primary" />
            <Typography variant="subtitle2" fontWeight="bold">
              Refresh Interval
            </Typography>
          </Box>

          <FormControl fullWidth size="small" sx={{ mb: 2 }}>
            <InputLabel>Refresh Rate</InputLabel>
            <Select
              value={refreshInterval}
              onChange={handleRefreshIntervalChange}
              label="Refresh Rate"
            >
              {refreshIntervalOptions.map((option) => (
                <MenuItem key={option.value} value={option.value}>
                  <Box
                    display="flex"
                    alignItems="center"
                    justifyContent="space-between"
                    width="100%"
                  >
                    <span>{option.label}</span>
                    <Chip
                      size="small"
                      label={option.value + "ms"}
                      color={option.color}
                    />
                  </Box>
                </MenuItem>
              ))}
            </Select>
          </FormControl>

          <Typography variant="body2" color="text.secondary" gutterBottom>
            Custom Interval (ms)
          </Typography>
          <Box sx={{ px: 3, py: 1 }}>
            {" "}
            {/* Increased padding to prevent overlap */}
            <Slider
              value={refreshInterval}
              onChange={handleSliderChange}
              min={500}
              max={10000}
              step={500}
              marks={refreshIntervalOptions.map((opt) => ({
                value: opt.value,
                label: `${opt.value}ms`,
              }))}
              valueLabelDisplay="auto"
              valueLabelFormat={(value) => `${value}ms`}
              sx={{
                "& .MuiSlider-markLabel": {
                  fontSize: "0.6rem", // Even smaller font to prevent overlap
                  transform: "translateX(-50%)",
                  whiteSpace: "nowrap",
                  writingMode: "horizontal-tb",
                  marginTop: "8px",
                },
                "& .MuiSlider-mark": {
                  height: 8,
                },
                "& .MuiSlider-track": {
                  height: 6,
                },
                "& .MuiSlider-rail": {
                  height: 6,
                },
                marginBottom: "24px", // Extra space for labels
              }}
            />
          </Box>
        </Box>

        <Divider sx={{ my: 2 }} />

        {/* Process Table Settings */}
        <Box>
          {" "}
          <Box display="flex" alignItems="center" gap={1} mb={2}>
            <VisibilityIcon fontSize="small" color="primary" />
            <Typography variant="subtitle2" fontWeight="bold">
              Process Table
            </Typography>
          </Box>
          <FormControl fullWidth size="small" sx={{ mb: 2 }}>
            <InputLabel>Processes per Page</InputLabel>
            <Select
              value={processPerPage}
              onChange={(e) => onProcessPerPageChange(e.target.value as number)}
              label="Processes per Page"
            >
              {processPerPageOptions.map((option) => (
                <MenuItem key={option} value={option}>
                  {option} processes
                </MenuItem>
              ))}
            </Select>
          </FormControl>
          <Typography variant="body2" color="text.secondary" mt={1}>
            Higher values may impact performance on systems with many processes
          </Typography>
        </Box>

        {/* System Performance Info */}
        <Alert severity="info" sx={{ mt: 2 }}>
          <Typography variant="body2">
            <strong>Performance Tips:</strong>
            <br />
            • Use slower refresh rates for better battery life
            <br />
            • Reduce processes per page if experiencing lag
            <br />• Pause updates when not actively monitoring
          </Typography>
        </Alert>
      </Collapse>
    </BaseCard>
  );
};
export default SettingsCard;
