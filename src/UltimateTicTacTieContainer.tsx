import React, {useEffect, useMemo, useRef, useState} from "react";
import WasmLoading from "./WasmLoading.tsx";
import {wrap} from "comlink";
import WasmWorker from "./wasm.worker.ts?worker";
import {WorkerType} from "./wasm.worker.ts";
import initWasm, {Turn} from "rust";
import UltimateTicTacToe from "./UltimateTicTacToe.tsx";

const UltimateTicTacToeContainer: React.FC = () => {
    const wasmWorker = useMemo<WorkerType>(() => wrap(new WasmWorker()) as unknown as WorkerType, []);
    const [isWasmLoaded, setIsWasmLoaded] = useState<boolean>(false);
    const [firstPlayer, setFirstPlayer] = useState<Turn>(Turn.Player);
    const [showEvals, setShowEvals] = useState<boolean>(false);
    const [gameId, setGameId] = useState<number>(0);
    console.log("reload container");

    useEffect(() => {
        initWasm()
            .then(wasm => wasmWorker.init(wasm.memory))
            .finally(() => setIsWasmLoaded(true));
    }, [wasmWorker]);

    return (
        <>
            <h2>Ultimate Tic-Tac-Toe</h2>
            {isWasmLoaded ? <>
                <button onClick={() => setShowEvals(val => !val)}> {showEvals ? "show evals" : "hide evals"} </button>
                <button onClick={() => setFirstPlayer(turn => turn == Turn.Player ? Turn.Ai : Turn.Player)}> {firstPlayer == Turn.Player ? "Player first" : "AI first"} </button>
                <button onClick={() => setGameId(gameId => gameId + 1)}>Restart</button>
                <input type="number" defaultValue="100" min="100" step="100"></input>
                <UltimateTicTacToe worker={wasmWorker} gameId={gameId} firstPlayer={firstPlayer} showEvals={showEvals} />
            </> : <WasmLoading />}
        </>
    );
};

export default UltimateTicTacToeContainer;
