import { createRoot } from "react-dom/client";
import App from "./App";

window.addEventListener("dragover", (event) => event.preventDefault());
window.addEventListener("drop", (event) => event.preventDefault());

createRoot(document.getElementById("root")!).render(<App />);
