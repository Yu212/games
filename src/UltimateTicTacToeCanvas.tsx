import React, {useState} from "react";
import {Cell, Turn} from "rust";
import Konva from "konva";
import {Circle, Group, Layer, Line, Rect, Stage, Text} from "react-konva";
import {Game} from "./UltimateTicTacToe.tsx";

const UltimateTicTacToeCanvas: React.FC<{ game: Game, showEvals: boolean, advance: (game: Game, cell: Cell) => void }> = ({ game, showEvals, advance }) => {
    const [hoveredCell, setHoveredCell] = useState<[number, number] | null>(null);

    const canAdvanceByPlayer = game.grid.winner === undefined && !game.calculating_evals && game.grid.is_player_turn;

    console.log("canvas reload");
    const onClick = (event: Konva.KonvaEventObject<MouseEvent>) => {
        if (!canAdvanceByPlayer) {
            return;
        }
        const attrs = event.target.attrs;
        console.log("clicked! %d %d %o", attrs.b, attrs.s, event.target);
        const cell = new Cell(attrs.b, attrs.s);
        if (game.grid.is_valid_action(cell)) {
            advance(game, cell);
        }
    };

    let bestEval = -Infinity;
    if (showEvals && game.evals) {
        for (let b = 0; b < 9; b++) {
            for (let s = 0; s < 9; s++) {
                if (game.grid.is_valid_action(new Cell(b, s))) {
                    bestEval = Math.max(bestEval, game.evals[b * 9 + s]);
                }
            }
        }
    }

    const width = 1280;
    const height = 720;
    const size = Math.min(width, height) - 100;
    let infoText =
        game.grid.winner === Turn.Player ? "Player Win!" :
        game.grid.winner === Turn.Ai ? "AI Win!" :
        game.grid.is_player_turn ? "Player Turn" : "AI Turn";
    if (game.calculating_evals) {
        infoText += "\nevals calculating...";
    }
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
                    const bigCell = game.grid.get_big_cell(b);
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
                        const isBoardActive = game.grid.winner === undefined && (game.grid.last_big === undefined || game.grid.last_big === b);
                        const boardColor = isBoardActive ? "#ffffff" : "#848484";
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
                                    const smallCell = game.grid.get_small_cell(b, s);
                                    if (smallCell === Turn.Player) {
                                        return <Circle key={s} x={smallX+30} y={smallY+30} radius={19} strokeWidth={8} stroke="#22a1e4" />
                                    } else if (smallCell === Turn.Ai) {
                                        return <Group key={s} x={smallX} y={smallY}>
                                            <Line points={[12, 12, 48, 48]} strokeWidth={8} stroke="#f2b213" />
                                            <Line points={[12, 48, 48, 12]} strokeWidth={8} stroke="#f2b213" />
                                        </Group>
                                    } else {
                                        const cell = new Cell(b, s);
                                        const canHover = canAdvanceByPlayer && game.grid.is_valid_action(cell);
                                        const hovered = canHover && hoveredCell !== null && hoveredCell[0] === b && hoveredCell[1] === s;
                                        const color = game.grid.winner === undefined && game.grid.is_player_turn && hovered ? "#303030" : "#000000";
                                        const showEval = canHover && showEvals && game.evals;
                                        if (canHover) {
                                            return <Group key={s} x={smallX} y={smallY} >
                                                <Rect width={60} height={60} fill={color} attrs={{ b, s }} onClick={onClick} onMouseEnter={() => setHoveredCell([b, s])} onMouseLeave={() => setHoveredCell(null)} />
                                                {showEval && <Text text={game.evals[b * 9 + s].toFixed(2)} fill={bestEval == game.evals[b * 9 + s] ? "red" : "white"} align="center" verticalAlign="middle" width={60} height={60} fontSize={20} attrs={{ b, s }} onClick={onClick} onMouseEnter={() => setHoveredCell([b, s])} onMouseLeave={() => setHoveredCell(null)} />}
                                            </Group>
                                        } else {
                                            return <Group key={s} x={smallX} y={smallY} >
                                                <Rect width={60} height={60} fill={color} />
                                                {showEval && <Text text={game.evals[b * 9 + s].toFixed(2)} fill={bestEval == game.evals[b * 9 + s] ? "red" : "white"} align="center" verticalAlign="middle" width={60} height={60} fontSize={20} />}
                                            </Group>
                                        }
                                    }
                                })}
                            </Group>
                        </Group>
                    }
                })}
            </Layer>
        </Stage>
    );
};

export default UltimateTicTacToeCanvas;
