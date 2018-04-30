import * as React from 'react';
import AppState, {Page} from '../appState';

interface IAllProps {
    source: string;
    destination: string;
    pages: Page[];

    updateRequested: () => void;
    sourceChanged: (newPath: string) => void;
    destChange: (newPath: string) => void;
    pageSelected: (page: Page) => void;
    generateSite: () => void;
}

export default class All extends React.Component<IAllProps, {}> {
    render() {
        return (
            <div className="all-container">
                <div id="infile-input" className="input-group">
                    <label>Input Directory</label>
                    <input 
                        id="infile" 
                        value={this.props.source} 
                        onChange={ev => this.props.sourceChanged(ev.currentTarget.value)}
                    />
                    <button 
                        type="button" 
                        onClick={ev => this.props.updateRequested()}
                    >
                        Update
                    </button>
                </div>
                <div id="outfile-input" className="input-group">
                    <label>Output Directory</label>
                    <input
                        id="outfile"
                        value={this.props.destination}
                        onChange={ev => this.props.destChange(ev.currentTarget.value)}
                    />
                    <button 
                        type="button" 
                        onClick={ev => this.props.generateSite()}
                    >
                        Generate
                    </button>
                </div>
                <div className="page-list">
                    {this.props.pages.map(p => {
                        return (
                            <div
                                className="page-link"
                                onClick={ev => this.props.pageSelected(p)}
                            >
                                <span className="page-name">{p.title}</span>
                            </div>
                        )
                    })}
                </div>
            </div>
        )
    }
}