export let rows: number
export let cols: number
export let win: number
export let calculate_move: (board: any, moves: number) => number
export let calculate_scores: (board: any, moves: number) => number[]

export async function load() {
    const wasm = await import('wasm')
    rows = wasm.rows()
    cols = wasm.cols()
    win = wasm.wins()
    calculate_move= wasm.calculate_move
    calculate_scores = wasm.calculate_scores
}
