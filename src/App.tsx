// Performance optimized App component with production-ready patterns
import React, { useCallback, useEffect, useRef, useState } from "react";
import {
  Box,
  Container,
  Fade,
  Skeleton,
  SvgIcon,
  Typography,
} from "@mui/material";
import { DndProvider } from "react-dnd";
import { TouchBackend } from "react-dnd-touch-backend";
import ProcessTable from "./components/ProcessTable";
import ProcessControls, { ProcessFilter } from "./components/ProcessControls";
import ProcessControlBar from "./components/ProcessControlBar";
import GpuCard from "./components/GpuCard";
import Optimizations from "./components/Optimizations";
import SettingsCard from "./components/SettingsCard";
import DraggableCard from "./components/DraggableCard";
import MasonryLayout from "./components/MasonryLayout";
import { invoke } from "@tauri-apps/api/core";
import ModalProcessInfo from "./components/ModalProcessInfo.tsx";
import StatsCard from "./components/StatsCard";
import { SystemStats } from "./data/SystemStats";
import { GpuStats } from "./data/GpuStats";
import CpuUsageCard from "./components/CpuUsageCard";
import MemoryCard from "./components/MemoryCard";
import StorageCard from "./components/StorageCard";
import NetworkCard from "./components/NetworkCard";
import MonitorHealthCard from "./components/MonitorHealthCard";
import Logo from "./assets/aura-icon.svg?react";
import StorageIcon from "@mui/icons-material/Storage";
import ComputerIcon from "@mui/icons-material/Computer";
import MemoryIcon from "@mui/icons-material/Memory";
import NetworkCheckIcon from "@mui/icons-material/NetworkCheck";
import HealthAndSafetyIcon from "@mui/icons-material/HealthAndSafety";

