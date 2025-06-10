import React, { useCallback, useState } from "react";
import {
  Box,
  Button,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  TextField,
} from "@mui/material";
import SearchIcon from "@mui/icons-material/Search";
import ClearIcon from "@mui/icons-material/Clear";
import Grid from "@mui/material/Grid";

export interface ProcessFilter {
  searchQuery: string;
  status: string;
  page: number;
  perPage: number;
  minCpu?: number;
  minMemory?: number;
  sortBy?: "name" | "cpu" | "memory" | "pid";
  sortOrder?: "asc" | "desc";
}

export interface ProcessControlsProps {
  filter: ProcessFilter;
  onFilterChange: (newFilter: Partial<ProcessFilter>) => void;
}

const STATUS_OPTIONS = [
  { value: "runnable", label: "Running" },
  { value: "sleeping", label: "Sleeping" },
  { value: "suspended", label: "Suspended" },
  { value: "stopped", label: "Stopped" },
] as const;
export const ProcessControls: React.FC<ProcessControlsProps> = ({
  filter,
  onFilterChange,
}) => {
  // Local state for filters before applying
  const [localFilter, setLocalFilter] = useState({
    searchQuery: filter.searchQuery,
    status: filter.status,
    minCpu: filter.minCpu,
    minMemory: filter.minMemory,
    sortBy: filter.sortBy,
    sortOrder: filter.sortOrder,
  });
  const handleLocalSearchChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      setLocalFilter((prev) => ({ ...prev, searchQuery: event.target.value }));
    },
    [],
  );
  const handleLocalStatusChange = useCallback((event: SelectChangeEvent) => {
    setLocalFilter((prev) => ({ ...prev, status: event.target.value }));
  }, []);
  const handleLocalMinCpuChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const value = event.target.value;
      setLocalFilter((prev) => ({
        ...prev,
        minCpu: value === "" ? undefined : parseFloat(value),
      }));
    },
    [],
  );
  const handleLocalMinMemoryChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const value = event.target.value;
      setLocalFilter((prev) => ({
        ...prev,
        minMemory: value === "" ? undefined : parseFloat(value) * 1024 * 1024, // Convert MB to bytes
      }));
    },
    [],
  );
  const handleLocalSortChange = useCallback((event: SelectChangeEvent) => {
    const [sortBy, sortOrder] = event.target.value.split("-");
    setLocalFilter((prev) => ({
      ...prev,
      sortBy: sortBy as "name" | "cpu" | "memory" | "pid",
      sortOrder: sortOrder as "asc" | "desc",
    }));
  }, []);
  const applyFilters = useCallback(() => {
    onFilterChange({
      ...localFilter,
      page: 0, // Reset page when applying filters
    });
  }, [localFilter, onFilterChange]);
  const clearFilters = useCallback(() => {
    const clearedFilter = {
      searchQuery: "",
      status: "",
      minCpu: undefined,
      minMemory: undefined,
      sortBy: "name" as const,
      sortOrder: "asc" as const,
    };
    setLocalFilter(clearedFilter);
    onFilterChange({
      ...clearedFilter,
      page: 0,
    });
  }, [onFilterChange]);
  return (
    <Box
      sx={{
        //p: 2,
        //mb: 2,
        borderRadius: 3,
        backgroundColor: "#1c1d2a",
        //border: '1px solid rgba(255, 255, 255, 0.1)',
        boxShadow: "0 2px 8px rgba(0, 0, 0, 0.05)",
      }}
    >
      <Grid container spacing={2} alignItems="center">
        <Grid size={3}>
          {" "}
          <TextField
            size="small"
            label="Search process"
            variant="outlined"
            placeholder="By name or PID"
            value={localFilter.searchQuery}
            onChange={handleLocalSearchChange}
            fullWidth
            sx={{
              "& .MuiOutlinedInput-root": {
                backgroundColor: "#0a0b11",
                borderRadius: 2,
                color: "white",
                "& fieldset": {
                  borderColor: "rgba(255, 255, 255, 0.2)",
                },
                "&:hover .MuiOutlinedInput-notchedOutline": {
                  borderColor: "rgba(255, 255, 255, 0.3)",
                },
                "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
                  borderColor: "primary.main",
                  borderWidth: 2,
                },
              },
              "& .MuiInputLabel-root": {
                color: "rgba(255, 255, 255, 0.7)",
              },
            }}
          />
        </Grid>
        <Grid size={1.5}>
          {" "}
          <FormControl size="small" variant="outlined" fullWidth>
            <InputLabel sx={{ color: "rgba(255, 255, 255, 0.7)" }}>
              Status
            </InputLabel>{" "}
            <Select
              value={localFilter.status}
              onChange={handleLocalStatusChange}
              label="Status"
              sx={{
                backgroundColor: "#0a0b11",
                borderRadius: 2,
                color: "white",
                "& .MuiOutlinedInput-notchedOutline": {
                  borderColor: "rgba(255, 255, 255, 0.2)",
                },
                "&:hover .MuiOutlinedInput-notchedOutline": {
                  borderColor: "rgba(255, 255, 255, 0.3)",
                },
                "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
                  borderColor: "primary.main",
                  borderWidth: 2,
                },
                "& .MuiSvgIcon-root": {
                  color: "rgba(255, 255, 255, 0.7)",
                },
              }}
              MenuProps={{
                PaperProps: {
                  sx: {
                    backgroundColor: "#0a0b11",
                    "& .MuiMenuItem-root": {
                      color: "white",
                      "&:hover": {
                        backgroundColor: "rgba(255, 255, 255, 0.1)",
                      },
                    },
                  },
                },
              }}
            >
              <MenuItem value="">All</MenuItem>
              {STATUS_OPTIONS.map((option) => (
                <MenuItem key={option.value} value={option.value}>
                  {option.label}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
        </Grid>
        <Grid size={1.5}>
          {" "}
          <TextField
            size="small"
            label="Min CPU %"
            variant="outlined"
            type="number"
            placeholder="0.0"
            value={localFilter.minCpu || ""}
            onChange={handleLocalMinCpuChange}
            fullWidth
            slotProps={{
              input: {
                inputProps: { min: 0, step: 0.1 },
              },
            }}
            sx={{
              "& .MuiOutlinedInput-root": {
                backgroundColor: "#0a0b11",
                borderRadius: 2,
                color: "white",
                "& fieldset": {
                  borderColor: "rgba(255, 255, 255, 0.2)",
                },
                "&:hover .MuiOutlinedInput-notchedOutline": {
                  borderColor: "rgba(255, 255, 255, 0.3)",
                },
                "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
                  borderColor: "primary.main",
                  borderWidth: 2,
                },
              },
              "& .MuiInputLabel-root": {
                color: "rgba(255, 255, 255, 0.7)",
              },
            }}
          />
        </Grid>
        <Grid size={1.5}>
          {" "}
          <TextField
            size="small"
            label="Min RAM MB"
            variant="outlined"
            type="number"
            placeholder="0"
            value={
              localFilter.minMemory
                ? Math.round(localFilter.minMemory / (1024 * 1024))
                : ""
            }
            onChange={handleLocalMinMemoryChange}
            fullWidth
            slotProps={{
              input: {
                inputProps: { min: 0, step: 1 },
              },
            }}
            sx={{
              "& .MuiOutlinedInput-root": {
                backgroundColor: "#0a0b11",
                borderRadius: 2,
                color: "white",
                "& fieldset": {
                  borderColor: "rgba(255, 255, 255, 0.2)",
                },
                "&:hover .MuiOutlinedInput-notchedOutline": {
                  borderColor: "rgba(255, 255, 255, 0.3)",
                },
                "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
                  borderColor: "primary.main",
                  borderWidth: 2,
                },
              },
              "& .MuiInputLabel-root": {
                color: "rgba(255, 255, 255, 0.7)",
              },
            }}
          />
        </Grid>
        <Grid size={1.5}>
          {" "}
          <FormControl size="small" variant="outlined" fullWidth>
            <InputLabel sx={{ color: "rgba(255, 255, 255, 0.7)" }}>
              Sort by
            </InputLabel>{" "}
            <Select
              value={`${localFilter.sortBy || "name"}-${localFilter.sortOrder || "asc"}`}
              onChange={handleLocalSortChange}
              label="Sort by"
              sx={{
                backgroundColor: "#0a0b11",
                borderRadius: 2,
                color: "white",
                "& .MuiOutlinedInput-notchedOutline": {
                  borderColor: "rgba(255, 255, 255, 0.2)",
                },
                "&:hover .MuiOutlinedInput-notchedOutline": {
                  borderColor: "rgba(255, 255, 255, 0.3)",
                },
                "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
                  borderColor: "primary.main",
                  borderWidth: 2,
                },
                "& .MuiSvgIcon-root": {
                  color: "rgba(255, 255, 255, 0.7)",
                },
              }}
              MenuProps={{
                PaperProps: {
                  sx: {
                    backgroundColor: "#0a0b11",
                    "& .MuiMenuItem-root": {
                      color: "white",
                      "&:hover": {
                        backgroundColor: "rgba(255, 255, 255, 0.1)",
                      },
                    },
                  },
                },
              }}
            >
              <MenuItem value="name-asc">Name ↑</MenuItem>
              <MenuItem value="name-desc">Name ↓</MenuItem>
              <MenuItem value="cpu-desc">CPU ↓</MenuItem>
              <MenuItem value="cpu-asc">CPU ↑</MenuItem>
              <MenuItem value="memory-desc">Memory ↓</MenuItem>
              <MenuItem value="memory-asc">Memory ↑</MenuItem>
              <MenuItem value="pid-asc">PID ↑</MenuItem>
              <MenuItem value="pid-desc">PID ↓</MenuItem>
            </Select>
          </FormControl>
        </Grid>{" "}
        <Grid size={3}>
          <Box display="flex" flexDirection="row-reverse" gap={1}>
            <Button
              variant="contained"
              size="small"
              startIcon={<SearchIcon />}
              onClick={applyFilters}
              sx={{
                borderRadius: 2,
                textTransform: "none",
                fontWeight: 600,
                px: 2,
                boxShadow: "0 2px 8px rgba(25, 118, 210, 0.2)",
                "&:hover": {
                  boxShadow: "0 4px 12px rgba(25, 118, 210, 0.3)",
                  transform: "translateY(-1px)",
                },
              }}
            >
              Search
            </Button>
            <Button
              variant="outlined"
              size="small"
              startIcon={<ClearIcon />}
              onClick={clearFilters}
              sx={{
                borderRadius: 2,
                textTransform: "none",
                fontWeight: 600,
                px: 2,
                borderColor: "grey.300",
                color: "text.secondary",
                "&:hover": {
                  borderColor: "error.main",
                  color: "error.main",
                  backgroundColor: "error.50",
                  transform: "translateY(-1px)",
                },
              }}
            >
              Clear
            </Button>{" "}
          </Box>
        </Grid>
      </Grid>
    </Box>
  );
};
export default React.memo(ProcessControls);
