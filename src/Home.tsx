import reactLogo from "/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import React from "react";

const Home: React.FC = () => {
    return (
        <>
            <div>
                <a href="https://vitejs.dev" target="_blank">
                    <img src={viteLogo} className="logo" alt="Vite logo" />
                </a>
                <a href="https://react.dev" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo" />
                </a>
            </div>
            <h1>Game</h1>
            <a href="./game-select">Game Select</a>
        </>
    );
};

export default Home
