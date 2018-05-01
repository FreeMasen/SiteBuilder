import * as React from "react";
import * as ReactDOM from "react-dom";

import Website, {Project, Meta} from './appState';
import Communicator from './services/communicator';

import All from './components/all';
import PageEditor from './components/page';

interface State {
    website: Website;
    currentView: Route;
    source: string;
    destination: string;
    selectedProject?: Project;
}

enum Route {
    All,
    Project,
}

class AppContainer extends React.Component<{}, State> {
    private comm: Communicator;
    constructor(props) {
        super(props);
        let website = new Website();
        this.state = {
            website,
            currentView: Route.All,
            source: '',
            destination: '',
            selectedProject: null,
        };
        this.comm = new Communicator(s => this.communicatorCallback(s))
    }

    communicatorCallback(w: Website) {
        console.log('communicator callback', w);
        this.setState((prev, props) => {
            return {
                website: w
            }
        })
    }

    selectPage(p: Project) {
        this.setState((prev, props) => {
            return {
                selectedProject: p,
                currentView: Route.Project,
            }
        })
    }

    gotoAll() {
        this.setState((prev, props) => {
            return {
                selectedProject: null,
                currentView: Route.All,
            }
        })
    }

    render() {
        console.log('AppContainer.render', this.state);
        switch (this.state.currentView) {
            case Route.All:
                return (
                    <All
                        source={this.state.source}
                        destination={this.state.destination}
                        pages={this.state.website.portfolio}
                        sourceChanged={p => {}}
                        destChange={p => {}}
                        pageSelected={p => this.selectPage(p)}
                        generateSite={() => {}}
                        updateRequested={() => {}}
                    />
                );
            case Route.Project:
                return (
                    <PageEditor
                        title={this.state.selectedProject.meta.title}
                        subtitle={this.state.selectedProject.meta.subtitle}
                        teammates={this.state.selectedProject.meta.teammates}
                        content={this.state.selectedProject.description}
                        images={this.state.selectedProject.images}
                        changeHandler={p => {}}
                    />
                )
        }
        return this.state.currentView;
    }


}
window.addEventListener('DOMContentLoaded', () => {
    ReactDOM.render(
        <AppContainer />,
        document.querySelector("main")

    );
});