import React from "react";
import {
  Box,
  Card,
  CardContent,
  Divider,
  Icon,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { SvgIconComponent } from "@mui/icons-material";

const StyledCard = styled(Card)(({ theme }) => ({
  backgroundColor: theme.palette.background.paper,
  borderRadius: theme.spacing(2),
  border: `1px solid ${theme.palette.divider}`,
  transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
  position: "relative",
  overflow: "hidden",
  "&::before": {
    content: '""',
    position: "absolute",
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    background: `linear-gradient(135deg, ${theme.palette.primary.main}10, ${theme.palette.secondary.main}05)`,
    opacity: 0,
    transition: "opacity 0.3s ease",
    zIndex: 0,
  },
  "&:hover": {
    transform: "translateY(-4px)",
    boxShadow: `0 12px 40px ${theme.palette.common.black}15`,
    "&::before": {
      opacity: 1,
    },
  },
}));

interface BaseCardProps {
  icon?: SvgIconComponent;
  title: string;
  children: React.ReactNode;
  headerActions?: React.ReactNode;
}

const BaseCard: React.FC<BaseCardProps> = ({
  icon,
  title,
  children,
  headerActions,
}) => {
  return (
    <StyledCard>
      <CardContent sx={{ position: "relative", zIndex: 1 }}>
        <Box display="flex" alignItems="center" mb={1}>
          {icon && (
            <Icon
              component={icon}
              sx={{
                mr: 1,
                color: "primary.main",
                fontSize: "1.5rem",
              }}
            />
          )}
          <Typography variant="h6" fontWeight="600" sx={{ fontSize: "1rem" }}>
            {title}
          </Typography>
          {headerActions && <Box sx={{ ml: "auto" }}>{headerActions}</Box>}
        </Box>
        <Divider sx={{ mb: 2 }} />
        {children}
      </CardContent>
    </StyledCard>
  );
};
export default BaseCard;
