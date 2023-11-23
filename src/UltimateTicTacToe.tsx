import React, {useEffect, useState} from "react";
import UltimateTicTacToeCanvas from "./UltimateTicTacToeCanvas.tsx";
import {WorkerType} from "./wasm.worker.ts";
import {Cell, Grid, Turn} from "rust";

const fix = (broken, clz) => {
    const obj = Object.create(clz.prototype);
    obj.__wbg_ptr = broken.__wbg_ptr;
    return obj;
};

export interface Game {
    gameId: number,
    gridId: number,
    grid: Grid,
    calculating_evals: boolean,
    evals?: Float32Array,
}

const UltimateTicTacToe: React.FC<{ worker: WorkerType, gameId: number, firstPlayer: Turn, timeLimit: number, showEvals: boolean }> = ({ worker, gameId, firstPlayer, timeLimit, showEvals }) => {
    const initializeGame = () => {
        return ({
            gameId: gameId,
            gridId: 0,
            grid: Grid.initial_grid(firstPlayer),
            calculating_evals: false,
        });
    };

    const [game, setGame] = useState<Game>(initializeGame);

    const calcEvals = (game: Game) => {
        if (game.evals) {
            return;
        }
        worker.calcEvals(game.grid).then(evals => {
            setGame(prev => {
                console.log(prev.gridId, game.gridId);
                if (prev.gridId == game.gridId) {
                    return { ...prev, calculating_evals: false, evals: evals };
                } else {
                    return prev;
                }
            });
        });
    }

    const advance = (game, cell) => {
        console.log("advance: ", game.gameId, game.gridId);
        setGame(prev => {
            const next = prev.gameId !== game.gameId || prev.gridId !== game.gridId ? prev : {
                gameId: gameId,
                gridId: prev.gridId + 1,
                grid: prev.grid.advance(cell),
                calculating_evals: !prev.grid.is_player_turn && showEvals,
            };
            if (!prev.grid.is_player_turn && showEvals) {
                next.calculating_evals = true;
                calcEvals(next);
            }
            return next;
        });
    }

    useEffect(() => {
        if (game.gameId < gameId) {
            console.log("restart");
            setGame(initializeGame);
        }
    }, [gameId]);

    useEffect(() => {
        if (!showEvals) {
            return;
        }
        setGame(prev => {
            const next = { ...prev, calculating_evals: prev.evals === undefined };
            if (!prev.calculating_evals) {
                calcEvals(next);
            }
            return next;
        });
    }, [showEvals]);

    useEffect(() => {
        if (game.grid.winner !== undefined) {
            return;
        }
        if (!game.grid.is_player_turn) {
            console.log("AI thinking...", gameId);
            worker.aiAction(game.grid, timeLimit).then(obj => {
                const cell = fix(obj, Cell);
                console.log("AI played: %o %o", cell.b, cell.s);
                advance(game, cell);
            });
        }
    }, [game.grid, worker]);

    return <UltimateTicTacToeCanvas width={1280} height={720} game={game} showEvals={showEvals} advance={advance} />;
};

export default UltimateTicTacToe;
