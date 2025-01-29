import React from "react";
import { Box, Chip, Grid, LinearProgress, Typography } from "@mui/material";
import { GenericData, ProgressData } from "../data/SystemStats";
import { SvgIconComponent } from "@mui/icons-material";
import BaseCard from "./BaseCard";

export interface StatsCardProps {
  icon: SvgIconComponent;
  title: string;
  percentage?: number;
  progressData?: ProgressData[];
  genericData?: GenericData[];
}

const StatsCard: React.FC<StatsCardProps> = ({
  icon,
  title,
  percentage,
  progressData,
  genericData,
}) => {
  const headerActions = percentage ? (
    <Chip label={`${Math.round(percentage)}%`} color="primary" size="small" />
  ) : undefined;
  return (
    <BaseCard icon={icon} title={title} headerActions={headerActions}>
      {/* Sezione ProgressData */}
      {progressData && progressData.length > 0 && (
        <Grid container spacing={1} sx={{ mb: 1 }}>
          {progressData.map((progress, idx) => (
            <Grid size={12} key={idx}>
              <Box display="flex" alignItems="center">
                {/* Nome del parametro */}
                <Typography
                  variant="body2"
                  sx={{ minWidth: "80px", fontSize: "0.875rem" }}
                >
                  {progress.title}
                </Typography>
                {/* Barra di progresso */}
                <Box sx={{ flexGrow: 1, mx: 1 }}>
                  <LinearProgress
                    variant="determinate"
                    value={progress.value}
                    sx={{ height: 6, borderRadius: 2 }}
                  />
                </Box>
                {/* Percentuale */}
                <Typography
                  variant="body2"
                  sx={{
                    fontSize: "0.875rem",
                    minWidth: "40px",
                    textAlign: "right",
                  }}
                >
                  {Math.round(progress.value)}%
                </Typography>
              </Box>
            </Grid>
          ))}
        </Grid>
      )}

      {/* Sezione GenericData */}
      {genericData && genericData.length > 0 && (
        <Grid container spacing={1}>
          {genericData.map((info, idx) => (
            <Grid size={12} key={idx}>
              <Typography variant="body2" sx={{ fontSize: "0.875rem" }}>
                <strong>{info.title}:</strong> {info.value}
              </Typography>
            </Grid>
          ))}
        </Grid>
      )}
    </BaseCard>
  );
};
export default StatsCard;
