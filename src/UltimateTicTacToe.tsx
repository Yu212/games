import React, {useEffect, useMemo, useState} from "react"
import UltimateTicTacToeCanvas from "./UltimateTicTacToeCanvas.tsx";
import initWasm from "rust"
import WasmLoading from "./WasmLoading.tsx";
import {wrap} from "comlink";
import WasmWorker from "./wasm.worker.ts?worker"
import {WorkerType} from "./wasm.worker.ts"

const UltimateTicTacToe: React.FC = () => {
    const wasmWorker = useMemo<WorkerType>(() => wrap(new WasmWorker()) as unknown as WorkerType, []);
    const [isWasmLoaded, setIsWasmLoaded] = useState<boolean>(false);
    const [a, setA] = useState<number>(1);
    console.log(a);
    useEffect(() => {
        initWasm()
            .then(wasm => wasmWorker.init(wasm.memory))
            .then(() => setIsWasmLoaded(true));
    }, [wasmWorker]);
    return (
        <>
            <button onClick={() => {setA(aa => aa + 1)}}>
                hello
            </button>
            <h2>Ultimate Tic-Tac-Toe</h2>
            {isWasmLoaded ? <UltimateTicTacToeCanvas worker={wasmWorker} /> : <WasmLoading />}
        </>
    );
}

export default UltimateTicTacToe;
