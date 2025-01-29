import { SvgIconComponent } from "@mui/icons-material";

export interface SystemStats {
  icon: SvgIconComponent;
  title: string;
  percentage?: number;
  progress_data?: ProgressData[];
  generic_data?: GenericData[];
}

export interface ProgressData {
  title: string;
  value: number;
  temperature?: number;
}

export interface GenericData {
  title: string;
  value: string;
}
