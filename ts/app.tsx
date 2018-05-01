import * as React from "react";
import * as ReactDOM from "react-dom";

import Website, {Project, Meta} from './appState';
import Communicator from './services/communicator';

import TitleBar from './components/titleBar';
import All from './components/all';
import ProjectEditor from './components/project';
import About from './components/about';

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
    About,
}

class AppContainer extends React.Component<{}, State> {
    private comm: Communicator;
    constructor(props) {
        super(props);
        let website = new Website();
        let lastPaths = this.getPathsFromStorage();
        this.state = {
            website,
            currentView: history.state ? history.state.currentView : Route.All,
            source: lastPaths.source,
            destination: lastPaths.destination,
            selectedProject: history.state ? history.state.project : null,
        };
        this.comm = new Communicator(this.state.source, s => this.communicatorCallback(s))
        this.comm.log('AppContainer.constructor');
    }

    componentDidMount() {
        this.comm.log('AppContainer.componentDidMount')
        this.comm.requestUpdate(this.state.source);
    }

    getPathsFromStorage(): {source: string, destination: string} {
        let fallback = {
            source: '',
            destination: '',
        };
        try {
            let stored;
            if (window.localStorage) {
                stored = localStorage.getItem('paths');
            } else {
                stored = document.cookie;
            }
            if (!stored) return fallback;
            return JSON.parse(stored);
        } catch(e) {
            return fallback;
        }
    }

    updateSource(newValue: string) {
        this.setState({source: newValue}, () => this.storePaths());
    }

    updateDestination(newValue: string) {
        this.setState({destination: newValue}, () => this.storePaths());
    }

    storePaths() {
        let value = JSON.stringify({source: this.state.source, destination: this.state.destination});
        localStorage.setItem('paths', value);
    }

    communicatorCallback(w: Website) {
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
    goToAbout() {
        this.setState((prev, props) => {
            return {
                selectedProject: null,
                currentView: Route.About,
            }
        })
    }
    componentWillMount() {
        let spinner = document.getElementById('spinner-container');
        if (!spinner) return;
        spinner.parentElement.removeChild(spinner);
    }

    render() {
        let title = this.state.currentView == Route.All ? 'Site Builder' :
                    this.state.currentView == Route.About ? 'About Editor' :
                    this.state.currentView == Route.Project ? `Project Editor` : 
                    ''
        return (
            <div id="app-container">
                <TitleBar
                    title={title}
                    backVisible={this.state.currentView != Route.All}
                    backHandler={() => this.gotoAll()}
                />
                <main>
                    {this.renderBody()}
                </main>
            </div>
        )
    }

    renderBody() {
        console.log('AppContainer.render', this.state);
        switch (this.state.currentView) {
            case Route.All:
                return (
                    <All
                        source={this.state.source}
                        destination={this.state.destination}
                        pages={this.state.website.portfolio}
                        sourceChanged={p => this.updateSource(p)}
                        destChange={p => this.updateDestination(p)}
                        pageSelected={p => this.selectPage(p)}
                        aboutSelected={() => this.goToAbout()}
                        generateSite={() => this.comm.build(this.state.source, this.state.destination)}
                        updateRequested={() => this.comm.requestUpdate(this.state.source)}
                    />
                );
            case Route.Project:
                return (
                    <ProjectEditor
                        project={this.state.selectedProject}
                        saveHandler={p => {
                            this.comm.updateProject(p);
                            this.gotoAll();
                        }}
                        cancelHandler={() => this.gotoAll()}
                        addImageHandler={() => {}}
                    />
                )
            case Route.About:
                return (
                    <About
                        content={this.state.website.about}
                        imagePath={this.state.website.image}
                        backHandler={() => this.gotoAll()}
                        saveHandler={(path, content) => this.comm.updateAbout(path, content)}
                    />
                )
        }
        return this.state.currentView;
    }


}
window.addEventListener('DOMContentLoaded', () => {
    console.log('DOMContentLoaded')
    ReactDOM.render(
        <AppContainer />,
        document.querySelector("#main"));
})
