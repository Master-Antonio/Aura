import {
  Box,
  Card,
  CardContent,
  Chip,
  CircularProgress,
  Divider,
  IconButton,
  Modal,
  Stack,
  Tooltip,
  Typography,
} from "@mui/material";
import Grid from "@mui/material/Grid";
import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import MonitorIcon from "@mui/icons-material/Monitor";
import InfoIcon from "@mui/icons-material/Info";
import AccountTreeIcon from "@mui/icons-material/AccountTree";
import CloseIcon from "@mui/icons-material/Close";
import PauseIcon from "@mui/icons-material/Pause";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";
import TuneIcon from "@mui/icons-material/Tune";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";

interface ProcessInfo {
  pid: number;
  parent_pid: number;
  name: string;
  exe_path: string;
  cpu_usage_percent: number;
  memory_working_set: number; // in MB
  memory_private: number; // in MB
  memory_virtual: number; // in MB
  memory_pagefile: number; // in MB
  handle_count: number;
  thread_count: number;
  is_suspended: boolean;
  session_id: number;
  io_read_bytes: string;
  io_write_bytes: string;
  io_read_operations: number;
  io_write_operations: number;
  run_time: string;
  children: ChildProcess[];
}

interface ChildProcess {
  pid: number;
  name: string;
  cpu_usage_percent: number;
  memory_working_set: number; // in MB
  is_suspended: boolean;
}

interface ProcessData {
  name: string;
  pid: number;
  cpu_usage: number;
  exe_path: string;
  affinity_set: boolean;
  ram_usage: number;
  run_time: string;
  status: string;
  disk_usage: DiskUsage;
}

interface DiskUsage {
  read: string;
  write: string;
}

interface Props {
  open: boolean;
  onClose: () => void;
  process: ProcessData;
  onAffinityChange: (pid: number, cpuIndex: number) => Promise<void>;
}

