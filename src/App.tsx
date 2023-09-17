import {Routes, Route} from "react-router-dom";
import "./App.css";
import Home from "./Home.tsx";
import GameSelect from "./GameSelect.tsx";
import React from "react";
import UltimateTicTacToeContainer from "./UltimateTicTacTieContainer.tsx";

const App: React.FC = () => {
    return (
        <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/game-select" element={<GameSelect />} />
            <Route path="/ultimate-tic-tac-toe" element={<UltimateTicTacToeContainer />} />
        </Routes>
    );
};

export default App
