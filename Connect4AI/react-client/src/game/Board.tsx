import React from "react";
import Game, {Token, TokenToColor} from "./Game";
import {rows, cols, calculate_scores, calculate_move} from '../WASM'
import './Board.css'
import logo from '../logo.svg'


interface Props {
}

interface State {
    board: Token[][],
    enabled: boolean,
    scores: number[]
}

class Board extends React.Component<Props, State> {

    private game: Game;

    constructor(props: Props) {
        super(props);

        this.game = new Game()

        this.state = {
            board: this.game.board,
            enabled: true,
            scores: []
        }
    }

    transpose(): Token[][] {
        let transpose: Token[][] = []
        for (let c = 0; c < cols; c++) {
            transpose.push([])
            for (let r = 0; r < rows; r++) {
                transpose[c].push(this.state.board[r][c])
            }
        }
        return transpose
    }

    won() {
        alert(`${this.game.playerName} won!`)
    }

    async play(col: number): Promise<boolean> {
        let inserted = this.game.play(col);
        if (inserted) {
            this.setState({board: this.game.board})
            let won = this.game.check_winner();
            if (won) {
                this.won();
                return false;
            } else {
                this.game.togglePlayer()
                return true;
            }
        }
        return false;
    }

    render() {
        return (
            <div>
                <h1>Connect4 AI</h1>
                <div className="Row">
                    <p>Current Player: </p>
                    <div className="CurrentPlayer" style={{backgroundColor: TokenToColor(this.game.token)}}/>
                    {!this.state.enabled && <div>
                        <p>AI Thinking</p>
                        <img src={logo} className="App-logo" alt="logo"/>
                    </div>}
                </div>
                <div className='Row'>
                    <div className='Board'>
                        {this.transpose().map((col, c) => (
                            <div className='Column' onClick={() => {
                                if (this.state.enabled) {
                                    this.setState({enabled: false}, () => {
                                        this.play(c).then((cont) => {
                                            if (cont) {
                                                let ai = calculate_move(this.game.board, 12);
                                                this.play(ai).then((c) => {
                                                    if (c) {
                                                        this.setState({enabled: true})
                                                    }
                                                })
                                            } else {
                                                this.setState({enabled: true})
                                            }
                                        })
                                    })
                                }
                            }}>
                                {col.map((tok, r) => (<TokenView r={r} c={c} tok={tok}/>))}
                            </div>
                        ))}
                    </div>
                    <div className='Column'>
                        {this.state.scores.map((val, index) => (
                            <p>COL {index + 1}: {val}</p>
                        ))}
                    </div>
                </div>
            </div>)
    }
}

interface TokenProps {
    tok: Token,
    r: number,
    c: number
}

interface TokenState {
}

class TokenView extends React.Component<TokenProps, TokenState> {

    render() {
        return <div key={`${this.props.r}, ${this.props.c}`}
                    className='Token'
                    style={{backgroundColor: TokenToColor(this.props.tok)}}/>
    }

}

export default Board;
