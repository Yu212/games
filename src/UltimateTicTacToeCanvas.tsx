import React, {useState, useEffect} from "react"
import {Cell, Turn, Grid} from "rust"
import {WorkerType} from "./wasm.worker.ts";
import Konva from "konva";
import {Circle, Group, Layer, Line, Rect, Stage, Text} from "react-konva";

const fix = (broken, clz) => {
    const obj = Object.create(clz.prototype);
    obj.__wbg_ptr = broken.__wbg_ptr;
    return obj;
}

const UltimateTicTacToeCanvas: React.FC<{ worker: WorkerType }> = ({ worker }) => {
    const [grid, setGrid] = useState<Grid>(() => Grid.initial_grid());
    const [hoveredCell, setHoveredCell] = useState<[number, number] | null>(null);

    const onClick = (event: Konva.KonvaEventObject<MouseEvent>) => {
        if (grid.winner !== undefined || !grid.is_player_turn) {
            return;
        }
        const attrs = event.target.attrs;
        console.log("clicked! %d %d %o", attrs.b, attrs.s);
        const cell = new Cell(attrs.b, attrs.s);
        if (grid.is_valid_action(cell)) {
            setGrid(oldGrid => oldGrid.play(cell));
        }
    };

    useEffect(() => {
        if (grid.winner !== undefined || grid.is_player_turn) {
            return;
        }
        console.log("ai thinking...");
        worker.aiAction(grid).then(obj => {
            const cell = fix(obj, Cell);
            console.log("ai played: %o %o", cell.b, cell.s);
            setGrid(oldGrid => oldGrid.play(cell));
        });
    }, [grid, worker]);

    const width = 1280;
    const height = 720;
    const size = Math.min(width, height) - 100;
    const infoText =
            grid.winner === Turn.Player ? "Player Win!" :
            grid.winner === Turn.Ai ? "AI Win!" :
            grid.is_player_turn ? "Player Turn" : "AI Turn";
    return (
        <Stage width={width} height={height}>
            <Layer>
                <Rect fill="#000000" width={width} height={height} />
                <Text x={50} y={70} fontSize={30} fill="#ffffff" text={infoText} />
            </Layer>
            <Layer x={(width-size)/2} y={(height-size)/2} scaleX={size/800} scaleY={size/800}>
                <Rect x={10} y={10} width={780} height={780} strokeWidth={20} cornerRadius={10} stroke="#ffffff" />
                <Rect x={40} y={260} width={720} height={20} fill="#f9b700" />
                <Rect x={40} y={520} width={720} height={20} fill="#f9b700" />
                <Rect x={260} y={40} width={20} height={720} fill="#f9b700" />
                <Rect x={520} y={40} width={20} height={720} fill="#f9b700" />
                {Array.from({ length: 9 }, (_, b) => {
                    const bigCell = grid.get_big_cell(b);
                    const bigX = (b % 3 | 0) * 260 + 40;
                    const bigY = (b / 3 | 0) * 260 + 40;
                    if (bigCell === Turn.Player) {
                        return <Circle key={b} x={bigX+100} y={bigY+100} radius={76} strokeWidth={32} stroke="#22a1e4" />
                    } else if (bigCell === Turn.Ai) {
                        return <Group key={b} x={bigX} y={bigY}>
                            <Line points={[28, 28, 172, 172]} strokeWidth={32} stroke="#f2b213" />
                            <Line points={[28, 172, 172, 28]} strokeWidth={32} stroke="#f2b213" />
                        </Group>
                    } else {
                        const boardColor = grid.last_big === undefined || grid.last_big === b ? "#ffffff" : "#848484";
                        return <Group key={b} x={bigX} y={bigY}>
                            <Group>
                                <Rect x={0} y={60} width={200} height={10} fill={boardColor} />
                                <Rect x={0} y={130} width={200} height={10} fill={boardColor} />
                                <Rect x={60} y={0} width={10} height={200} fill={boardColor} />
                                <Rect x={130} y={0} width={10} height={200} fill={boardColor} />
                            </Group>
                            <Group>
                                {Array.from({ length: 9 }, (_, s) => {
                                    const smallX = (s % 3 | 0) * 70;
                                    const smallY = (s / 3 | 0) * 70;
                                    const smallCell = grid.get_small_cell(b, s);
                                    if (smallCell === Turn.Player) {
                                        return <Circle key={s} x={smallX+30} y={smallY+30} radius={19} strokeWidth={8} stroke="#22a1e4" />
                                    } else if (smallCell === Turn.Ai) {
                                        return <Group key={s} x={smallX} y={smallY}>
                                            <Line points={[12, 12, 48, 48]} strokeWidth={8} stroke="#f2b213" />
                                            <Line points={[12, 48, 48, 12]} strokeWidth={8} stroke="#f2b213" />
                                        </Group>
                                    } else {
                                        const hovered = hoveredCell !== null && hoveredCell[0] === b && hoveredCell[1] === s;
                                        const color = grid.winner === undefined && grid.is_player_turn && hovered ? "#303030" : "#000000";
                                        return <Rect key={s} attrs={{ b, s }} x={smallX} y={smallY} width={60} height={60} fill={color}
                                                     onClick={onClick} onMouseEnter={() => setHoveredCell([b, s])} onMouseLeave={() => setHoveredCell(null)} />
                                    }
                                })}
                            </Group>
                        </Group>
                    }
                })}
            </Layer>
        </Stage>
    );
}

export default UltimateTicTacToeCanvas;
