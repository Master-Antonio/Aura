import React, { useCallback, useEffect, useState } from "react";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Alert,
  Box,
  Button,
  Chip,
  CircularProgress,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  FormControlLabel,
  Grid,
  IconButton,
  Snackbar,
  Switch,
  Tooltip,
  Typography,
} from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import SportsEsportsIcon from "@mui/icons-material/SportsEsports";
import SettingsIcon from "@mui/icons-material/Settings";
import BoltIcon from "@mui/icons-material/Bolt";
import NetworkCheckIcon from "@mui/icons-material/NetworkCheck";
import SecurityIcon from "@mui/icons-material/Security";
import RefreshIcon from "@mui/icons-material/Refresh";
import AdminPanelSettingsIcon from "@mui/icons-material/AdminPanelSettings";
import InfoIcon from "@mui/icons-material/Info";
import TuneIcon from "@mui/icons-material/Tune";
import DeleteSweepIcon from "@mui/icons-material/DeleteSweep";
import MemoryIcon from "@mui/icons-material/Memory";
import { invoke } from "@tauri-apps/api/core";
import {
  OPTIMIZATION_CATEGORIES,
  OptimizationCategory,
  OptimizationItem,
  OptimizationResult,
  PlatformInfo,
} from "../data/OptimizationTypes";
import BaseCard from "./BaseCard";

const CategoryIcon = ({ categoryId }: { categoryId: string }) => {
  switch (categoryId) {
    case "gaming":
      return <SportsEsportsIcon />;
    case "system":
      return <SettingsIcon />;
    case "power":
      return <BoltIcon />;
    case "network":
      return <NetworkCheckIcon />;
    case "privacy":
      return <SecurityIcon />;
    default:
      return <SettingsIcon />;
  }
};

interface OptimizationsProps {
  refreshInterval?: number;
}

