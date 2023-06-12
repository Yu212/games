import React, {useState, useEffect, useRef, useMemo} from "react"
import {Action, Cell, Grid} from "rust"
import {WorkerType} from "./wasm.worker.ts";
import Konva from "konva";
import {Circle, Group, Layer, Line, Rect, Stage} from "react-konva";

const fix = (broken, clz) => {
    const obj = Object.create(clz.prototype);
    obj.__wbg_ptr = broken.__wbg_ptr;
    return obj;
}

const UltimateTicTacToeCanvas: React.FC<{ worker: WorkerType }> = ({ worker }) => {
    const [grid, setGrid] = useState<Grid>(() => Grid.initial_grid());

    const onClick = (event: Konva.KonvaEventObject<MouseEvent>) => {
        if (!grid.is_player_turn) {
            return;
        }
        const attrs = event.target.attrs;
        console.log("clicked! %d %d %o", attrs.b, attrs.s);
        const action = Action.action(attrs.b, attrs.s);
        const valid = grid.is_valid_action(attrs.b, attrs.s);
        if (valid) {
            setGrid(oldGrid => oldGrid.play(action));
        }
    };

    useEffect(() => {
        console.log(grid.get_big_cell(0), grid.get_small_cell(0, 0), grid.last_big);
        if (!grid.is_player_turn) {
            console.log("ai thinking...");
            worker.aiAction(grid).then(obj => {
                const action = fix(obj, Action);
                console.log("ai result: %o %o", action.x, action.y);
                setGrid(oldGrid => oldGrid.play(action));
            });
        }
    }, [grid, worker]);

    const width = 1280;
    const height = 720;
    const size = Math.min(width, height) - 100;
    return (
        <Stage width={width} height={height}>
            <Layer>
                <Rect fill="#000000" width={width} height={height}/>
            </Layer>
            <Layer imageSmoothingEnabled={true} x={(width-size)/2} y={(height-size)/2} scaleX={size/800} scaleY={size/800}>
                <Rect x={10} y={10} width={780} height={780} strokeWidth={20} cornerRadius={10} stroke="#ffffff"/>
                <Rect x={40} y={260} width={720} height={20} fill="#f9b700"/>
                <Rect x={40} y={520} width={720} height={20} fill="#f9b700"/>
                <Rect x={260} y={40} width={20} height={720} fill="#f9b700"/>
                <Rect x={520} y={40} width={20} height={720} fill="#f9b700"/>
                {Array.from({ length: 9 }, (_, b) => {
                    const bigCell = grid.get_big_cell(b);
                    const bigX = (b % 3 | 0) * 260 + 40;
                    const bigY = (b / 3 | 0) * 260 + 40;
                    if (bigCell == Cell.Player) {
                        return <Circle key={b} x={bigX+100} y={bigY+100} radius={78} strokeWidth={32} stroke="#22a1e4"/>
                    } else if (bigCell == Cell.Ai) {
                        return <Group key={b} x={bigX} y={bigY}>
                            <Line points={[28, 28, 172, 172]} strokeWidth={32} stroke="#f2b213"/>
                            <Line points={[28, 172, 172, 28]} strokeWidth={32} stroke="#f2b213"/>
                        </Group>
                    } else if (bigCell == Cell.Empty) {
                        const boardColor = grid.last_big === undefined || grid.last_big === b ? "#ffffff" : "#848484";
                        return <Group key={b} x={bigX} y={bigY}>
                            <Group>
                                <Rect x={0} y={60} width={200} height={10} fill={boardColor}/>
                                <Rect x={0} y={130} width={200} height={10} fill={boardColor}/>
                                <Rect x={60} y={0} width={10} height={200} fill={boardColor}/>
                                <Rect x={130} y={0} width={10} height={200} fill={boardColor}/>
                            </Group>
                            <Group>
                                {Array.from({ length: 9 }, (_, s) => {
                                    const smallX = (s % 3 | 0) * 70;
                                    const smallY = (s / 3 | 0) * 70;
                                    const smallCell = grid.get_small_cell(b, s);
                                    if (smallCell == Cell.Player) {
                                        return <Circle key={s} x={smallX+30} y={smallY+30} radius={19} strokeWidth={8} stroke="#22a1e4"/>
                                    } else if (smallCell == Cell.Ai) {
                                        return <Group key={s} x={smallX} y={smallY}>
                                            <Line points={[12, 12, 48, 48]} strokeWidth={8} stroke="#f2b213"/>
                                            <Line points={[12, 48, 48, 12]} strokeWidth={8} stroke="#f2b213"/>
                                        </Group>
                                    } else {
                                        return <Rect key={s} attrs={{ b, s }} x={smallX} y={smallY} width={60} height={60} fill="#404040" onClick={onClick}/>
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
