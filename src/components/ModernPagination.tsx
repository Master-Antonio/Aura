import React from "react";
import {
  alpha,
  Box,
  Chip,
  IconButton,
  Tooltip,
  Typography,
} from "@mui/material";
import {
  ChevronLeft,
  ChevronRight,
  FirstPage,
  LastPage,
} from "@mui/icons-material";
import { styled } from "@mui/material/styles";

const PaginationContainer = styled(Box)(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  gap: theme.spacing(1),
  padding: theme.spacing(1),
  background: alpha(theme.palette.background.paper, 0.8),
  backdropFilter: "blur(10px)",
  borderRadius: theme.spacing(3),
  border: `1px solid ${alpha(theme.palette.divider, 0.3)}`,
  boxShadow: `0 4px 20px ${alpha(theme.palette.common.black, 0.1)}`,
}));
const StyledIconButton = styled(IconButton)(({ theme }) => ({
  width: 36,
  height: 36,
  borderRadius: theme.spacing(1.5),
  transition: "all 0.2s ease-in-out",
  "&:hover": {
    background: alpha(theme.palette.primary.main, 0.1),
    transform: "translateY(-1px)",
    boxShadow: `0 4px 12px ${alpha(theme.palette.primary.main, 0.3)}`,
  },
  "&:disabled": {
    opacity: 0.3,
    transform: "none",
    boxShadow: "none",
  },
}));
const PageChip = styled(Chip, {
  shouldForwardProp: (prop) => prop !== "isActive",
})<{ isActive?: boolean }>(({ theme, isActive }) => ({
  minWidth: 36,
  height: 36,
  borderRadius: theme.spacing(1.5),
  fontWeight: isActive ? 600 : 400,
  cursor: "pointer",
  transition: "all 0.2s ease-in-out",
  ...(isActive
    ? {
        background: `linear-gradient(135deg, ${theme.palette.primary.main}, ${theme.palette.primary.dark})`,
        color: theme.palette.primary.contrastText,
        boxShadow: `0 4px 12px ${alpha(theme.palette.primary.main, 0.4)}`,
        transform: "translateY(-1px)",
      }
    : {
        background: alpha(theme.palette.background.default, 0.5),
        color: theme.palette.text.secondary,
        "&:hover": {
          background: alpha(theme.palette.primary.main, 0.1),
          color: theme.palette.primary.main,
          transform: "translateY(-1px)",
          boxShadow: `0 2px 8px ${alpha(theme.palette.primary.main, 0.2)}`,
        },
      }),
}));
const InfoText = styled(Typography)(({ theme }) => ({
  fontSize: "0.875rem",
  color: theme.palette.text.secondary,
  background: alpha(theme.palette.background.default, 0.7),
  padding: `${theme.spacing(0.5)} ${theme.spacing(1.5)}`,
  borderRadius: theme.spacing(1),
  border: `1px solid ${alpha(theme.palette.divider, 0.3)}`,
}));

interface ModernPaginationProps {
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
  totalItems?: number;
  itemsPerPage?: number;
  showInfo?: boolean;
}

const ModernPagination: React.FC<ModernPaginationProps> = ({
  currentPage,
  totalPages,
  onPageChange,
  totalItems,
  itemsPerPage,
  showInfo = true,
}) => {
  const getVisiblePages = () => {
    const delta = 2;
    const range = [];
    const rangeWithDots = [];
    for (
      let i = Math.max(2, currentPage - delta);
      i <= Math.min(totalPages - 1, currentPage + delta);
      i++
    ) {
      range.push(i);
    }
    if (currentPage - delta > 2) {
      rangeWithDots.push(1, "...");
    } else {
      rangeWithDots.push(1);
    }
    rangeWithDots.push(...range);
    if (currentPage + delta < totalPages - 1) {
      rangeWithDots.push("...", totalPages);
    } else if (totalPages > 1) {
      rangeWithDots.push(totalPages);
    }
    return rangeWithDots;
  };
  const handlePageClick = (page: number | string) => {
    if (typeof page === "number" && page !== currentPage) {
      onPageChange(page);
    }
  };
  if (totalPages <= 1) return null;
  const visiblePages = getVisiblePages();
  return (
    <Box
      display="flex"
      alignItems="center"
      gap={2}
      justifyContent="center"
      flexWrap="wrap"
    >
      <PaginationContainer>
        <Tooltip title="First page">
          <StyledIconButton
            onClick={() => handlePageClick(1)}
            disabled={currentPage === 1}
            size="small"
          >
            <FirstPage fontSize="small" />
          </StyledIconButton>
        </Tooltip>

        <Tooltip title="Previous page">
          <StyledIconButton
            onClick={() => handlePageClick(currentPage - 1)}
            disabled={currentPage === 1}
            size="small"
          >
            <ChevronLeft fontSize="small" />
          </StyledIconButton>
        </Tooltip>

        <Box display="flex" gap={0.5}>
          {visiblePages.map((page, index) => (
            <React.Fragment key={index}>
              {page === "..." ? (
                <Typography
                  variant="body2"
                  sx={{
                    px: 1,
                    py: 1,
                    color: "text.disabled",
                    fontSize: "0.875rem",
                  }}
                >
                  …
                </Typography>
              ) : (
                <PageChip
                  label={page}
                  isActive={page === currentPage}
                  onClick={() => handlePageClick(page)}
                  size="small"
                />
              )}
            </React.Fragment>
          ))}
        </Box>

        <Tooltip title="Next page">
          <StyledIconButton
            onClick={() => handlePageClick(currentPage + 1)}
            disabled={currentPage === totalPages}
            size="small"
          >
            <ChevronRight fontSize="small" />
          </StyledIconButton>
        </Tooltip>

        <Tooltip title="Last page">
          <StyledIconButton
            onClick={() => handlePageClick(totalPages)}
            disabled={currentPage === totalPages}
            size="small"
          >
            <LastPage fontSize="small" />
          </StyledIconButton>
        </Tooltip>
      </PaginationContainer>

      {showInfo && totalItems && itemsPerPage && (
        <InfoText>
          {Math.min(itemsPerPage * (currentPage - 1) + 1, totalItems)} -{" "}
          {Math.min(itemsPerPage * currentPage, totalItems)} of {totalItems}
        </InfoText>
      )}
    </Box>
  );
};
export default ModernPagination;
