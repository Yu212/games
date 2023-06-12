import init, {Action, Grid} from "rust"
import {expose} from "comlink"

const module = {
    init: async (memory: WebAssembly.Memory): Promise<void> => {
        await init(undefined, memory);
    },
    aiAction: (grid: Grid): Action => {
        grid = fix(grid, Grid);
        return grid.ai_action();
    }
};

export type WorkerType = {
    [K in keyof typeof module]:
        typeof module[K] extends (...args: infer A) => Exclude<infer U, Promise<unknown>> ?
            (...args: A) => Promise<U> : typeof module[K];
};

expose(module);

const fix = (broken, clz) => {
    const obj = Object.create(clz.prototype);
    obj.__wbg_ptr = broken.__wbg_ptr;
    return obj;
}
