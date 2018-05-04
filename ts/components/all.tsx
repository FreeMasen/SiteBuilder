import * as React from 'react';
import AppState, {Project, Fonts} from '../appState';
import InputGroup from './inputGroup';
interface IAllProps {
    source: string;
    destination: string;
    pages: Project[];
    fonts: Fonts;
    updateRequested: () => void;
    sourceSelected: () => void;
    destSelected: () => void;
    addPage: () => void;
    pageSelected: (project: Project) => void;
    aboutSelected: () => void;
    generateSite: () => void;
}

export default class All extends React.Component<IAllProps, {}> {

    render() {
        return (
            <div className="all-container">
                <div className="button-group">
                <button 
                    type="button" 
                    onClick={ev => this.props.updateRequested()}
                >Update
                </button>
                <button 
                    type="button" 
                    onClick={ev => this.props.generateSite()}
                >Generate
                </button>
                <button
                    type="button"
                    onClick={ev => this.props.addPage()}
                >Add Project</button>
            </div>
                <div className="paths">
                    <InputGroup id="infile-input"
                        label="Input Directory"
                        value={this.props.source} 
                        onFocus={ev => this.props.sourceSelected()}
                    />
                    
                    <InputGroup 
                        id="outfile-input"
                        label="Output Directory"
                        value={this.props.destination}
                        onFocus={ev => this.props.destSelected()}
                    />
                </div>
                <h2>Pages</h2>
                <div className="page-list">
                    <div onClick={ev => this.props.aboutSelected()} className="page-link">
                        <span className="page-name">About</span>
                    </div>
                    {this.props.pages.map((p, i) => {
                        return (
                            <div
                                key={`page-${i}`}
                                className="page-link"
                                onClick={ev => this.props.pageSelected(p)}
                            >
                                <span className="page-name">{p.meta.title}</span>
                            </div>
                        )
                    })}
                </div>
                <h2>Fonts</h2>
                <div className="font-list">
                    <div><span className="font">Normal: {this.filename(this.props.fonts.normal)}</span></div>
                    <div><span className="font">Bold: {this.filename(this.props.fonts.bold)}</span></div>
                </div>
            </div>
        )
    }

    filename(path: string): string {
        if (!path || path == '') return 'Unset';
        let parts;
        if (path.indexOf('/') > -1) {
            parts = path.split('/');
        } else if (path.indexOf('\\') > -1) {
            parts = path.split('\\');
        }
        return parts[parts.length - 1];
    }
}