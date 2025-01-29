import React, {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { Box, useMediaQuery, useTheme } from "@mui/material";

interface MasonryLayoutProps {
  children: React.ReactElement[];
  itemMinWidth?: number;
  gap?: number;
}

const MasonryLayout: React.FC<MasonryLayoutProps> = React.memo(
  ({ children, itemMinWidth = 300, gap = 16 }) => {
    const theme = useTheme();
    const containerRef = useRef<HTMLDivElement>(null);
    const resizeObserverRef = useRef<ResizeObserver | null>(null);
    const [columns, setColumns] = useState(1);
    const isDesktop = useMediaQuery(theme.breakpoints.up("md"));
    const rafId = useRef<number | undefined>(undefined);
    // Memoizza il calcolo delle colonne per evitare ricalcoli non necessari
    const calculateColumns = useCallback(() => {
      if (rafId.current) {
        cancelAnimationFrame(rafId.current);
      }
      rafId.current = requestAnimationFrame(() => {
        if (!containerRef.current || !isDesktop) {
          setColumns(1);
          return;
        }
        const containerWidth = containerRef.current.offsetWidth;
        const newColumns = Math.max(
          1,
          Math.floor(containerWidth / (itemMinWidth + gap)),
        );
        setColumns((prev) => {
          // Evita aggiornamenti non necessari se il numero di colonne non è cambiato
          return prev !== newColumns ? newColumns : prev;
        });
      });
    }, [itemMinWidth, gap, isDesktop]);
    // Ottimizza con ResizeObserver invece di window resize per performance migliori
    useEffect(() => {
      if (!containerRef.current) return;
      const container = containerRef.current;
      // Usa ResizeObserver per performance migliori rispetto a window.addEventListener
      resizeObserverRef.current = new ResizeObserver((entries) => {
        for (const entry of entries) {
          if (entry.target === container) {
            calculateColumns();
          }
        }
      });
      resizeObserverRef.current.observe(container);
      // Calcolo iniziale
      calculateColumns();
      return () => {
        if (resizeObserverRef.current) {
          resizeObserverRef.current.disconnect();
        }
        if (rafId.current) {
          cancelAnimationFrame(rafId.current);
        }
      };
    }, [calculateColumns]);
    // Memoizza le props del container per evitare re-render non necessari
    const containerStyles = useMemo(
      () => ({
        display: "grid",
        gridTemplateColumns:
          isDesktop && columns > 1 ? `repeat(${columns}, 1fr)` : "1fr",
        gap: `${gap}px`,
        width: "100%",
        gridAutoFlow: "row", // Mantiene l'ordine sequenziale
        // Ottimizzazioni per il rendering
        willChange: columns > 1 ? "contents" : "auto",
        transform: "translateZ(0)", // Force hardware acceleration
      }),
      [isDesktop, columns, gap],
    );
    // Memoizza i children processati se necessario
    const processedChildren = useMemo(() => {
      return React.Children.map(children, (child, index) => {
        if (!React.isValidElement(child)) return child;
        // Aggiungi key se mancante per performance migliori
        return React.cloneElement(child, {
          key: child.key || `masonry-item-${index}`,
        });
      });
    }, [children]);
    return (
      <Box ref={containerRef} sx={containerStyles}>
        {processedChildren}
      </Box>
    );
  },
);
// Imposta il displayName per debugging
MasonryLayout.displayName = "MasonryLayout";
export default MasonryLayout;
