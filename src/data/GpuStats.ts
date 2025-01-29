export interface GpuInfo {
  name: string;
  vendor: string;
  utilization: number;
  memory_used: number;
  memory_total: number;
  memory_usage_percentage: number;
  temperature?: number;
  power_usage?: number;
  clock_speed?: number;
  memory_clock?: number;
  driver_version?: string;
  is_nvidia: boolean;
  is_amd: boolean;
}

export interface GpuStats {
  gpus: GpuInfo[];
  total_vram_used: number;
  total_vram: number;
  average_utilization: number;
}