interface DiskUsage {
  read: string;
  write: string;
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

const App: React.FC = () => {
  // State ottimizzato con pattern più efficaci
  const [processes, setProcesses] = useState<ProcessData[]>([]);
  const [updating, setUpdating] = useState(true);
  const [selectedProcess, setSelectedProcess] = useState<ProcessData | null>(
    null,
  );
  const [modalOpen, setModalOpen] = useState(false); // Stato filtro di default: null values to avoid filtering
  const [processFilter, setProcessFilter] = useState<ProcessFilter>({
    searchQuery: "",
    status: "",
    page: 0,
    perPage: 25, // Default to 25 processes per page
  });
  const [totalProcesses, setTotalProcesses] = useState(0);
  // System stats con loading states
  const [systemStats, setSystemStats] = useState({
    cpu: null as SystemStats | null,
    memory: null as SystemStats | null,
    storage: null as SystemStats | null,
    system: null as SystemStats | null,
    network: null as SystemStats | null,
    gpu: null as GpuStats | null,
    loading: {
      cpu: false,
      memory: false,
      storage: false,
      system: false,
      network: false,
      gpu: false,
    },
  });
  const [refreshInterval, setRefreshInterval] = useState(2000);  const [cardOrder, setCardOrder] = useState([
    "cpu",
    "memory",
    "storage",
    "system",
    "network",
    "gpu",
    "monitor-health",
    "settings",
    "optimizations",
  ]);
  // Refs per ottimizzazione
  const processesAbortController = useRef<AbortController | null>(null);
  const statsAbortController = useRef<AbortController | null>(null);
  const lastProcessUpdateTime = useRef(Date.now());
  const lastStatsUpdateTime = useRef(Date.now()); // Memoized handlers per evitare re-render inutili
  const handleFilterChange = useCallback(
    (newFilter: Partial<ProcessFilter>) => {
      setProcessFilter((prev) => ({
        ...prev,
        ...newFilter,
        page: newFilter.page !== undefined ? newFilter.page : 0,
      }));
    },
    [],
  ); // Optimized fetch with debouncing and error recovery
  const fetchProcesses = useCallback(async () => {
    // Cancel previous request if still in progress
    if (processesAbortController.current) {
      processesAbortController.current.abort();
    }
    processesAbortController.current = new AbortController();
    const controller = processesAbortController.current;
    try {
      const timeoutId = setTimeout(() => controller.abort(), 8000);
      const response = await invoke<{
        processes: ProcessData[];
        total_count: number;
      }>("get_running_processes", {
        filter: {
          search_query: processFilter.searchQuery?.trim() || null,
          status: processFilter.status?.trim() || null,
          page: processFilter.page || 0,
          per_page: processFilter.perPage || 50,
          min_cpu: processFilter.minCpu || null,
          min_memory: processFilter.minMemory || null,
          sort_by: processFilter.sortBy || null,
          sort_order: processFilter.sortOrder || null,
        },
      });
      clearTimeout(timeoutId);
      if (!controller.signal.aborted) {
        if (response && Array.isArray(response.processes)) {
          setProcesses(response.processes);
          setTotalProcesses(response.total_count);
        } else {
          console.error(
            "[ERROR] Response does not contain 'processes' key:",
            response,
          );
        }
      }
    } catch (error) {
      if (!controller.signal.aborted) {
        console.error("Error fetching processes:", error);
      }
    }
  }, [
    processFilter.searchQuery,
    processFilter.status,
    processFilter.page,
    processFilter.perPage,
    processFilter.minCpu,
    processFilter.minMemory,
    processFilter.sortBy,
    processFilter.sortOrder,
  ]);
  // Fetch stats ottimizzato con gestione errori intelligente
  const fetchStatsData = useCallback(
    async (statsType: keyof typeof systemStats.loading) => {
      if (systemStats.loading[statsType]) return; // Previeni chiamate duplicate
      setSystemStats((prev) => ({
        ...prev,
        loading: { ...prev.loading, [statsType]: true },
      }));
      try {
        let stats: SystemStats | GpuStats;
        let iconComponent;
        switch (statsType) {
          case "cpu":
            stats = (await invoke("get_cpu_stats")) as SystemStats;
            iconComponent = MemoryIcon;
            break;
          case "memory":
            stats = (await invoke("get_memory_stats")) as SystemStats;
            iconComponent = MemoryIcon;
            break;
          case "storage":
            stats = (await invoke("get_storage_stats")) as SystemStats;
            iconComponent = StorageIcon;
            break;
          case "system":
            stats = (await invoke("get_system_stats")) as SystemStats;
            iconComponent = ComputerIcon;
            break;
          case "network":
            stats = (await invoke("get_network_stats")) as SystemStats;
            iconComponent = NetworkCheckIcon;
            break;
          case "gpu":
            stats = (await invoke("get_gpu_stats")) as GpuStats;
            break;
          default:
            return;
        }
        if (statsType !== "gpu" && iconComponent) {
          (stats as SystemStats).icon = iconComponent;
        }
        setSystemStats((prev) => ({
          ...prev,
          [statsType]: stats,
          loading: { ...prev.loading, [statsType]: false },
        }));
      } catch (error) {
        console.error(`Error fetching ${statsType} stats:`, error);
        setSystemStats((prev) => ({
          ...prev,
          loading: { ...prev.loading, [statsType]: false },
        }));
      }
    },
    [systemStats.loading],
  );
  // Batch fetch per migliorare performance
  const fetchAllStats = useCallback(async () => {
    if (statsAbortController.current) {
      statsAbortController.current.abort();
    }
    statsAbortController.current = new AbortController();
    const promises = [
      "cpu",
      "memory",
      "storage",
      "system",
      "network",
      "gpu",
    ].map((statsType) =>
      fetchStatsData(statsType as keyof typeof systemStats.loading),
    );
    try {
      await Promise.allSettled(promises);
    } catch (error) {
      console.error("Error in batch stats fetch:", error);
    }
  }, [fetchStatsData, systemStats.loading]); // Initial fetch on component mount - runs only once
  useEffect(() => {
    // Simple initial fetch without dependencies
    const initialFetch = async () => {
      try {
        const response = await invoke<{
          processes: ProcessData[];
          total_count: number;
        }>("get_running_processes", {
          filter: {
            search_query: null,
            status: null,
            page: 0,
            per_page: 50,
            min_cpu: null,
            min_memory: null,
            sort_by: null,
            sort_order: null,
          },
        });
        if (response && Array.isArray(response.processes)) {
          setProcesses(response.processes);
          setTotalProcesses(response.total_count);
        }
      } catch (error) {
        console.error("Initial fetch error:", error);
      }
    };
    initialFetch();
    fetchAllStats();
  }, []); // Empty dependency array - runs only once on mount  // Effect for automatic updates with improved debouncing
  useEffect(() => {
    let processInterval: number;
    let statsInterval: number;
    if (updating) {
      // Intervals with intelligent throttling
      processInterval = setInterval(
        () => {
          const now = Date.now();
          if (now - lastProcessUpdateTime.current >= refreshInterval) {
            lastProcessUpdateTime.current = now;
            fetchProcesses();
          }
        },
        Math.max(refreshInterval, 1000),
      ); // Minimum 1 second
      statsInterval = setInterval(
        () => {
          const now = Date.now();
          if (now - lastStatsUpdateTime.current >= refreshInterval * 2) {
            lastStatsUpdateTime.current = now;
            fetchAllStats();
          }
        },
        Math.max(refreshInterval * 2, 2000),
      ); // Stats less frequent
    }
    return () => {
      clearInterval(processInterval);
      clearInterval(statsInterval);
      // Cleanup abort controllers
      if (processesAbortController.current) {
        processesAbortController.current.abort();
      }
      if (statsAbortController.current) {
        statsAbortController.current.abort();
      }
    };
  }, [updating, refreshInterval, fetchProcesses, fetchAllStats]); // Separate effect for filter changes - immediate fetch
  useEffect(() => {
    fetchProcesses();
  }, [processFilter, fetchProcesses]); // Event handlers ottimizzati
  const handleOpenModal = useCallback((process: ProcessData) => {
    setSelectedProcess(process);
    setModalOpen(true);
  }, []);
  const handleCloseModal = useCallback(() => {
    setSelectedProcess(null);
    setModalOpen(false);
  }, []);
  const toggleUpdating = useCallback(() => {
    setUpdating((prev) => !prev);
  }, []);
  const handleAffinityChange = useCallback(
    async (pid: number, cpuIndex: number) => {
      try {
        await invoke("toggle_cpu_affinity", { pid, cpuIndex });
        await fetchProcesses();
      } catch (error) {
        console.error("Error toggling CPU affinity:", error);
      }
    },
    [fetchProcesses],
  );
  const handleRefreshIntervalChange = useCallback((interval: number) => {
    setRefreshInterval(interval);
  }, []);
  const handleProcessPerPageChange = useCallback((perPage: number) => {
    setProcessFilter((prev) => ({ ...prev, perPage, page: 0 }));
  }, []);
  // Caricamento ordine card da localStorage
  useEffect(() => {
    const savedOrder = localStorage.getItem("aura_card_order");
    if (savedOrder) {
      try {
        const parsed = JSON.parse(savedOrder);
        if (
          Array.isArray(parsed) &&
          parsed.every((x) => typeof x === "string")
        ) {
          setCardOrder(parsed);
        }
      } catch {}
    }
  }, []);
  // Salvataggio ordine card su spostamento
  useEffect(() => {
    localStorage.setItem("aura_card_order", JSON.stringify(cardOrder));
  }, [cardOrder]);
  // Drag & drop ottimizzato con debouncing
  const handleCardMove = useCallback(
    (dragIndex: number, hoverIndex: number) => {
      setCardOrder((prev) => {
        const newOrder = [...prev];
        const draggedItem = newOrder[dragIndex];
        newOrder.splice(dragIndex, 1);
        newOrder.splice(hoverIndex, 0, draggedItem);
        return newOrder;
      });
    },
    [],
  ); // Memoized card renderer per evitare re-render inutili
  const renderCard = useCallback(
    (cardType: string, index: number) => {
      const cardProps = { id: cardType, index, onMove: handleCardMove };
      switch (cardType) {        case "cpu":
          return systemStats.cpu ? (
            <DraggableCard key={cardType} {...cardProps}>
              <CpuUsageCard
                icon={systemStats.cpu.icon}
                title={systemStats.cpu.title}
                percentage={systemStats.cpu?.percentage}
                progressData={systemStats.cpu?.progress_data}
                genericData={systemStats.cpu?.generic_data}
              />
            </DraggableCard>
          ) : (
            <DraggableCard key={cardType} {...cardProps}>
              <Skeleton variant="rectangular" height={200} />
            </DraggableCard>
          );
        case "memory":
          return systemStats.memory ? (
            <DraggableCard key={cardType} {...cardProps}>
              <MemoryCard
                icon={systemStats.memory.icon}
                title={systemStats.memory.title}
                percentage={systemStats.memory.percentage}
                progressData={systemStats.memory.progress_data}
                genericData={systemStats.memory.generic_data}
              />
            </DraggableCard>
          ) : (
            <DraggableCard key={cardType} {...cardProps}>
              <Skeleton variant="rectangular" height={200} />
            </DraggableCard>
          );
        case "storage":
          return systemStats.storage ? (
            <DraggableCard key={cardType} {...cardProps}>
              <StorageCard
                icon={systemStats.storage.icon}
                title={systemStats.storage.title}
                percentage={systemStats.storage.percentage}
                progressData={systemStats.storage.progress_data}
                genericData={systemStats.storage.generic_data}
              />
            </DraggableCard>
          ) : (
            <DraggableCard key={cardType} {...cardProps}>
              <Skeleton variant="rectangular" height={200} />
            </DraggableCard>
          );
        case "system":
          return systemStats.system ? (
            <DraggableCard key={cardType} {...cardProps}>
              <StatsCard
                icon={systemStats.system.icon}
                title={systemStats.system.title}
                percentage={systemStats.system.percentage}
                progressData={systemStats.system.progress_data}
                genericData={systemStats.system.generic_data}
              />
            </DraggableCard>
          ) : (
            <DraggableCard key={cardType} {...cardProps}>
              <Skeleton variant="rectangular" height={200} />
            </DraggableCard>
          );
        case "network":
          return systemStats.network ? (
            <DraggableCard key={cardType} {...cardProps}>
              <NetworkCard
                icon={systemStats.network.icon}
                title={systemStats.network.title}
                percentage={systemStats.network.percentage}
                progressData={systemStats.network.progress_data}
                genericData={systemStats.network.generic_data}
              />
            </DraggableCard>
          ) : (
            <DraggableCard key={cardType} {...cardProps}>
              <Skeleton variant="rectangular" height={200} />
            </DraggableCard>
          );        case "gpu":
          return systemStats.gpu ? (
            <DraggableCard key={cardType} {...cardProps}>
              <GpuCard gpuStats={systemStats.gpu} />
            </DraggableCard>
          ) : (
            <DraggableCard key={cardType} {...cardProps}>
              <Skeleton variant="rectangular" height={200} />
            </DraggableCard>
          );
        case "monitor-health":
          return (
            <DraggableCard key={cardType} {...cardProps}>
              <MonitorHealthCard
                icon={HealthAndSafetyIcon}
                title="Monitor Health"
              />
            </DraggableCard>
          );
        case "settings":
          return (
            <DraggableCard key={cardType} {...cardProps}>
              <SettingsCard
                refreshInterval={refreshInterval}
                onRefreshIntervalChange={handleRefreshIntervalChange}
                updating={updating}
                onToggleUpdating={toggleUpdating}
                processPerPage={processFilter.perPage}
                onProcessPerPageChange={handleProcessPerPageChange}
              />
            </DraggableCard>
          );
        case "optimizations":
          return (
            <DraggableCard key={cardType} {...cardProps}>
              <Optimizations refreshInterval={refreshInterval} />
            </DraggableCard>
          );
        default:
          return null;
      }
    },
    [
      handleCardMove,
      refreshInterval,
      updating,
      processFilter.perPage,
      handleRefreshIntervalChange,
      toggleUpdating,
      handleProcessPerPageChange,
      systemStats.cpu,
      systemStats.memory,
      systemStats.storage,
      systemStats.system,
      systemStats.network,
      systemStats.gpu,
    ],
  );
  return (
    <DndProvider backend={TouchBackend} options={{ enableMouseEvents: true }}>
      <Container maxWidth="xl" sx={{ p: 2 }}>
        <Fade in timeout={300}>
          <Box
            display="flex"
            alignItems="center"
            justifyContent="center"
            mb={1}
          >
            <SvgIcon component={Logo} inheritViewBox />
            <Typography variant="body1" ml={1} fontWeight="bold">
              Aura Manager
            </Typography>
          </Box>
        </Fade>
        <MasonryLayout
          children={
            cardOrder
              .map((cardType, index) => renderCard(cardType, index))
              .filter(Boolean) as React.ReactElement[]
          }
        />
        <Fade in timeout={700}>
          <Box mt={3}>
            <ProcessControls
              filter={processFilter}
              onFilterChange={handleFilterChange}
            />
          </Box>
        </Fade>{" "}
        <Fade in timeout={900}>
          <Box mt={2}>
            <ProcessTable processes={processes} onOpenModal={handleOpenModal} />
          </Box>
        </Fade>
        <Fade in timeout={1100}>
          <Box>
            <ProcessControlBar
              currentPage={processFilter.page + 1}
              totalPages={Math.ceil(totalProcesses / processFilter.perPage)}
              onPageChange={(page) => handleFilterChange({ page: page - 1 })}
              updating={updating}
              toggleUpdating={toggleUpdating}
            />
          </Box>
        </Fade>
        {selectedProcess && (
          <ModalProcessInfo
            open={modalOpen}
            process={selectedProcess}
            onClose={handleCloseModal}
            onAffinityChange={handleAffinityChange}
          />
        )}
      </Container>
    </DndProvider>
  );
};
export default App;