const style = {
  position: "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  width: "90%",
  maxWidth: "900px",
  maxHeight: "90vh",
  bgcolor: "#1c1d2a",
  borderRadius: 3,
  border: "1px solid rgba(255, 255, 255, 0.1)",
  boxShadow: "0 20px 40px rgba(0, 0, 0, 0.3)",
  p: 3,
  overflow: "auto",
};
const ModalProcessInfo = ({ open, onClose, process }: Props) => {
  const [processInfo, setProcessInfo] = useState<ProcessInfo>({
    pid: 0,
    parent_pid: 0,
    name: "",
    exe_path: "",
    cpu_usage_percent: 0,
    memory_working_set: 0,
    memory_private: 0,
    memory_virtual: 0,
    memory_pagefile: 0,
    handle_count: 0,
    thread_count: 0,
    is_suspended: false,
    session_id: 0,
    io_read_bytes: "",
    io_write_bytes: "",
    io_read_operations: 0,
    io_write_operations: 0,
    run_time: "",
    children: [],
  });
  const [currentPid, setCurrentPid] = useState<number>(0);
  const [cpuCoreCount, setCpuCoreCount] = useState<number>(8);
  const [currentAffinity, setCurrentAffinity] = useState<number[]>([]);
  const [selectedCores, setSelectedCores] = useState<number[]>([]);
  const [applyingAffinity, setApplyingAffinity] = useState(false);
  const [affinityAppliedSuccessfully, setAffinityAppliedSuccessfully] =
    useState(false);
  const [affinityApplyError, setAffinityApplyError] = useState(false);
  const fetchProcessInfo = async (pid: number) => {
    try {
      const result = await invoke("get_detailed_process_info", { pid });
      setProcessInfo(result as ProcessInfo);
    } catch (error) {
      console.error("Failed to fetch process info:", error);
    }
  };
  const fetchCpuAffinityInfo = async (
    pid: number,
    preserveSelection = false,
  ) => {
    try {
      // Get system CPU core count
      const coreCount = (await invoke<number>("get_cpu_core_count")) || 8;
      setCpuCoreCount(coreCount);
      // Get current process affinity
      const affinity =
        (await invoke<number[]>("get_process_affinity", { pid })) || [];
      setCurrentAffinity(affinity);
      // Only update selected cores if we're not preserving the current selection
      if (!preserveSelection) {
        setSelectedCores(affinity.length > 0 ? affinity : [0]); // Default to core 0 if no affinity
      }
    } catch (error) {
      console.error("Failed to fetch CPU affinity info:", error);
      // Set defaults
      setCpuCoreCount(8);
      setCurrentAffinity([]);
      if (!preserveSelection) {
        setSelectedCores([0]); // Default to core 0
      }
    }
  };
  const handleCoreToggle = (coreIndex: number) => {
    setSelectedCores((prev) => {
      if (prev.includes(coreIndex)) {
        return prev.filter((core) => core !== coreIndex);
      } else {
        return [...prev, coreIndex];
      }
    });
  };
  const handleApplyAffinity = async () => {
    if (selectedCores.length === 0) {
      return; // Don't allow setting no cores
    }
    setApplyingAffinity(true);
    setAffinityAppliedSuccessfully(false);
    setAffinityApplyError(false);
    try {
      await invoke("set_process_affinity", {
        pid: processInfo.pid,
        cores: selectedCores,
      });
      // Refresh affinity info without preserving selection (to get the new applied state)
      await fetchCpuAffinityInfo(processInfo.pid, false);
      setAffinityAppliedSuccessfully(true);
      // Reset success state after 3 seconds
      setTimeout(() => {
        setAffinityAppliedSuccessfully(false);
      }, 3000);
    } catch (error) {
      console.error("Failed to set process affinity:", error);
      setAffinityApplyError(true);
      // Reset error state after 3 seconds
      setTimeout(() => {
        setAffinityApplyError(false);
      }, 3000);
    } finally {
      setApplyingAffinity(false);
    }
  };
  const handleSelectCore0Only = () => {
    // Toggle logic: if only core 0 is selected, select all cores; otherwise select only core 0
    if (selectedCores.length === 1 && selectedCores[0] === 0) {
      // Currently only core 0 is selected, select all cores
      setSelectedCores(Array.from({ length: cpuCoreCount }, (_, i) => i));
    } else {
      // Select only core 0
      setSelectedCores([0]);
    }
  };
  const handleParentPidClick = (parentPid: number) => {
    if (parentPid > 0) {
      setCurrentPid(parentPid);
    }
  };
  const handleChildPidClick = (childPid: number) => {
    if (childPid > 0) {
      setCurrentPid(childPid);
    }
  };
  // Action handlers
  const handleKillProcess = useCallback(
    async (pid: number) => {
      try {
        await invoke("kill_process", { pid });
        onClose(); // Close modal after killing process
      } catch (error) {
        console.error("Failed to kill process:", error);
      }
    },
    [onClose],
  );
  const handleSuspendProcess = useCallback(async (pid: number) => {
    try {
      await invoke("suspend_process", { pid });
      // Refresh process info to get updated status
      await fetchProcessInfo(pid);
    } catch (error) {
      console.error("Failed to suspend process:", error);
    }
  }, []);
  const handleResumeProcess = useCallback(async (pid: number) => {
    try {
      await invoke("resume_process", { pid });
      // Refresh process info to get updated status
      await fetchProcessInfo(pid);
    } catch (error) {
      console.error("Failed to resume process:", error);
    }
  }, []);
  const handleOpenFileLocation = useCallback(async (exePath: string) => {
    try {
      await invoke("open_file_location", { path: exePath });
    } catch (error) {
      console.error("Failed to open file location:", error);
    }
  }, []);
  useEffect(() => {
    let intervalId: number;
    if (process && open) {
      // Initialize currentPid when modal opens for the first time
      if (currentPid === 0) {
        setCurrentPid(process.pid);
        return; // Let the next useEffect call handle the fetching
      }
      // Fetch immediately when PID changes
      fetchProcessInfo(currentPid);
      // Fetch CPU affinity info only on initial load (not preserving selection)
      fetchCpuAffinityInfo(currentPid, false);
      // Set up periodic updates (only process info, preserve affinity selection)
      intervalId = window.setInterval(() => {
        fetchProcessInfo(currentPid);
        fetchCpuAffinityInfo(currentPid, true); // Preserve selection during updates
      }, 1000);
    } else {
      // Reset currentPid when modal closes
      setCurrentPid(0);
      // Reset affinity selection when modal closes
      setSelectedCores([]);
      setCurrentAffinity([]);
    }
    // Cleanup interval when component unmounts or modal closes
    return () => {
      if (intervalId) {
        clearInterval(intervalId);
      }
    };
  }, [process, open, currentPid]);
  return (
    <Modal
      keepMounted
      open={open}
      onClose={onClose}
      sx={{
        "& .MuiBackdrop-root": {
          backgroundColor: "rgba(0, 0, 0, 0.8)",
          backdropFilter: "blur(4px)",
        },
      }}
    >
      <Box sx={{ ...style }}>
        <Box
          sx={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            mb: 2,
          }}
        >
          <Typography
            variant="h5"
            component="div"
            fontWeight="bold"
            sx={{
              color: "white",
              display: "flex",
              alignItems: "center",
              gap: 1,
            }}
          >
            <InfoIcon color="primary" />
            {processInfo.name} - Process Information
          </Typography>

          {/* Action buttons */}
          <Box sx={{ display: "flex", gap: 1 }}>
            {processInfo.is_suspended ? (
              <Tooltip title="Resume Process">
                <IconButton
                  size="small"
                  onClick={() => handleResumeProcess(processInfo.pid)}
                  sx={{
                    color: "#34d399",
                    backgroundColor: "rgba(52, 211, 153, 0.1)",
                    "&:hover": {
                      backgroundColor: "rgba(52, 211, 153, 0.2)",
                    },
                  }}
                >
                  <PlayArrowIcon fontSize="small" />
                </IconButton>
              </Tooltip>
            ) : (
              <Tooltip title="Suspend Process">
                <IconButton
                  size="small"
                  onClick={() => handleSuspendProcess(processInfo.pid)}
                  sx={{
                    color: "#fbbf24",
                    backgroundColor: "rgba(251, 191, 36, 0.1)",
                    "&:hover": {
                      backgroundColor: "rgba(251, 191, 36, 0.2)",
                    },
                  }}
                >
                  <PauseIcon fontSize="small" />
                </IconButton>
              </Tooltip>
            )}

            <Tooltip title="Open File Location">
              <IconButton
                size="small"
                onClick={() => handleOpenFileLocation(processInfo.exe_path)}
                disabled={
                  !processInfo.exe_path || processInfo.exe_path === "N/A"
                }
                sx={{
                  color: "#a78bfa",
                  backgroundColor: "rgba(167, 139, 250, 0.1)",
                  "&:hover": {
                    backgroundColor: "rgba(167, 139, 250, 0.2)",
                  },
                  "&:disabled": {
                    color: "rgba(167, 139, 250, 0.3)",
                    backgroundColor: "rgba(167, 139, 250, 0.05)",
                  },
                }}
              >
                <FolderOpenIcon fontSize="small" />
              </IconButton>
            </Tooltip>

            <Tooltip title="Kill Process">
              <IconButton
                size="small"
                onClick={() => handleKillProcess(processInfo.pid)}
                sx={{
                  color: "#f87171",
                  backgroundColor: "rgba(248, 113, 113, 0.1)",
                  "&:hover": {
                    backgroundColor: "rgba(248, 113, 113, 0.2)",
                  },
                }}
              >
                <CloseIcon fontSize="small" />
              </IconButton>
            </Tooltip>
          </Box>
        </Box>
        <Divider sx={{ mb: 3, borderColor: "rgba(255, 255, 255, 0.1)" }} />{" "}
        <Grid container spacing={3}>
          <Grid size={12}>
            <Card
              sx={{
                backgroundColor: "#0a0b11",
                borderRadius: 3,
                border: "1px solid rgba(255, 255, 255, 0.1)",
                boxShadow: "0 4px 12px rgba(0, 0, 0, 0.2)",
              }}
            >
              <CardContent sx={{ p: 3 }}>
                <Grid container spacing={3}>
                  <Grid size={3}>
                    <Typography
                      variant="subtitle2"
                      sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 1 }}
                    >
                      PID
                    </Typography>
                    <Typography
                      variant="h5"
                      component="div"
                      fontWeight="bold"
                      sx={{ color: "white" }}
                    >
                      {processInfo.pid}
                    </Typography>
                  </Grid>
                  <Grid size={3}>
                    <Typography
                      variant="subtitle2"
                      sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 1 }}
                    >
                      Status
                    </Typography>
                    <Typography
                      variant="h6"
                      component="div"
                      fontWeight="bold"
                      sx={{
                        color: processInfo.is_suspended ? "#f87171" : "#4ade80",
                        textTransform: "capitalize",
                      }}
                    >
                      {processInfo.is_suspended ? "Suspended" : "Running"}
                    </Typography>
                  </Grid>
                  <Grid size={3}>
                    <Typography
                      variant="subtitle2"
                      sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 1 }}
                    >
                      CPU Usage
                    </Typography>
                    <Typography
                      variant="h6"
                      component="div"
                      fontWeight="bold"
                      sx={{
                        color:
                          processInfo.cpu_usage_percent > 50
                            ? "#f87171"
                            : processInfo.cpu_usage_percent > 25
                              ? "#fbbf24"
                              : "#4ade80",
                      }}
                    >
                      {processInfo.cpu_usage_percent.toFixed(1)}%
                    </Typography>
                  </Grid>
                  <Grid size={3}>
                    <Typography
                      variant="subtitle2"
                      sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 1 }}
                    >
                      Memory Usage
                    </Typography>
                    <Typography
                      variant="h6"
                      component="div"
                      fontWeight="bold"
                      sx={{ color: "white" }}
                    >
                      {processInfo.memory_working_set} MB
                    </Typography>
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          </Grid>{" "}
          <Grid size={6}>
            <Card
              sx={{
                backgroundColor: "#0a0b11",
                borderRadius: 3,
                border: "1px solid rgba(255, 255, 255, 0.1)",
                boxShadow: "0 4px 12px rgba(0, 0, 0, 0.2)",
                height: "100%",
              }}
            >
              <CardContent sx={{ p: 0 }}>
                <Stack
                  direction="row"
                  spacing={1}
                  alignItems="center"
                  sx={{
                    backgroundColor: "#1c1d2a",
                    p: 2.5,
                    borderTopLeftRadius: "inherit",
                    borderTopRightRadius: "inherit",
                    borderBottom: "1px solid rgba(255, 255, 255, 0.1)",
                  }}
                >
                  <InfoIcon sx={{ color: "#60a5fa" }} fontSize="small" />
                  <Typography
                    variant="subtitle1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white" }}
                  >
                    Process Information
                  </Typography>
                </Stack>
                <Box sx={{ p: 2.5 }}>
                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    Process Name
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white", mb: 2 }}
                  >
                    {processInfo.name}
                  </Typography>

                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    Executable Path
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    sx={{
                      color: "white",
                      mb: 2,
                      fontSize: "0.875rem",
                      wordBreak: "break-all",
                    }}
                  >
                    {processInfo.exe_path || "N/A"}
                  </Typography>

                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    Parent PID
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    onClick={() => handleParentPidClick(processInfo.parent_pid)}
                    sx={{
                      color: "#60a5fa",
                      cursor: "pointer",
                      mb: 2,
                      "&:hover": {
                        color: "#93c5fd",
                        textDecoration: "underline",
                      },
                    }}
                  >
                    {processInfo.parent_pid}
                  </Typography>

                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    Session ID
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white" }}
                  >
                    {processInfo.session_id || "N/A"}
                  </Typography>
                </Box>
              </CardContent>
            </Card>
          </Grid>{" "}
          <Grid size={6}>
            <Card
              sx={{
                backgroundColor: "#0a0b11",
                borderRadius: 3,
                border: "1px solid rgba(255, 255, 255, 0.1)",
                boxShadow: "0 4px 12px rgba(0, 0, 0, 0.2)",
                height: "100%",
              }}
            >
              <CardContent sx={{ p: 0 }}>
                <Stack
                  direction="row"
                  spacing={1}
                  alignItems="center"
                  sx={{
                    backgroundColor: "#1c1d2a",
                    p: 2.5,
                    borderTopLeftRadius: "inherit",
                    borderTopRightRadius: "inherit",
                    borderBottom: "1px solid rgba(255, 255, 255, 0.1)",
                  }}
                >
                  <MonitorIcon sx={{ color: "#34d399" }} fontSize="small" />
                  <Typography
                    variant="subtitle1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white" }}
                  >
                    Resource Usage
                  </Typography>
                </Stack>
                <Box sx={{ p: 2.5 }}>
                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    CPU Usage
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    sx={{
                      color:
                        processInfo.cpu_usage_percent > 50
                          ? "#f87171"
                          : processInfo.cpu_usage_percent > 25
                            ? "#fbbf24"
                            : "#4ade80",
                      mb: 2,
                    }}
                  >
                    {processInfo.cpu_usage_percent.toFixed(1)}%
                  </Typography>

                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    Working Set Memory
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white", mb: 2 }}
                  >
                    {processInfo.memory_working_set.toFixed(1)} MB
                  </Typography>

                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    Private Memory
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white", mb: 2 }}
                  >
                    {processInfo.memory_private.toFixed(1)} MB
                  </Typography>

                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    Virtual Memory
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white", mb: 2 }}
                  >
                    {processInfo.memory_virtual.toFixed(1)} MB
                  </Typography>

                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 0.5 }}
                  >
                    Pagefile Memory
                  </Typography>
                  <Typography
                    variant="body1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white" }}
                  >
                    {processInfo.memory_pagefile.toFixed(1)} MB
                  </Typography>
                </Box>
              </CardContent>
            </Card>
          </Grid>{" "}
          <Grid size={6}>
            <Card
              sx={{
                backgroundColor: "#0a0b11",
                borderRadius: 3,
                border: "1px solid rgba(255, 255, 255, 0.1)",
                boxShadow: "0 4px 12px rgba(0, 0, 0, 0.2)",
                height: "100%",
              }}
            >
              <CardContent sx={{ p: 0 }}>
                <Stack
                  direction="row"
                  spacing={1}
                  alignItems="center"
                  sx={{
                    backgroundColor: "#1c1d2a",
                    p: 2.5,
                    borderTopLeftRadius: "inherit",
                    borderTopRightRadius: "inherit",
                    borderBottom: "1px solid rgba(255, 255, 255, 0.1)",
                  }}
                >
                  <AccountTreeIcon sx={{ color: "#a78bfa" }} fontSize="small" />
                  <Typography
                    variant="subtitle1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white" }}
                  >
                    Child Processes
                  </Typography>
                </Stack>
                <Box sx={{ p: 2.5, maxHeight: "200px", overflowY: "auto" }}>
                  {Array.isArray(processInfo.children) &&
                  processInfo.children.length > 0 ? (
                    processInfo.children.map(
                      (childProcess: ChildProcess, index: number) => (
                        <Box
                          key={childProcess.pid}
                          sx={{
                            mb: index < processInfo.children.length - 1 ? 2 : 0,
                            p: 1.5,
                            backgroundColor: "rgba(255, 255, 255, 0.05)",
                            borderRadius: 2,
                            border: "1px solid rgba(255, 255, 255, 0.1)",
                          }}
                        >
                          <Typography
                            onClick={() =>
                              handleChildPidClick(childProcess.pid)
                            }
                            sx={{
                              color: "#60a5fa",
                              cursor: "pointer",
                              fontSize: "0.875rem",
                              fontWeight: "bold",
                              mb: 0.5,
                              "&:hover": {
                                color: "#93c5fd",
                                textDecoration: "underline",
                              },
                            }}
                          >
                            PID: {childProcess.pid}
                          </Typography>
                          <Typography
                            variant="body2"
                            component="div"
                            sx={{ color: "white", mb: 0.5 }}
                          >
                            {childProcess.name}
                          </Typography>
                          <Typography
                            variant="caption"
                            component="div"
                            sx={{ color: "rgba(255, 255, 255, 0.7)" }}
                          >
                            CPU: {childProcess.cpu_usage_percent.toFixed(1)}% |
                            Memory: {childProcess.memory_working_set.toFixed(1)}{" "}
                            MB | Status:{" "}
                            {childProcess.is_suspended
                              ? "Suspended"
                              : "Running"}
                          </Typography>
                        </Box>
                      ),
                    )
                  ) : (
                    <Typography
                      variant="body2"
                      component="div"
                      sx={{
                        color: "rgba(255, 255, 255, 0.7)",
                        textAlign: "center",
                        fontStyle: "italic",
                        py: 2,
                      }}
                    >
                      No child processes
                    </Typography>
                  )}
                </Box>
              </CardContent>
            </Card>
          </Grid>
          {/* CPU Affinity Card */}
          <Grid size={6}>
            <Card
              sx={{
                backgroundColor: "#0a0b11",
                borderRadius: 3,
                border: "1px solid rgba(255, 255, 255, 0.1)",
                boxShadow: "0 4px 12px rgba(0, 0, 0, 0.2)",
                height: "100%",
              }}
            >
              <CardContent sx={{ p: 0 }}>
                <Stack
                  direction="row"
                  spacing={1}
                  alignItems="center"
                  sx={{
                    backgroundColor: "#1c1d2a",
                    p: 2.5,
                    borderTopLeftRadius: "inherit",
                    borderTopRightRadius: "inherit",
                    borderBottom: "1px solid rgba(255, 255, 255, 0.1)",
                  }}
                >
                  <TuneIcon sx={{ color: "#f59e0b" }} fontSize="small" />
                  <Typography
                    variant="subtitle1"
                    component="div"
                    fontWeight="bold"
                    sx={{ color: "white" }}
                  >
                    CPU Affinity
                  </Typography>
                  {affinityAppliedSuccessfully && (
                    <CheckCircleIcon
                      sx={{ color: "#10b981", ml: "auto" }}
                      fontSize="small"
                    />
                  )}
                  {affinityApplyError && (
                    <CloseIcon
                      sx={{ color: "#ef4444", ml: "auto" }}
                      fontSize="small"
                    />
                  )}
                </Stack>
                <Box sx={{ p: 2.5 }}>
                  <Typography
                    variant="subtitle2"
                    sx={{ color: "rgba(255, 255, 255, 0.7)", mb: 1.5 }}
                  >
                    Select CPU cores for this process:
                  </Typography>

                  <Box
                    sx={{ display: "flex", flexWrap: "wrap", gap: 1, mb: 2.5 }}
                  >
                    {Array.from({ length: cpuCoreCount }, (_, index) => (
                      <Chip
                        key={index}
                        label={`Core ${index}`}
                        variant={
                          selectedCores.includes(index) ? "filled" : "outlined"
                        }
                        color={
                          selectedCores.includes(index)
                            ? "primary"
                            : currentAffinity.includes(index)
                              ? "success"
                              : "default"
                        }
                        onClick={() => handleCoreToggle(index)}
                        clickable
                        sx={{
                          minWidth: "70px",
                          fontSize: "0.75rem",
                          height: "28px",
                          "& .MuiChip-label": {
                            px: 1.5,
                          },
                          ...(currentAffinity.includes(index) &&
                            !selectedCores.includes(index) && {
                              borderColor: "#10b981",
                              color: "#10b981",
                            }),
                        }}
                      />
                    ))}
                  </Box>

                  <Box
                    sx={{
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "space-between",
                      mb: 2,
                    }}
                  >
                    <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                      <Tooltip
                        title={
                          selectedCores.length === 1 && selectedCores[0] === 0
                            ? "Select all cores"
                            : "Select only Core 0"
                        }
                      >
                        <IconButton
                          onClick={handleSelectCore0Only}
                          size="small"
                          sx={{
                            color: "#60a5fa",
                            backgroundColor: "rgba(96, 165, 250, 0.1)",
                            "&:hover": {
                              backgroundColor: "rgba(96, 165, 250, 0.2)",
                            },
                          }}
                        >
                          <TuneIcon fontSize="small" />
                        </IconButton>
                      </Tooltip>

                      <Typography
                        variant="caption"
                        sx={{ color: "rgba(255, 255, 255, 0.6)" }}
                      >
                        {selectedCores.length > 0
                          ? `${selectedCores.length} core${selectedCores.length > 1 ? "s" : ""} selected`
                          : "Select at least one core"}
                      </Typography>
                    </Box>

                    <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                      <IconButton
                        onClick={handleApplyAffinity}
                        disabled={
                          selectedCores.length === 0 || applyingAffinity
                        }
                        size="small"
                        sx={{
                          color:
                            selectedCores.length === 0 || applyingAffinity
                              ? "rgba(96, 165, 250, 0.3)"
                              : "#60a5fa",
                          backgroundColor:
                            selectedCores.length === 0 || applyingAffinity
                              ? "rgba(96, 165, 250, 0.05)"
                              : "rgba(96, 165, 250, 0.1)",
                          "&:hover": {
                            backgroundColor:
                              selectedCores.length === 0 || applyingAffinity
                                ? "rgba(96, 165, 250, 0.05)"
                                : "rgba(96, 165, 250, 0.2)",
                          },
                          "&:disabled": {
                            color: "rgba(96, 165, 250, 0.3)",
                            backgroundColor: "rgba(96, 165, 250, 0.05)",
                          },
                        }}
                      >
                        {applyingAffinity ? (
                          <CircularProgress size={16} />
                        ) : (
                          <CheckCircleIcon fontSize="small" />
                        )}
                      </IconButton>

                      <Typography
                        variant="body2"
                        sx={{
                          color:
                            selectedCores.length === 0
                              ? "rgba(255, 255, 255, 0.4)"
                              : "white",
                          fontWeight: 500,
                        }}
                      >
                        {applyingAffinity ? "Applying..." : "Apply Affinity"}
                      </Typography>
                    </Box>
                  </Box>
                </Box>
              </CardContent>
            </Card>
          </Grid>
        </Grid>
      </Box>
    </Modal>
  );
};
export default ModalProcessInfo;
