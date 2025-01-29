import React from "react";
import { alpha, Box, Chip, IconButton, Tooltip } from "@mui/material";
import { Pause, PlayArrow, Stop } from "@mui/icons-material";
import { keyframes, styled } from "@mui/material/styles";

const pulse = keyframes`
  0% {
    box-shadow: 0 0 0 0 rgba(25, 118, 210, 0.7);
  }
  70% {
    box-shadow: 0 0 0 10px rgba(25, 118, 210, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(25, 118, 210, 0);
  }
`;
const ripple = keyframes`
  0% {
    transform: scale(0.8);
    opacity: 1;
  }
  100% {
    transform: scale(2.4);
    opacity: 0;
  }
`;
const PlayPauseContainer = styled(Box)(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  gap: theme.spacing(1),
  padding: theme.spacing(0.5),
  background: alpha(theme.palette.background.paper, 0.9),
  backdropFilter: "blur(20px)",
  borderRadius: theme.spacing(3),
  border: `1px solid ${alpha(theme.palette.divider, 0.2)}`,
  boxShadow: `0 8px 32px ${alpha(theme.palette.common.black, 0.1)}`,
}));
const StyledIconButton = styled(IconButton, {
  shouldForwardProp: (prop) => prop !== "isActive",
})<{ isActive?: boolean }>(({ theme, isActive }) => ({
  width: 48,
  height: 48,
  borderRadius: "50%",
  position: "relative",
  overflow: "hidden",
  transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
  background: isActive
    ? `linear-gradient(135deg, ${theme.palette.success.main}, ${theme.palette.success.dark})`
    : `linear-gradient(135deg, ${theme.palette.error.main}, ${theme.palette.error.dark})`,
  color: theme.palette.common.white,
  boxShadow: isActive
    ? `0 4px 20px ${alpha(theme.palette.success.main, 0.4)}`
    : `0 4px 20px ${alpha(theme.palette.error.main, 0.4)}`,
  "&:hover": {
    transform: "translateY(-2px) scale(1.05)",
    boxShadow: isActive
      ? `0 8px 25px ${alpha(theme.palette.success.main, 0.5)}`
      : `0 8px 25px ${alpha(theme.palette.error.main, 0.5)}`,
  },
  "&:active": {
    transform: "translateY(0) scale(0.95)",
  },
  "&::before": isActive
    ? {
        content: '""',
        position: "absolute",
        top: "50%",
        left: "50%",
        width: "100%",
        height: "100%",
        transform: "translate(-50%, -50%)",
        background: `radial-gradient(circle, ${alpha(theme.palette.success.light, 0.3)} 0%, transparent 70%)`,
        borderRadius: "50%",
        animation: `${pulse} 2s infinite`,
      }
    : {},
  "&::after": {
    content: '""',
    position: "absolute",
    top: "50%",
    left: "50%",
    width: "100%",
    height: "100%",
    transform: "translate(-50%, -50%)",
    background: alpha(theme.palette.common.white, 0.1),
    borderRadius: "50%",
    opacity: 0,
    transition: "opacity 0.3s ease",
  },
  "&:hover::after": {
    opacity: 1,
    animation: `${ripple} 0.6s ease-out`,
  },
}));
const StatusChip = styled(Chip, {
  shouldForwardProp: (prop) => prop !== "isActive",
})<{ isActive?: boolean }>(({ theme, isActive }) => ({
  height: 28,
  fontSize: "0.75rem",
  fontWeight: 600,
  borderRadius: theme.spacing(1.5),
  background: isActive
    ? `linear-gradient(135deg, ${alpha(theme.palette.success.main, 0.2)}, ${alpha(theme.palette.success.dark, 0.3)})`
    : `linear-gradient(135deg, ${alpha(theme.palette.error.main, 0.2)}, ${alpha(theme.palette.error.dark, 0.3)})`,
  color: isActive ? theme.palette.success.main : theme.palette.error.main,
  border: `1px solid ${isActive ? alpha(theme.palette.success.main, 0.3) : alpha(theme.palette.error.main, 0.3)}`,
  "& .MuiChip-icon": {
    fontSize: "1rem",
  },
}));

interface ModernPlayPauseProps {
  isPlaying: boolean;
  onToggle: () => void;
  disabled?: boolean;
  showLabel?: boolean;
  size?: "small" | "medium" | "large";
  tooltip?: string;
}

const ModernPlayPause: React.FC<ModernPlayPauseProps> = ({
  isPlaying,
  onToggle,
  disabled = false,
  showLabel = true,
  size = "medium",
  tooltip,
}) => {
  const getIcon = () => {
    return isPlaying ? (
      <Pause fontSize={size} />
    ) : (
      <PlayArrow fontSize={size} />
    );
  };
  const getLabel = () => {
    return isPlaying ? "Active" : "Paused";
  };
  const getTooltip = () => {
    if (tooltip) return tooltip;
    return isPlaying ? "Pause updates" : "Resume updates";
  };
  return (
    <PlayPauseContainer>
      <Tooltip title={getTooltip()} arrow placement="top">
        <StyledIconButton
          onClick={onToggle}
          disabled={disabled}
          isActive={isPlaying}
          size={size}
        >
          {getIcon()}
        </StyledIconButton>
      </Tooltip>

      {showLabel && (
        <StatusChip
          icon={isPlaying ? <PlayArrow /> : <Stop />}
          label={getLabel()}
          isActive={isPlaying}
          size="small"
        />
      )}
    </PlayPauseContainer>
  );
};
export default ModernPlayPause;