const Optimizations: React.FC<OptimizationsProps> = ({
  refreshInterval = 5000,
}) => {
  const [optimizations, setOptimizations] = useState<OptimizationItem[]>([]);
  const [platform, setPlatform] = useState<PlatformInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [applying, setApplying] = useState<string | null>(null);
  const [snackbar, setSnackbar] = useState({
    open: false,
    message: "",
    severity: "success" as "success" | "error",
  });
  const [confirmDialog, setConfirmDialog] = useState<{
    open: boolean;
    optimization: OptimizationItem | null;
  }>({
    open: false,
    optimization: null,
  });
  const fetchPlatform = useCallback(async () => {
    try {
      const platformInfo = await invoke<PlatformInfo>("get_current_platform");
      setPlatform(platformInfo);
    } catch (error) {
      console.error("Error fetching platform info:", error);
    }
  }, []);
  const fetchOptimizations = useCallback(async () => {
    try {
      const categories = await invoke<OptimizationCategory[]>(
        "get_available_optimizations",
      );
      // Convert backend categories to frontend format
      const allOptimizations: OptimizationItem[] = [];
      categories.forEach((category) => {
        category.items?.forEach((item: any) => {
          allOptimizations.push({
            id: item.id,
            name: item.name,
            description: item.description,
            category: category.name, // Use category name as string
            requires_admin: item.requires_admin,
            supported_platforms: [item.platform || "All"],
            applied: item.is_applied || false,
            is_reversible: item.is_reversible,
            risk_level: item.risk_level,
            platform: item.platform,
          });
        });
      });
      setOptimizations(allOptimizations);
    } catch (error) {
      console.error("Error fetching optimizations:", error);
      setSnackbar({
        open: true,
        message: "Failed to load optimizations",
        severity: "error",
      });
    } finally {
      setLoading(false);
    }
  }, []);
  const applyOptimization = async (optimization: OptimizationItem) => {
    if (optimization.requires_admin) {
      setConfirmDialog({ open: true, optimization });
      return;
    }
    await executeOptimization(optimization);
  };
  const executeOptimization = async (optimization: OptimizationItem) => {
    setApplying(optimization.id);
    try {
      const result = await invoke<OptimizationResult>("apply_optimization", {
        optimizationId: optimization.id,
      });
      if (result.success) {
        setSnackbar({
          open: true,
          message: `${optimization.name} ${optimization.applied ? "reverted" : "applied"} successfully`,
          severity: "success",
        });
        // Refresh optimizations to get updated state
        await fetchOptimizations();
      } else {
        setSnackbar({
          open: true,
          message: result.error || "Failed to apply optimization",
          severity: "error",
        });
      }
    } catch (error) {
      console.error("Error applying optimization:", error);
      setSnackbar({
        open: true,
        message: `Failed to ${optimization.applied ? "revert" : "apply"} ${optimization.name}`,
        severity: "error",
      });
    } finally {
      setApplying(null);
      setConfirmDialog({ open: false, optimization: null });
    }
  };
  const revertOptimization = async (optimization: OptimizationItem) => {
    setApplying(optimization.id);
    try {
      const result = await invoke<OptimizationResult>("revert_optimization", {
        optimizationId: optimization.id,
      });
      if (result.success) {
        setSnackbar({
          open: true,
          message: `${optimization.name} reverted successfully`,
          severity: "success",
        });
        await fetchOptimizations();
      } else {
        setSnackbar({
          open: true,
          message: result.error || "Failed to revert optimization",
          severity: "error",
        });
      }
    } catch (error) {
      console.error("Error reverting optimization:", error);
      setSnackbar({
        open: true,
        message: `Failed to revert ${optimization.name}`,
        severity: "error",
      });
    } finally {
      setApplying(null);
    }
  };
  const clearMemoryCache = async () => {
    setApplying("clear_memory_cache");
    try {
      const result = await invoke<OptimizationResult>("apply_optimization", {
        optimizationId: "clear_memory_cache",
      });
      if (result.success) {
        setSnackbar({
          open: true,
          message: "Memory cache cleared successfully",
          severity: "success",
        });
      } else {
        setSnackbar({
          open: true,
          message: result.error || "Failed to clear memory cache",
          severity: "error",
        });
      }
    } catch (error) {
      console.error("Error clearing memory cache:", error);
      setSnackbar({
        open: true,
        message: "Failed to clear memory cache",
        severity: "error",
      });
    } finally {
      setApplying(null);
    }
  };
  const clearDnsCache = async () => {
    setApplying("clear_dns_cache");
    try {
      const result = await invoke<OptimizationResult>("apply_optimization", {
        optimizationId: "clear_dns_cache",
      });
      if (result.success) {
        setSnackbar({
          open: true,
          message: "DNS cache cleared successfully",
          severity: "success",
        });
      } else {
        setSnackbar({
          open: true,
          message: result.error || "Failed to clear DNS cache",
          severity: "error",
        });
      }
    } catch (error) {
      console.error("Error clearing DNS cache:", error);
      setSnackbar({
        open: true,
        message: "Failed to clear DNS cache",
        severity: "error",
      });
    } finally {
      setApplying(null);
    }
  };
  useEffect(() => {
    fetchPlatform();
    fetchOptimizations();
    const interval = setInterval(fetchOptimizations, refreshInterval);
    return () => clearInterval(interval);
  }, [fetchPlatform, fetchOptimizations, refreshInterval]); // Group optimizations by category
  const groupedOptimizations = optimizations.reduce(
    (acc, opt) => {
      const categoryId = opt.category; // Now it's a string
      if (!acc[categoryId]) {
        acc[categoryId] = [];
      }
      acc[categoryId].push(opt);
      return acc;
    },
    {} as Record<string, OptimizationItem[]>,
  );
  if (loading) {
    return (
      <BaseCard icon={TuneIcon} title="System Optimizations">
        <Box
          display="flex"
          alignItems="center"
          justifyContent="center"
          minHeight={200}
        >
          <CircularProgress />
        </Box>
      </BaseCard>
    );
  }
  return (
    <>
      <BaseCard
        icon={TuneIcon}
        title="System Optimizations"
        headerActions={
          <Box display="flex" alignItems="center" gap={1}>
            {platform && (
              <Chip
                label={`${platform.os} ${platform.arch}`}
                size="small"
                variant="outlined"
              />
            )}
            <Tooltip title="Clear Memory Cache">
              <IconButton
                onClick={clearMemoryCache}
                size="small"
                disabled={applying === "clear_memory_cache"}
              >
                {applying === "clear_memory_cache" ? (
                  <CircularProgress size={16} />
                ) : (
                  <MemoryIcon />
                )}
              </IconButton>
            </Tooltip>
            <Tooltip title="Clear DNS Cache">
              <IconButton
                onClick={clearDnsCache}
                size="small"
                disabled={applying === "clear_dns_cache"}
              >
                {applying === "clear_dns_cache" ? (
                  <CircularProgress size={16} />
                ) : (
                  <DeleteSweepIcon />
                )}
              </IconButton>
            </Tooltip>
            <Tooltip title="Refresh optimizations">
              <IconButton onClick={fetchOptimizations} size="small">
                <RefreshIcon />
              </IconButton>
            </Tooltip>
          </Box>
        }
      >
        {/* Platform-specific notice */}{" "}
        {platform && (
          <Alert severity="info" sx={{ mb: 2 }}>
            Optimizations are dynamically loaded based on your {platform.os}{" "}
            system.
            {platform.os &&
              platform.os.toLowerCase().includes("windows") &&
              " Gaming optimizations and Windows-specific tweaks are available."}
          </Alert>
        )}
        {/* Optimization Categories */}
        {Object.keys(groupedOptimizations).length === 0 ? (
          <Typography
            variant="body2"
            color="text.secondary"
            textAlign="center"
            py={4}
          >
            No optimizations available for your platform
          </Typography>
        ) : (
          Object.entries(groupedOptimizations).map(
            ([categoryId, categoryOptimizations]) => {
              // Find matching predefined category or use default
              const category = Object.values(OPTIMIZATION_CATEGORIES).find(
                (cat) =>
                  cat.name === categoryId ||
                  cat.id === categoryId.toLowerCase(),
              );
              return (
                <Accordion key={categoryId} defaultExpanded={false}>
                  <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                    <Box display="flex" alignItems="center" gap={1}>
                      <CategoryIcon
                        categoryId={category?.id || categoryId.toLowerCase()}
                      />
                      <Typography variant="subtitle1" fontWeight="bold">
                        {category?.name || categoryId}
                      </Typography>
                      <Chip
                        label={categoryOptimizations.length}
                        size="small"
                        color="primary"
                      />
                    </Box>
                  </AccordionSummary>
                  <AccordionDetails>
                    {" "}
                    <Typography variant="body2" color="text.secondary" mb={2}>
                      {category?.description || `${categoryId} optimizations`}
                    </Typography>
                    <Grid container spacing={2}>
                      {categoryOptimizations.map((optimization) => (
                        <Grid size={12} key={optimization.id}>
                          <Box
                            sx={{
                              p: 2,
                              border: "1px solid",
                              borderColor: "divider",
                              borderRadius: 1,
                              backgroundColor: "background.paper",
                            }}
                          >
                            <Box
                              display="flex"
                              alignItems="center"
                              justifyContent="space-between"
                              mb={1}
                            >
                              <Box display="flex" alignItems="center" gap={1}>
                                <Typography
                                  variant="subtitle2"
                                  fontWeight="bold"
                                >
                                  {optimization.name}
                                </Typography>
                                {optimization.requires_admin && (
                                  <Tooltip title="Requires administrator privileges">
                                    <AdminPanelSettingsIcon
                                      fontSize="small"
                                      color="warning"
                                    />
                                  </Tooltip>
                                )}
                                <Tooltip title={optimization.description}>
                                  <InfoIcon fontSize="small" color="action" />
                                </Tooltip>
                              </Box>

                              <Box display="flex" alignItems="center" gap={1}>
                                {optimization.applied && (
                                  <Button
                                    size="small"
                                    variant="outlined"
                                    color="secondary"
                                    onClick={() =>
                                      revertOptimization(optimization)
                                    }
                                    disabled={applying === optimization.id}
                                  >
                                    {applying === optimization.id ? (
                                      <CircularProgress size={16} />
                                    ) : (
                                      "Revert"
                                    )}
                                  </Button>
                                )}

                                <FormControlLabel
                                  control={
                                    <Switch
                                      checked={optimization.applied}
                                      onChange={() =>
                                        applyOptimization(optimization)
                                      }
                                      disabled={applying === optimization.id}
                                    />
                                  }
                                  label=""
                                />
                              </Box>
                            </Box>

                            <Typography variant="body2" color="text.secondary">
                              {optimization.description}
                            </Typography>

                            <Box display="flex" gap={1} mt={1}>
                              {optimization.supported_platforms.map(
                                (platform) => (
                                  <Chip
                                    key={platform}
                                    label={platform}
                                    size="small"
                                    variant="outlined"
                                  />
                                ),
                              )}{" "}
                            </Box>
                          </Box>
                        </Grid>
                      ))}
                    </Grid>
                  </AccordionDetails>
                </Accordion>
              );
            },
          )
        )}
      </BaseCard>

      {/* Admin Confirmation Dialog */}
      <Dialog
        open={confirmDialog.open}
        onClose={() => setConfirmDialog({ open: false, optimization: null })}
      >
        <DialogTitle>Administrator Privileges Required</DialogTitle>
        <DialogContent>
          <DialogContentText>
            The optimization "{confirmDialog.optimization?.name}" requires
            administrator privileges to apply. This may modify system settings
            and could require a restart. Do you want to continue?
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button
            onClick={() =>
              setConfirmDialog({ open: false, optimization: null })
            }
          >
            Cancel
          </Button>
          <Button
            onClick={() =>
              confirmDialog.optimization &&
              executeOptimization(confirmDialog.optimization)
            }
            variant="contained"
            color="warning"
          >
            Apply with Admin Rights
          </Button>
        </DialogActions>
      </Dialog>

      {/* Snackbar for notifications */}
      <Snackbar
        open={snackbar.open}
        autoHideDuration={4000}
        onClose={() => setSnackbar({ ...snackbar, open: false })}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
      >
        <Alert
          severity={snackbar.severity}
          onClose={() => setSnackbar({ ...snackbar, open: false })}
        >
          {snackbar.message}
        </Alert>
      </Snackbar>
    </>
  );
};
export default Optimizations;
