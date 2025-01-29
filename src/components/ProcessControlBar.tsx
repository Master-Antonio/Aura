import React from "react";
import { Box } from "@mui/material";
import ModernPagination from "./ModernPagination";
import ModernPlayPause from "./ModernPlayPause";

interface ProcessControlBarProps {
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
  updating: boolean;
  toggleUpdating: () => void;
}

const ProcessControlBar: React.FC<ProcessControlBarProps> = ({
  currentPage,
  totalPages,
  onPageChange,
  updating,
  toggleUpdating,
}) => {
  return (
    <Box
      display="flex"
      justifyContent="space-between"
      alignItems="center"
      sx={{
        p: 2,
        borderRadius: 2,
        backgroundColor: "#1c1d2a",
        //border: '1px solid',
        borderColor: "divider",
        boxShadow: "0 2px 8px rgba(0, 0, 0, 0.05)",
      }}
    >
      {/* Left side - Play/Pause Control */}
      <Box display="flex" alignItems="center">
        <ModernPlayPause
          isPlaying={updating}
          onToggle={toggleUpdating}
          showLabel={true}
          size="medium"
        />
      </Box>

      {/* Center - Pagination */}
      <Box display="flex" justifyContent="center" flexGrow={1}>
        <ModernPagination
          currentPage={currentPage}
          totalPages={totalPages}
          onPageChange={onPageChange}
          showInfo={true}
        />
      </Box>

      {/* Right side - Empty for now, could add other controls */}
      <Box width={120} />
    </Box>
  );
};
export default React.memo(ProcessControlBar);
