import {cols, rows, win} from "../WASM";
import '../Extensions'

export enum Token {
    red, yellow, none
}

export enum Player {
    ai, human
}

export function TokenToColor(tok: Token) {
    switch (tok) {
        case Token.none:
            return '#fff'
        case Token.red:
            return '#ff5252'
        case Token.yellow:
            return '#ffea00'
    }
}

export function PlayerToToken(player: Player) {
    switch (player) {
        case Player.ai:
            return Token.red
        case Player.human:
            return Token.yellow
    }
}

export function PlayerToText(player: Player) {
    switch (player) {
        case Player.ai:
            return "AI"
        case Player.human:
            return "Human"
    }
}

class Game {

    public board: Token[][]
    public currentPlayer: Player

    private heights: number[]

    constructor() {
        let board: Token[][] = []
        for (let r = 0; r < rows; r++) {
            board.push([])
            for (let c = 0; c < cols; c++) {
                board[r].push(Token.none)
            }
        }

        this.board = board
        this.currentPlayer = Player.human
        this.heights = Array.repeat(0, cols)
    }

    public get token(): Token {
        return PlayerToToken(this.currentPlayer)
    }

    public get playerName(): string {
        return PlayerToText(this.currentPlayer)
    }

    public togglePlayer() {
        if (this.currentPlayer === Player.human) {
            this.currentPlayer = Player.ai
        } else {
            this.currentPlayer = Player.human
        }
    }

    /**
     * plays selected column with current player
     * @param col column to choose
     * @return returns if the current player won
     */
    public play(col: number): boolean {
        if (col >= cols) {
            throw new Error("invalid column number")
        } else if (this.heights[col] >= rows) {
            return false;
        }

        // set token
        this.board[rows - this.heights[col] - 1][col] = this.token

        // add to colHeights
        this.heights[col]++;

        return true;
    }

    public check_winner(): boolean {
        let tok = this.token

        for (let r = 0; r < rows; r++) {
            for (let c = 0; c < cols; c++) {
                if ((r > rows - win && c > cols - win) || this.board[r][c] !== tok) {
                    // no need to check through positions
                    continue;
                }
                let ct = 1;

                // check right
                if (c <= cols - win) {
                    for (let i = 1; i < win; i++) {
                        if (this.board[r][c + i] !== tok) {
                            break;
                        }
                        ct++;
                    }


                    if (ct === win) {
                        return true;
                    }
                }

                // check down
                if (r <= rows - win) {
                    ct = 1;
                    for (let i = 1; i < win; i++) {
                        if (this.board[r + i][c] !== tok) {
                            break;
                        }
                        ct++;
                    }

                    if (ct === win) {
                        return true;
                    }
                }

                // check r-d diagonal
                if (r <= rows - win && c <= cols - win) {
                    ct = 1;
                    for (let i = 1; i < win; i++) {
                        if (this.board[r + i][c + i] !== tok) {
                            break;
                        }
                        ct++;
                    }

                    if (ct === win) {
                        return true;
                    }
                }

                // check r-u diagonal
                if (r >= win - 1 && c <= cols - win) {
                    ct = 1;
                    for (let i = 1; i < win; i++) {
                        if (this.board[r - i][c + i] !== tok) {
                            break;
                        }
                        ct++;
                    }

                    if (ct === win) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

}

export default Game
