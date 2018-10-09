import * as React from 'react';
import { Color, Fonts, Project } from '../appState';
import StringHandler from '../services/stringHandler';
import ColorPicker from './colorPicker';
import InputGroup from './inputGroup';


interface IAllProps {
    source: string;
    destination: string;
    pages: Project[];
    fonts: Fonts;
    title: string;
    color: Color;
    templateOptions: string[];
    selectedTemplate: string;
    updateRequested: () => void;
    sourceSelected: () => void;
    destSelected: () => void;
    addPage: () => void;
    pageSelected: (project: Project) => void;
    aboutSelected: () => void;
    generateSite: () => void;
    selectFontClicked: (bold: boolean) => void;
    updateTitle: (title: string) => void;
    colorSaved: (color: Color) => void;
    selectedTemplateChange: (name: string) => void;
}

interface IAllState {
    title: string;
}

export default class All extends React.Component<IAllProps, IAllState> {
    titleChangedTimeout?: number = null;
    constructor(props: IAllProps) {
        super(props);
        console.log('new All', props);
        this.state = {
            title: props.title
        }
    }

    componentWillReceiveProps(props) {
        if (this.state.title != props.title) {
            this.setState({title: props.title});
        }
    }

    render() {
        return (
            <div className="all-container">
                <div className="site-title">
                <InputGroup
                    id="site-title"
                    label="Site Title"
                    defaultValue={this.props.title}
                    onBlur={ev => this.props.updateTitle(ev.currentTarget.value)}
                />
                </div>
                <div className="button-group">
                    <button
                        title="Refresh from the source folder"
                        type="button" 
                        onClick={ev => this.props.updateRequested()}
                    >Refresh
                    </button>
                    <button 
                        title="Generate the html for this site"
                        type="button" 
                        onClick={ev => this.props.generateSite()}
                    >Generate
                    </button>
                    <button
                        title="Add a new empty project to this site"
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
                    <div 
                        className="font"
                        onClick={ev => this.props.selectFontClicked(false)}
                    >
                        <span className="font-name">Normal: {StringHandler.fileName(this.props.fonts.normal)}</span>
                    </div>
                    <div
                        className="font"
                        onClick={e => this.props.selectFontClicked(true)}
                    >
                        <span className="font-name">Bold: {StringHandler.fileName(this.props.fonts.bold)}</span>
                    </div>
                </div>
                <h2>Template</h2>
                <div className="template-option">
                    <div
                        className="selected-template">
                        <span className="template-name">Template Placeholder</span>
                    </div>
                    <select className="template-select" value={this.props.selectedTemplate} onChange={ev => this.props.selectedTemplateChange(ev.currentTarget.value)}>
                        {
                            this.props.templateOptions.map((name, i) => {
                                return (
                                    <option key={`template-option-${i}`} value={name}>{name}</option>
                                )
                            })
                        }
                    </select>
                </div>
                <ColorPicker
                    red={this.props.color.red}
                    green={this.props.color.green}
                    blue={this.props.color.blue}
                    alpha={this.props.color.alpha}
                    colorSaved={color => this.props.colorSaved(color)}
                />
            </div>
        )
    }
}