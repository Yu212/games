import React, {useEffect, useMemo, useState} from "react"
import UltimateTicTacToeCanvas from "./UltimateTicTacToeCanvas.tsx";
import init from "rust"
import WasmLoading from "./WasmLoading.tsx";
import * as Comlink from "comlink";
import WasmWorker from "./wasm.worker.ts?worker"
import {WorkerType} from "./wasm.worker.ts"

const UltimateTicTacToe: React.FC = () => {
    const wasmWorker = useMemo<WorkerType>(() => Comlink.wrap(new WasmWorker()) as unknown as WorkerType, []);
    const [isWasmLoaded, setIsWasmLoaded] = useState<boolean>(false);
    const [a, setA] = useState(1);
    console.log(a);
    useEffect(() => {
        (async () => {
            const wasm = await init();
            await wasmWorker.init(wasm.memory);
            setIsWasmLoaded(true);
        })();
    }, [wasmWorker]);
    return (
        <>
            <button onClick={() => {setA(aa => aa + 1)}}>
                hello
            </button>
            <h2>Ultimate Tic-Tac-Toe</h2>
            {isWasmLoaded ? <UltimateTicTacToeCanvas worker={wasmWorker}/> : <WasmLoading />}
        </>
    );
}

export default UltimateTicTacToe;
