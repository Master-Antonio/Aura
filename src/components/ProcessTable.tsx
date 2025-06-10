import React, { useCallback, useMemo } from "react";
import {
  Box,
  Chip,
  IconButton,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Tooltip,
  Typography,
} from "@mui/material";
import SportsEsportsIcon from "@mui/icons-material/SportsEsports";
import CloseIcon from "@mui/icons-material/Close";
import InfoIcon from "@mui/icons-material/Info";
import PauseIcon from "@mui/icons-material/Pause";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import { invoke } from "@tauri-apps/api/core";

export interface ProcessData {
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

interface ProcessTableProps {
  processes: ProcessData[];
  onOpenModal: (process: ProcessData) => void;
}

const ProcessTable: React.FC<ProcessTableProps> = ({
  processes,
  onOpenModal,
}) => {
  const handleKillProcess = useCallback(async (pid: number) => {
    try {
      await invoke("kill_process", { pid });
    } catch (error) {
      console.error("Failed to kill process:", error);
    }
  }, []);
  const handleGamingBoost = useCallback(async (pid: number) => {
    try {
      await invoke("boost_process_for_gaming", { pid });
    } catch (error) {
      console.error("Failed to apply gaming boost:", error);
    }
  }, []);
  const handleSuspendProcess = useCallback(async (pid: number) => {
    try {
      await invoke("suspend_process", { pid });
    } catch (error) {
      console.error("Failed to suspend process:", error);
    }
  }, []);
  const handleResumeProcess = useCallback(async (pid: number) => {
    try {
      await invoke("resume_process", { pid });
    } catch (error) {
      console.error("Failed to resume process:", error);
    }
  }, []);
  const handleOpenFileLocation = useCallback(async (exePath: string) => {
    try {
      // This would need to be implemented in the backend
      await invoke("open_file_location", { path: exePath });
    } catch (error) {
      console.error("Failed to open file location:", error);
    }
  }, []);
  const handleOpenModal = useCallback(
    (process: ProcessData) => () => {
      onOpenModal(process);
    },
    [onOpenModal],
  );
  const tableHeaders = useMemo(
    () => [
      { id: "name", label: "Process Name", width: "12.5%" },
      { id: "pid", label: "PID", width: "12.5%" },
      { id: "status", label: "Status", width: "12.5%" },
      { id: "cpu", label: "CPU %", width: "12.5%" },
      { id: "ram", label: "RAM", width: "12.5%" },
      { id: "disk", label: "Disk I/O", width: "12.5%" },
      { id: "runtime", label: "Run Time", width: "12.5%" },
      { id: "actions", label: "Actions", width: "13%" },
    ],
    [],
  );
  // Helper per formattare l'uso della memoria
  const formatMemoryUsage = useCallback((ramUsage: number) => {
    if (ramUsage >= 1024) {
      return `${(ramUsage / 1024).toFixed(1)} GB`;
    }
    return `${ramUsage} MB`;
  }, []);
  // Componente per lo status del processo
  const ProcessStatusChip = useCallback(({ status }: { status: string }) => {
    const getStatusConfig = (status: string) => {
      const lowerStatus = status.toLowerCase();
      if (lowerStatus.includes("running") || lowerStatus.includes("runnable")) {
        return {
          color: "#2e7d32", // Dark green
          bgColor: "#c8e6c9", // Light green background
          textColor: "#1b5e20", // Very dark green text
        };
      } else if (lowerStatus.includes("suspended")) {
        return {
          color: "#f57c00", // Orange
          bgColor: "#ffe0b2", // Light orange background
          textColor: "#e65100", // Dark orange text
        };
      } else if (lowerStatus.includes("sleeping")) {
        return {
          color: "#1976d2", // Blue
          bgColor: "#bbdefb", // Light blue background
          textColor: "#0d47a1", // Dark blue text
        };
      } else if (lowerStatus.includes("stopped")) {
        return {
          color: "#d32f2f", // Red
          bgColor: "#ffcdd2", // Light red background
          textColor: "#b71c1c", // Dark red text
        };
      }
      return {
        color: "#757575", // Grey
        bgColor: "#f5f5f5", // Light grey background
        textColor: "#424242", // Dark grey text
      };
    };
    const config = getStatusConfig(status);
    return (
      <Chip
        label={status}
        size="small"
        sx={{
          backgroundColor: config.bgColor,
          color: config.textColor,
          fontWeight: 600,
          textTransform: "capitalize",
          minWidth: "80px",
          border: `1px solid ${config.color}`,
          fontSize: "0.75rem",
          height: 24,
          margin: "0 auto", // Centrato nella cella
          display: "block", // Per far funzionare il margin auto
          "& .MuiChip-label": {
            px: 1.5,
          },
        }}
      />
    );
  }, []);
  // Componente per l'uso della CPU
  const CpuUsageCell = useCallback(({ usage }: { usage: number }) => {
    const getColor = (usage: number) => {
      if (usage > 80) return "error.main";
      if (usage > 50) return "warning.main";
      if (usage > 20) return "info.main";
      return "success.main";
    };
    const getBgColor = (usage: number) => {
      if (usage > 80) return "error.50";
      if (usage > 50) return "warning.50";
      if (usage > 20) return "info.50";
      return "success.50";
    };
    return (
      <Box
        display="flex"
        alignItems="center"
        justifyContent="center" // Centrato
        gap={1.5}
        sx={{
          backgroundColor: getBgColor(usage),
          borderRadius: 1,
          px: 1,
          py: 0.5,
          minWidth: 70,
          width: "fit-content", // Adatta alla larghezza del contenuto
          margin: "0 auto", // Centrato nella cella
        }}
      >
        <Box
          sx={{
            width: 10,
            height: 10,
            borderRadius: "50%",
            backgroundColor: getColor(usage),
            boxShadow: `0 0 6px ${getColor(usage)}`,
          }}
        />
        <Typography
          variant="body2"
          sx={{
            fontWeight: 600,
            color: getColor(usage),
            fontSize: "0.875rem",
          }}
        >
          {usage.toFixed(1)}%
        </Typography>
      </Box>
    );
  }, []);
  return (
    <TableContainer
      component={Paper}
      sx={{
        borderRadius: 3,
        position: "relative",
        boxShadow: "0 4px 20px rgba(0, 0, 0, 0.08)",
        border: "1px solid",
        borderColor: "divider",
        overflow: "hidden", // Previene lo scroll orizzontale
        maxWidth: "100%",
        overflowX: "hidden", // Forza nascondere lo scroll orizzontale
        "&::-webkit-scrollbar": {
          display: "none",
        },
        scrollbarWidth: "none", // Firefox
      }}
    >
      <Table
        sx={{
          tableLayout: "fixed",
          position: "relative",
          width: "100%",
          minWidth: "1000px", // Aumentata larghezza minima
          // Stabilizza le dimensioni durante gli aggiornamenti
          transition: "none",
          "& .MuiTableCell-root": {
            transition: "none", // Rimuove transizioni che possono causare flicker
            padding: "12px 8px", // Padding uniforme per tutte le celle
            borderRight: "1px solid rgba(224, 224, 224, 0.1)", // Linee di separazione sottili
            "&:last-child": {
              borderRight: "none",
            },
          },
        }}
      >
        <TableHead
          sx={{
            position: "sticky",
            top: 0,
            zIndex: 1,
            backgroundColor: "#0a0b11",
            borderBottom: "2px solid",
            borderColor: "divider",
          }}
        >
          <TableRow>
            {tableHeaders.map((header) => (
              <TableCell
                key={header.id}
                sx={{
                  fontWeight: 600,
                  fontSize: "0.875rem",
                  color: "white",
                  py: 2,
                  textTransform: "uppercase",
                  letterSpacing: "0.5px",
                  width: header.width,
                  minWidth: header.width,
                  maxWidth: header.width,
                  textAlign: 
                    header.id === "name" ? "left" :
                    header.id === "actions" ? "center" :
                    "center", // Tutti gli altri header centrati
                }}
              >
                {header.label}
              </TableCell>
            ))}
          </TableRow>
        </TableHead>
        <TableBody>
          {processes.map((process) => (
            <TableRow
              hover
              key={process.pid}
              sx={{
                transition: "all 0.2s ease-in-out",
                "&:nth-of-type(odd)": {
                  backgroundColor: "grey.25",
                },
                borderLeft: "3px solid transparent",
                "&:hover": {
                  borderLeftColor: "primary.main",
                  backgroundColor: "primary.50",
                  transform: "scale(1.002)",
                  boxShadow: "0 2px 8px rgba(0, 0, 0, 0.1)",
                },
              }}
            >
              <TableCell
                sx={{
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                  whiteSpace: "nowrap",
                  fontWeight: 500,
                  py: 2,
                  width: "12.5%",
                  minWidth: "12.5%",
                  maxWidth: "12.5%",
                  position: "relative", // Per contenere il contenuto
                }}
                title={process.name} // Tooltip per nome completo
              >
                {process.affinity_set && (
                  <CheckCircleIcon
                    fontSize="small"
                    sx={{
                      color: "success.main",
                      mr: 1,
                      verticalAlign: "middle",
                    }}
                  />
                )}
                {process.name}
              </TableCell>
              <TableCell
                sx={{
                  fontFamily: "monospace",
                  fontSize: "0.875rem",
                  color: "text.secondary",
                  py: 2,
                  width: "12.5%",
                  minWidth: "12.5%",
                  maxWidth: "12.5%",
                  textAlign: "center",
                }}
              >
                {process.pid}
              </TableCell>
              <TableCell
                sx={{
                  py: 2,
                  width: "12.5%",
                  minWidth: "12.5%",
                  maxWidth: "12.5%",
                  textAlign: "center", // Centrato
                }}
              >
                <ProcessStatusChip status={process.status} />
              </TableCell>
              <TableCell
                sx={{
                  py: 2,
                  width: "12.5%",
                  minWidth: "12.5%",
                  maxWidth: "12.5%",
                  textAlign: "center", // Centrato
                }}
              >
                <CpuUsageCell usage={process.cpu_usage} />
              </TableCell>
              <TableCell
                sx={{
                  fontWeight: 500,
                  py: 2,
                  width: "12.5%",
                  minWidth: "12.5%",
                  maxWidth: "12.5%",
                  textAlign: "center", // Centrato invece di right
                }}
              >
                {formatMemoryUsage(process.ram_usage)}
              </TableCell>
              <TableCell
                sx={{
                  fontSize: "0.8rem",
                  color: "text.secondary",
                  fontFamily: "monospace",
                  py: 2,
                  width: "12.5%",
                  minWidth: "12.5%",
                  maxWidth: "12.5%",
                  textAlign: "center",
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                  whiteSpace: "nowrap",
                }}
                title={`Read: ${process.disk_usage.read} / Write: ${process.disk_usage.write}`}
              >
                <Box
                  sx={{
                    display: "inline-block",
                    maxWidth: "100%",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    whiteSpace: "nowrap",
                  }}
                >
                  {process.disk_usage.read} / {process.disk_usage.write}
                </Box>
              </TableCell>
              <TableCell
                sx={{
                  fontSize: "0.8rem",
                  color: "text.secondary",
                  py: 2,
                  width: "12.5%",
                  minWidth: "12.5%",
                  maxWidth: "12.5%",
                  textAlign: "center",
                }}
              >
                {process.run_time}
              </TableCell>
              <TableCell
                sx={{
                  width: "13%",
                  minWidth: "13%",
                  maxWidth: "13%",
                  py: 2,
                  textAlign: "center", // Centrato
                }}
              >
                <ProcessActions
                  process={process}
                  onGamingBoost={handleGamingBoost}
                  onOpenInfo={handleOpenModal(process)}
                  onKill={handleKillProcess}
                  onSuspend={handleSuspendProcess}
                  onResume={handleResumeProcess}
                  onOpenFileLocation={handleOpenFileLocation}
                />
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

interface ProcessActionsProps {
  process: ProcessData;
  onGamingBoost: (pid: number) => Promise<void>;
  onOpenInfo: () => void;
  onKill: (pid: number) => Promise<void>;
  onSuspend: (pid: number) => Promise<void>;
  onResume: (pid: number) => Promise<void>;
  onOpenFileLocation: (exePath: string) => Promise<void>;
}

const ProcessActions: React.FC<ProcessActionsProps> = React.memo(
  ({
    process,
    onGamingBoost,
    onOpenInfo,
    onKill,
    onSuspend,
    onResume,
    onOpenFileLocation,
  }) => {
    const isRunning =
      process.status.toLowerCase() === "running" ||
      process.status.toLowerCase() === "runnable";
    const isSuspended =
      process.status.toLowerCase().includes("stopped") ||
      process.status.toLowerCase().includes("suspended");
    return (
      <Box
        display="flex"
        gap={0.5}
        alignItems="center"
        justifyContent="center" // Centrato orizzontalmente
        sx={{
          width: "100%", // Occupa tutta la larghezza della cella
          "& .MuiIconButton-root": {
            transition: "all 0.2s ease-in-out",
            "&:hover": {
              transform: "scale(1.1)",
              boxShadow: "0 2px 8px rgba(0, 0, 0, 0.15)",
            },
          },
        }}
      >
        <Tooltip title="Gaming Aura Boost (Optimized for P-cores)" arrow>
          <IconButton
            sx={{
              backgroundColor: "primary.50",
              color: "primary.main",
              border: "1px solid",
              borderColor: "primary.200",
              "&:hover": {
                backgroundColor: "primary.100",
                borderColor: "primary.main",
              },
            }}
            onClick={() => onGamingBoost(process.pid)}
            size="small"
          >
            <SportsEsportsIcon fontSize="small" />
          </IconButton>
        </Tooltip>

        <Tooltip title="Process Info" arrow>
          <IconButton
            sx={{
              backgroundColor: "info.50",
              color: "info.main",
              border: "1px solid",
              borderColor: "info.200",
              "&:hover": {
                backgroundColor: "info.100",
                borderColor: "info.main",
              },
            }}
            onClick={onOpenInfo}
            size="small"
          >
            <InfoIcon fontSize="small" />
          </IconButton>
        </Tooltip>

        {/* Suspend/Resume based on process status */}
        {isRunning ? (
          <Tooltip title="Suspend Process" arrow>
            <IconButton
              sx={{
                backgroundColor: "warning.50",
                color: "warning.main",
                border: "1px solid",
                borderColor: "warning.200",
                "&:hover": {
                  backgroundColor: "warning.100",
                  borderColor: "warning.main",
                },
              }}
              onClick={() => onSuspend(process.pid)}
              size="small"
            >
              <PauseIcon fontSize="small" />
            </IconButton>
          </Tooltip>
        ) : isSuspended ? (
          <Tooltip title="Resume Process" arrow>
            <IconButton
              sx={{
                backgroundColor: "success.50",
                color: "success.main",
                border: "1px solid",
                borderColor: "success.200",
                "&:hover": {
                  backgroundColor: "success.100",
                  borderColor: "success.main",
                },
              }}
              onClick={() => onResume(process.pid)}
              size="small"
            >
              <PlayArrowIcon fontSize="small" />
            </IconButton>
          </Tooltip>
        ) : null}

        {/* Open file location if exe path exists */}
        {process.exe_path && process.exe_path !== "N/A" && (
          <Tooltip title="Open File Location" arrow>
            <IconButton
              sx={{
                backgroundColor: "secondary.50",
                color: "secondary.main",
                border: "1px solid",
                borderColor: "secondary.200",
                "&:hover": {
                  backgroundColor: "secondary.100",
                  borderColor: "secondary.main",
                },
              }}
              onClick={() => onOpenFileLocation(process.exe_path)}
              size="small"
            >
              <FolderOpenIcon fontSize="small" />
            </IconButton>
          </Tooltip>
        )}

        <Tooltip title="End Process" arrow>
          <IconButton
            sx={{
              backgroundColor: "error.50",
              color: "error.main",
              border: "1px solid",
              borderColor: "error.200",
              "&:hover": {
                backgroundColor: "error.100",
                borderColor: "error.main",
              },
            }}
            onClick={() => onKill(process.pid)}
            size="small"
          >
            <CloseIcon fontSize="small" />
          </IconButton>
        </Tooltip>
      </Box>
    );
  },
);
// Export default ProcessTable
export default ProcessTable;
