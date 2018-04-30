import * as React from "react";
import * as ReactDOM from "react-dom";

import AppState from './appState';
import Communicator from './services/communicator';
import All from './components/all';

interface State {
    foreignState: AppState;
    currentView: JSX.Element;
    source: string;
    destination: string;
}

class AppContainer extends React.Component<{}, State> {
    private comm: Communicator;
    constructor(props) {
        super(props);
        this.state = {
            foreignState: new AppState([], '', ''),
            currentView: (<All 
            source={this.state.source}
            destination={this.state.destination}
            pages={this.state.foreignState.portfolio}
            sourceChanged={p => {}}
            destChange={p => {}} 
            pageSelected={p => {}}
            generateSite={() => {}}
            updateRequested={() => {}}
            />),
            source: '',
            destination: '',
        };
        this.comm = new Communicator(
            (s: AppState) => {
                this.setState({foreignState: s})
            }
        );
    }

    render() {
        return this.state.currentView;
    }


}
window.addEventListener('DOMContentLoaded', () => {
    ReactDOM.render(
        <AppContainer />,
        document.querySelector("main")

    );
});