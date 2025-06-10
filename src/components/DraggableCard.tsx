import React, { useEffect, useRef } from "react";
import { useDrag, useDrop } from "react-dnd";
import { alpha, Box } from "@mui/material";
import { styled } from "@mui/material/styles";

const DraggableContainer = styled(Box, {
  shouldForwardProp: (prop) => prop !== "isDragging" && prop !== "canDrop",
})<{ isDragging?: boolean; canDrop?: boolean }>(
  ({ theme, isDragging, canDrop }) => ({
    cursor: "grab",
    opacity: isDragging ? 0.6 : 1,
    transform: isDragging ? "rotate(2deg) scale(1.02)" : "none",
    transition: "all 0.2s ease",
    position: "relative",
    zIndex: isDragging ? 1000 : "auto",
    boxShadow: isDragging
      ? `0 8px 32px ${alpha(theme.palette.primary.main, 0.3)}`
      : "none",
    "&:hover": {
      transform: isDragging ? "rotate(2deg) scale(1.02)" : "translateY(-1px)",
      boxShadow: isDragging
        ? `0 8px 32px ${alpha(theme.palette.primary.main, 0.3)}`
        : `0 4px 16px ${alpha(theme.palette.common.black, 0.1)}`,
      "& .drag-handle": {
        opacity: 1,
      },
    },
    "&:active": {
      cursor: "grabbing",
    },
    ...(canDrop && {
      "&::before": {
        content: '""',
        position: "absolute",
        top: -2,
        left: -2,
        right: -2,
        bottom: -2,
        background: `linear-gradient(45deg, ${alpha(theme.palette.primary.main, 0.2)}, ${alpha(theme.palette.secondary.main, 0.2)})`,
        borderRadius: theme.spacing(2.5),
        zIndex: -1,
        opacity: 0.5,
      },
    }),
  }),
);
const DragHandle = styled(Box)(({ theme }) => ({
  position: "absolute",
  top: theme.spacing(1),
  right: theme.spacing(1),
  width: 24,
  height: 24,
  opacity: 0.8,
  transition: "all 0.2s ease",
  background: alpha(theme.palette.primary.main, 0.15),
  borderRadius: "50%",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  cursor: "grab",
  zIndex: 10,
  pointerEvents: "auto", // Allow drag interaction
  "&:hover": {
    opacity: 1,
    background: alpha(theme.palette.primary.main, 0.25),
    transform: "scale(1.1)",
  },
  "&:active": {
    cursor: "grabbing",
    transform: "scale(1.05)",
  },
  "&::before": {
    content: '"⋮⋮"',
    fontSize: "12px",
    color: theme.palette.primary.main,
    fontWeight: "bold",
    lineHeight: 1,
  },
}));
// Wrapper per forwardRef su DragHandle
const DragHandleWithRef = React.forwardRef<
  HTMLDivElement,
  React.ComponentProps<typeof DragHandle>
>((props, ref) => <DragHandle {...props} ref={ref} />);
DragHandleWithRef.displayName = "DragHandleWithRef";

interface DraggableCardProps {
  id: string;
  index: number;
  children: React.ReactNode;
  onMove: (dragIndex: number, hoverIndex: number) => void;
  className?: string;
}

const DraggableCard: React.FC<DraggableCardProps> = React.memo(
  ({ id, index, children, onMove, className }) => {
    const ref = useRef<HTMLDivElement>(null);
    const handleRef = useRef<HTMLDivElement>(null);
    const [{ isDragging }, drag] = useDrag({
      type: "CARD",
      item: () => {
        console.log(`[DND] Starting drag for card: ${id} at index: ${index}`);
        return { id, index };
      },
      collect: (monitor) => ({
        isDragging: monitor.isDragging(),
      }),
      canDrag: true,
    });
    const [{ canDrop, isOver }, drop] = useDrop({
      accept: "CARD",
      hover: (item: { id: string; index: number }, monitor) => {
        if (!ref.current) return;
        if (item.index === index) return;
        // Get the client offset of the mouse
        const clientOffset = monitor.getClientOffset();
        if (!clientOffset) return;
        // Get the bounding rectangle of the drop target
        const hoverBoundingRect = ref.current.getBoundingClientRect();
        // Get the vertical middle of the drop target
        const hoverMiddleY =
          (hoverBoundingRect.bottom - hoverBoundingRect.top) / 2;
        // Get the mouse position relative to the drop target
        const hoverClientY = clientOffset.y - hoverBoundingRect.top; // Only perform the move when the mouse has crossed half of the item's height
        // When dragging downwards, only move when the cursor is below 50%
        // When dragging upwards, only move when the cursor is above 50%
        if (item.index < index && hoverClientY > hoverMiddleY) return;
        if (item.index > index && hoverClientY < hoverMiddleY) return;
        console.log(`[DND] Moving card from ${item.index} to ${index}`);
        onMove(item.index, index);
        item.index = index;
      },
      collect: (monitor) => ({
        canDrop: monitor.canDrop(),
        isOver: monitor.isOver(),
      }),
    });
    useEffect(() => {
      if (handleRef.current) drag(handleRef.current);
    }, [drag]);
    // Combine refs for drag, drop and DOM
    drop(ref); // Drop solo sul container, drag solo sull'handle
    const containerProps = {
      isDragging,
      canDrop: canDrop && isOver,
    };
    return (
      <DraggableContainer
        ref={ref}
        {...containerProps}
        className={className}
        onMouseDown={() => console.log(`[DND] Mouse down on card: ${id}`)}
      >
        <Box sx={{ height: "100%", position: "relative" }}>
          <DragHandle
            className="drag-handle"
            ref={handleRef}
            onMouseDown={() => {
              console.log(`[DND] MouseDown su DragHandle card: ${id}`);
              handleRef.current && (handleRef.current.style.background = "red");
              setTimeout(() => {
                if (handleRef.current) handleRef.current.style.background = "";
              }, 300);
            }}
            onClick={() => {
              console.log(`[DND] Click su DragHandle card: ${id}`);
              handleRef.current &&
                (handleRef.current.style.background = "orange");
              setTimeout(() => {
                if (handleRef.current) handleRef.current.style.background = "";
              }, 300);
            }}
          />
          {children}
        </Box>
      </DraggableContainer>
    );
  },
);
// Imposta il displayName per debugging
DraggableCard.displayName = "DraggableCard";
export default DraggableCard;
