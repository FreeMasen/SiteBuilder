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
        this.state = {
            website,
            currentView: Route.All,
            source: '',
            destination: '',
            selectedProject: null,
        };
        this.comm = new Communicator(s => this.communicatorCallback(s));
    }

    componentDidMount() {
        console.log('AppContainer.componentDidMount')
        this.comm.init();
    }

    communicatorCallback(s: AppState) {
        console.log('communicatorCallback', s)
        this.setState((prev, props) => {
            return s
        })
    }

    changeView(route: Route, project: Project = null) {
        this.comm.changeView(route, project);
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
                    backHandler={() => this.changeView(Route.All)}
                    lastBuilt={this.state.lastBuilt}
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
                console.log('Route.All', this.state.website.portfolio.map(p => p.meta.title));
                return (
                    <All
                        source={this.state.source}
                        destination={this.state.destination}
                        pages={this.state.website.portfolio}
                        sourceSelected={() => this.comm.updateSource()}
                        destSelected={() => this.comm.updateDest()}
                        addPage={() => this.comm.add(`project-${this.state.website.portfolio.length}`)}
                        pageSelected={p => this.changeView(Route.Project, p)}
                        aboutSelected={() => this.changeView(Route.About)}
                        generateSite={() => this.comm.build()}
                        updateRequested={() => this.comm.requestUpdate()}
                        fonts={this.state.website.fonts}
                        selectFontClicked={bold => this.comm.selectFont(bold)}
                    />
                );
            case Route.Project:
                return (
                    <ProjectEditor
                        project={this.state.selectedProject}
                        saveHandler={p => {
                            this.comm.updateProject(p);
                            this.changeView(Route.All);
                        }}
                        cancelHandler={() => this.changeView(Route.All)}
                        addImageHandler={() => {}}
                    />
                )
            case Route.About:
                return (
                    <About
                        content={this.state.website.about}
                        imagePath={this.state.website.image}
                        backHandler={() => this.changeView(Route.All)}
                        saveHandler={(content) => this.comm.updateAbout(content)}
                        imageHandler={() => this.comm.updateAboutImage()}
                    />
                )
        }
        return this.state.currentView;
    }


}

ReactDOM.render(<AppContainer />, document.querySelector("#main"));