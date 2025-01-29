import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import CssBaseline from "@mui/material/CssBaseline";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import "@fontsource/inter/300.css"; // Peso 300
import "@fontsource/inter/400.css"; // Peso 400
import "@fontsource/inter/500.css"; // Peso 500
import "@fontsource/inter/600.css"; // Peso 600
import "@fontsource/inter/700.css"; // Peso 700
const theme = createTheme({
  typography: {
    fontFamily: "Inter, sans-serif", // Imposta Inter come font predefinito
    fontSize: 12, // Riduce la dimensione base del font
    body1: {
      fontSize: "0.85rem", // Testo principale più piccolo
    },
    body2: {
      fontSize: "0.75rem", // Testo secondario ancora più piccolo
    },
    button: {
      fontSize: "0.75rem", // Testo dei pulsanti più piccolo
    },
  },
  palette: {
    mode: "dark",
    primary: { main: "#7aa2f7", contrastText: "#1a1b26" },
    secondary: { main: "#bb9af7", contrastText: "#1a1b26" },
    error: { main: "#f7768e" },
    background: {
      default: "#1c1d2a",
      paper: "#0a0b11",
    },
    text: {
      primary: "#a9b1d6",
      secondary: "#a9b1d6",
    },
  },
});
const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement,
);
root.render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <App />
    </ThemeProvider>
  </React.StrictMode>,
);
