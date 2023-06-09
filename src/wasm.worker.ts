import init, {Action, Cell, Grid} from "rust"
import * as Comlink from "comlink"

const module = {
    init: async (memory: WebAssembly.Memory): Promise<void> => {
        await init(undefined, memory);
    },
    aiAction: (grid: Grid): Action => {
        grid = fix(grid, Grid);
        console.log(grid);
        const action = grid.ai_action();
        console.log("! %o %o", grid, action);
        return action;
    }
};

export type WorkerType = {
    [K in keyof typeof module]:
        typeof module[K] extends (...args: infer A) => Exclude<infer U, Promise<unknown>> ?
            (...args: A) => Promise<U> : typeof module[K];
};

Comlink.expose(module);

const fix = (broken, clz) => {
    const obj = Object.create(clz.prototype);
    obj.__wbg_ptr = broken.__wbg_ptr;
    return obj;
}
