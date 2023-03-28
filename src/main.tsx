import React from "react";
import ReactDOM from "react-dom/client";
import { Viewer } from "resium";
import App from "./App";
import { init } from "satellite-rs";

console.log("Initalizing Logging");
init();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <App />
);
