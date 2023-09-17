import React, {useEffect, useMemo, useState} from "react";
import UltimateTicTacToeCanvas from "./UltimateTicTacToeCanvas.tsx";
import {WorkerType} from "./wasm.worker.ts";
import {Cell, Turn, Grid} from "rust";

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

const UltimateTicTacToe: React.FC<{ worker: WorkerType, gameId: number, firstPlayer: Turn, showEvals: boolean }> = ({ worker, gameId, firstPlayer, showEvals }) => {
    const initializeGame = () => {
        console.log("initializeGame");
        const game = ({
            gameId: gameId,
            gridId: 0,
            grid: Grid.initial_grid(firstPlayer),
            calculating_evals: false,
        });
        console.log("initialGame: ", game);
        return game;
    };

    const [game, setGame] = useState<Game>(initializeGame);
    console.log("reload");

    const calcEvals = (game: Game) => {
        if (game.evals) {
            return;
        }
        console.log("eval start !!!", game.gridId);
        worker.calcEvals(game.grid).then(evals => {
            console.log("eval end !!!");
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
        console.log("advance: ", game.gameId, game.gridId, game.grid);
        setGame(prev => {
            console.log(prev.grid, game.grid, prev, game);
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
            console.log("restart!", game.grid);
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
            console.log("AI thinking...", ""+gameId, ""+game.grid.last_big);
            worker.aiAction(game.grid).then(obj => {
                const cell = fix(obj, Cell);
                console.log("AI played: %o %o", cell.b, cell.s);
                advance(game, cell);
            });
        }
    }, [game.grid, worker]);

    return <UltimateTicTacToeCanvas game={game} showEvals={showEvals} advance={advance} />;
};

export default UltimateTicTacToe;
