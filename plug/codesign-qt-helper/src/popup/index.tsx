import "./styles.css"
import React from 'react';
import ReactDOM from 'react-dom/client';
import HistoryPage from "./page/HistoryPage";
import DialogPage from "./page/DialogPage";
import {HashRouter, Route, Routes} from "react-router-dom";
import ConfigPage from "./page/ConfigPage";
import HomePage from "./page/HomePage";

const Popup: React.FC = () => {
    return (
        <HashRouter>
            <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="/history" element={<HistoryPage />} />
                <Route path="/config" element={<ConfigPage />} />
                <Route path="/dialog/:downloadId" element={<DialogPage />} />
            </Routes>
        </HashRouter>
    )
};

const root = ReactDOM.createRoot(document.getElementById('root')!);
root.render(<Popup />);