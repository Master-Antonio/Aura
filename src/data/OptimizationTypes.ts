export interface OptimizationItem {
  id: string;
  name: string;
  description: string;
  category: string; // Changed from OptimizationCategory to string to match backend
  requires_admin: boolean;
  supported_platforms: string[];
  applied: boolean; // Maps to is_applied from backend
  is_reversible?: boolean;
  risk_level?: string;
  platform?: string;
}

export interface OptimizationCategory {
  id?: string;
  name: string;
  description?: string;
  icon?: string;
  items?: BackendOptimizationItem[]; // Items from backend
}

export interface BackendOptimizationItem {
  id: string;
  name: string;
  description: string;
  category: string;
  is_applied: boolean;
  is_reversible: boolean;
  requires_admin: boolean;
  risk_level: string;
  platform: string;
}

export interface OptimizationResult {
  success: boolean;
  message: string;
  error?: string;
}

export interface PlatformInfo {
  os: string;
  version: string;
  arch: string;
}

// Pre-defined categories for easier management
export const OPTIMIZATION_CATEGORIES = {
  GAMING: {
    id: "gaming",
    name: "Gaming Performance",
    description: "Optimizations focused on improving gaming performance",
    icon: "videogame_asset",
  },
  SYSTEM: {
    id: "system",
    name: "System Performance",
    description: "General system performance improvements",
    icon: "settings",
  },
  POWER: {
    id: "power",
    name: "Power Management",
    description: "Power and thermal management settings",
    icon: "power",
  },
  NETWORK: {
    id: "network",
    name: "Network Optimization",
    description: "Network performance and latency optimizations",
    icon: "network_check",
  },
  PRIVACY: {
    id: "privacy",
    name: "Privacy & Security",
    description: "Privacy and security related optimizations",
    icon: "security",
  },
} as const;
