import {Routes, Route} from "react-router-dom";
import "./App.css"
import Home from "./Home.tsx";
import GameSelect from "./GameSelect.tsx";
import UltimateTicTacToe from "./UltimateTicTacToe.tsx";
import React from "react";

const App: React.FC = () => {
    return (
        <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/game-select" element={<GameSelect />} />
            <Route path="/ultimate-tic-tac-toe" element={<UltimateTicTacToe />} />
        </Routes>
    );
}

export default App
