import React, {useState} from "react";
import {Cell, Turn} from "rust";
import {Circle, Group, Layer, Line, Rect, Stage, Text} from "react-konva";
import {Game} from "./UltimateTicTacToe.tsx";

const UltimateTicTacToeCanvas: React.FC<{ width: number, height: number, game: Game, showEvals: boolean, advance: (game: Game, cell: Cell) => void }> = ({ width, height, game, showEvals, advance }) => {
    const [hoveredCell, setHoveredCell] = useState<number>(-1);

    const canAdvanceByPlayer = game.grid.winner === undefined && !game.calculating_evals && game.grid.is_player_turn;

    console.log("reload canvas");
    const onClick = (b: number, s: number) => {
        if (!canAdvanceByPlayer) {
            return;
        }
        const cell = new Cell(b, s);
        console.log("clicked: %d %d %o", b, s);
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

    const size = Math.min(width, height) - 100;

    const InfoText: React.FC = () => {
        let infoText =
            game.grid.winner === Turn.Player ? "Player Win!" :
            game.grid.winner === Turn.Ai ? "AI Win!" :
            game.grid.is_player_turn ? "Player Turn" : "AI Turn";
        if (game.calculating_evals) {
            infoText += "\nevals calculating...";
        }
        return <Text x={50} y={70} fontSize={30} fill="#ffffff" text={infoText} />;
    };
    const X: React.FC<{ cx: number, cy: number, scale: number }> = ({ cx, cy, scale }) => (
        <Group x={cx} y={cy}>
            <Line points={[-18*scale, -18*scale, 18*scale, 18*scale]} strokeWidth={8 * scale} stroke="#f2b213" />
            <Line points={[-18*scale, 18*scale, 18*scale, -18*scale]} strokeWidth={8 * scale} stroke="#f2b213" />
        </Group>
    );
    const O: React.FC<{ cx: number, cy: number, scale: number }> = ({ cx, cy, scale }) => (
        <Circle x={cx} y={cy} radius={19 * scale} strokeWidth={8 * scale} stroke="#22a1e4" />
    );
    const BigBoard: React.FC<{ b: number }> = ({ b }) => {
        const bigCell = game.grid.get_big_cell(b);
        const bigX = (b % 3 | 0) * 260 + 40;
        const bigY = (b / 3 | 0) * 260 + 40;
        if (bigCell === Turn.Player) {
            return <O cx={bigX+100} cy={bigY+100} scale={4} />
        } else if (bigCell === Turn.Ai) {
            return <X cx={bigX+100} cy={bigY+100} scale={4} />
        } else {
            const isBoardActive = game.grid.winner === undefined && (game.grid.last_big === undefined || game.grid.last_big === b);
            const boardColor = isBoardActive ? "#ffffff" : "#848484";
            return <Group x={bigX} y={bigY}>
                <Group>
                    <Rect x={0} y={60} width={200} height={10} fill={boardColor} />
                    <Rect x={0} y={130} width={200} height={10} fill={boardColor} />
                    <Rect x={60} y={0} width={10} height={200} fill={boardColor} />
                    <Rect x={130} y={0} width={10} height={200} fill={boardColor} />
                </Group>
                <Group>
                    {Array.from({ length: 9 }, (_, s) => <SmallBoard key={s} b={b} s={s} />)}
                </Group>
            </Group>
        }
    };
    const SmallBoard: React.FC<{ b: number, s: number }> = ({ b, s }) => {
        const smallX = (s % 3 | 0) * 70;
        const smallY = (s / 3 | 0) * 70;
        const smallCell = game.grid.get_small_cell(b, s);
        if (smallCell === Turn.Player) {
            return <O cx={smallX+30} cy={smallY+30} scale={1} />
        } else if (smallCell === Turn.Ai) {
            return <X cx={smallX+30} cy={smallY+30} scale={1} />
        } else {
            const canHover = canAdvanceByPlayer && game.grid.is_valid_action(new Cell(b, s));
            const hovered = canHover && hoveredCell === b * 9 + s;
            const color = game.grid.winner === undefined && game.grid.is_player_turn && hovered ? "#303030" : "#000000";
            const showEval = canHover && showEvals && game.evals;
            return <Group x={smallX} y={smallY}
                          onPointerClick={() => canHover && onClick(b, s)}
                          onPointerEnter={() => canHover && setHoveredCell(b * 9 + s)}
                          onPointerOut={() => canHover && setHoveredCell(-1)} >
                <Rect width={60} height={60} fill={color} />
                {showEval && <Text text={game.evals[b * 9 + s].toFixed(2)} fill={bestEval == game.evals[b * 9 + s] ? "red" : "white"} align="center" verticalAlign="middle" width={60} height={60} fontSize={20} />}
            </Group>
        }
    };

    return (
        <Stage width={width} height={height}>
            <Layer>
                <Rect fill="#000000" width={width} height={height} />
                <InfoText />
            </Layer>
            <Layer x={(width-size)/2} y={(height-size)/2} scaleX={size/800} scaleY={size/800}>
                <Rect x={10} y={10} width={780} height={780} strokeWidth={20} cornerRadius={10} stroke="#ffffff" />
                <Rect x={40} y={260} width={720} height={20} fill="#f9b700" />
                <Rect x={40} y={520} width={720} height={20} fill="#f9b700" />
                <Rect x={260} y={40} width={20} height={720} fill="#f9b700" />
                <Rect x={520} y={40} width={20} height={720} fill="#f9b700" />
                {Array.from({ length: 9 }, (_, b) => <BigBoard key={b} b={b} />)}
            </Layer>
        </Stage>
    );
};

export default UltimateTicTacToeCanvas;
