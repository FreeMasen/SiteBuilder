import * as React from "react";
import * as ReactDOM from "react-dom";

import AppState, {Website, Project, Meta, Route} from './appState';
import Communicator from './services/communicator';

import TitleBar from './components/titleBar';
import All from './components/all';
import ProjectEditor from './components/project';
import About from './components/about';

class AppContainer extends React.Component<{}, AppState> {
    private comm: Communicator;
    constructor(props) {
        super(props);
        console.log('AppContainer.constructor');
        let website = new Website();
        let lastPaths = this.getPathsFromStorage();
        this.state = {
            website,
            currentView: Route.All,
            source: lastPaths.source,
            destination: lastPaths.destination,
            selectedProject: null,
        };
        this.comm = new Communicator(s => this.communicatorCallback(s));
    }

    componentDidMount() {
        console.log('AppContainer.componentDidMount')
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

    updateSource() {
        this.comm.getDirectory('source');
    }

    updateDestination() {
        this.comm.getDirectory('destination');
    }

    storePaths() {
        let value = JSON.stringify({source: this.state.source, destination: this.state.destination});
        if (localStorage) {
            localStorage.setItem('paths', value);
        } else {
            document.cookie = value;
        }
    }

    communicatorCallback(s: AppState) {
        this.setState((prev, props) => {
            return s
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
        console.log('App.render()');
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
                <div>
                    {this.renderBody()}
                </div>
            </div>
        )
    }

    renderBody() {
        console.log('AppContainer.renderBody()');
        switch (this.state.currentView) {
            case Route.All:
                console.log('Route.All', this.state.website.portfolio.length, 'projects');
                return (
                    <All
                        source={this.state.source}
                        destination={this.state.destination}
                        pages={this.state.website.portfolio}
                        sourceSelected={() => this.updateSource()}
                        destSelected={() => this.updateDestination()}
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

ReactDOM.render(<AppContainer />, document.querySelector("#main"));
