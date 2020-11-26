import React from 'react';
import './App.css';
import Board from './game/Board'

interface Props {
}

interface State {
}

class App extends React.Component<Props, State> {

    constructor(props: Props) {
        super(props);
        this.state = {}
    }

    render() {
        return (
            <div className="App">
                <header className="App-header">
                    <Board/>
                </header>
            </div>
        );
    }
}

export default App;
