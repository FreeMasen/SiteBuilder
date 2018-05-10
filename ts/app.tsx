import * as React from "react";
import * as ReactDOM from "react-dom";

import AppState, {Website, Project, Meta, Route} from './appState';
import Communicator from './services/communicator';

import TitleBar from './components/titleBar';
import All from './components/all';
import ProjectEditor from './components/project';
import About from './components/about';
import Toast from './components/toast';
import SelectSite from './components/selectSite';

class AppContainer extends React.Component<{}, AppState> {
    private comm: Communicator;
    constructor(props) {
        super(props);
        console.log('AppContainer.constructor');
        this.state = new AppState();
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

    back() {
        if (this.state.currentView == Route.All) {
            this.changeView(Route.Select);
        } else {
            this.changeView(Route.All);
        }
    }

    render() {
        let title = this.state.currentView == Route.All ? 'Site Builder' :
                    this.state.currentView == Route.About ? 'About Editor' :
                    this.state.currentView == Route.Project ? `Project Editor` :
                    this.state.currentView == Route.Select ? 'Select Site' :
                    '';
        return (
            <div id="app-container">
                <TitleBar
                    title={title}
                    backVisible={this.state.currentView != Route.Select}
                    backHandler={() => this.back()}
                    lastBuilt={this.state.site ? this.state.site.lastBuilt : null}
                />
                <div id="app-body">
                    {this.renderBody()}
                </div>
                {
                    this.state.message ? 
                    (
                        <Toast
                            message={this.state.message.content}
                            isError={this.state.message.isError}
                        />
                    )
                    : null
                }
            </div>
        )
    }

    renderBody() {
        console.log('AppContainer.renderBody()');
        switch (this.state.currentView) {
            case Route.All:
                return (
                    <All
                        title={this.state.site.website.title}
                        source={this.state.site ? this.state.site.source : null}
                        destination={this.state.site ? this.state.site.destination : null}
                        pages={this.state.site ? this.state.site.website.portfolio : null}
                        sourceSelected={() => this.comm.updateSource()}
                        destSelected={() => this.comm.updateDest()}
                        addPage={() => this.comm.add(`project-${this.state.site.website.portfolio.length}`)}
                        pageSelected={p => this.changeView(Route.Project, p)}
                        aboutSelected={() => this.changeView(Route.About)}
                        generateSite={() => this.comm.build()}
                        updateRequested={() => this.comm.requestUpdate()}
                        fonts={this.state.site ? this.state.site.website.fonts: null}
                        selectFontClicked={bold => this.comm.selectFont(bold)}
                        updateTitle={title => this.comm.updateTitle(title)}
                    />
                );
            case Route.Project:
                return (
                    <ProjectEditor
                        project={this.state.site.selectedProject}
                        saveHandler={p => this.comm.updateProject(p)}
                        cancelHandler={() => this.changeView(Route.All)}
                        addImageHandler={() => this.comm.addProjectImage()}
                        moveImage={(old, newPos) => this.comm.moveImage(old, newPos)}
                        deleteProject={() => this.comm.deleteProject()}
                    />
                )
            case Route.About:
                return (
                    <About
                        content={this.state.site.website.about}
                        imagePath={this.state.site.website.image}
                        backHandler={() => this.changeView(Route.All)}
                        saveHandler={(content) => this.comm.updateAbout(content)}
                        imageHandler={() => this.comm.updateAboutImage()}
                    />
                )
            case Route.Select:
                return (
                    <SelectSite
                        options={this.state.siteOptions || []}
                        selectionHandler={idx => this.comm.selectSite(idx)}
                        newSiteHandler={() => this.comm.newSite()}
                    />
                );
        }
        return this.state.currentView;
    }


}

ReactDOM.render(<AppContainer />, document.querySelector("#main"));