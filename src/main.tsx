import React from "react";
import ReactDOM from "react-dom/client";
import {Provider} from "@/components/ui/provider";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import App from "./App";
import { Panel } from "./components/Panel/Panel"
import { ScreenshotBay } from "./components/ScreenshotBay/ScreenshotBay"
import { ScreenshotAnnotationBay } from "./components/StorageBay/StorageBay"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <BrowserRouter>
            <Provider>
                <Routes>
                    <Route path="/" element={<App/>}/>
                    <Route path="/panel" element={<Panel/>}/>
                    <Route path="/screenshot-bay" element={<ScreenshotBay/>}/>
                    <Route path="/storage-bay" element={<ScreenshotAnnotationBay/>}/>
                </Routes>
            </Provider>
        </BrowserRouter>
    </React.StrictMode>
);
