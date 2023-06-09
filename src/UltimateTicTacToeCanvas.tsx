import React, {useState, useEffect, useRef} from "react"
import {Action, Cell, Grid} from "rust"
import {WorkerType} from "./wasm.worker.ts";

const draw = (context: CanvasRenderingContext2D, grid: Grid) => {
    console.log("draw: %o %d", grid);
    const w = context.canvas.width;
    const h = context.canvas.height;
    context.clearRect(0, 0, w, h);
    context.fillStyle = "#000";
    context.fillRect(0, 0, w, h);
    const cell = grid.get_cell(0, 0);
    if (cell === Cell.Empty) {
        context.fillStyle = "#f00";
        context.fillRect(10, 10, 50, 50);
    } else if (cell === Cell.Player) {
        context.fillStyle = "#0f0";
        context.fillRect(10, 10, 50, 50);
    } else {
        context.fillStyle = "#00f";
        context.fillRect(10, 10, 50, 50);
    }
};

const fix = (broken, clz) => {
    const obj = Object.create(clz.prototype);
    obj.__wbg_ptr = broken.__wbg_ptr;
    return obj;
}

interface Props {
    worker: WorkerType;
}

const UltimateTicTacToeCanvas: React.FC<Props> = ({ worker }) => {
    const [grid, setGrid] = useState<Grid>(() => Grid.initial_grid());
    const canvasRef = useRef<HTMLCanvasElement | null>(null);

    const onClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
        const canvas = canvasRef.current;
        const boundingClientRect = canvas.getBoundingClientRect();
        const x = event.clientX - boundingClientRect.left;
        const y = event.clientY - boundingClientRect.top;
        console.log(grid.is_player_turn);
        console.log("clicked! %d %d", x, y);
        const action = Action.action(0, 0);
        console.log(action);
        setGrid(oldGrid => oldGrid.play(action));
    };

    useEffect(() => {
        const canvas = canvasRef.current;
        const context = canvas.getContext("2d");
        draw(context, grid);
        if (!grid.is_player_turn) {
            console.log("ai thinking...");
            worker.aiAction(grid).then(obj => {
                const action = fix(obj, Action);
                console.log("ai result: %o %d %d", action.x, action.y);
                setGrid(oldGrid => oldGrid.play(action));
            });
        }
    }, [grid, worker]);

    return <canvas ref={canvasRef} width={1280} height={720} onClick={onClick} />
}

export default UltimateTicTacToeCanvas;
