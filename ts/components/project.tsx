import * as React from 'react';
import { Project, Meta } from '../appState';
import InputGroup from './inputGroup';

interface IProjectEditorProps {
    project: Project;
    addImageHandler: () => void;
    saveHandler: (p: Project) => void;
    cancelHandler: () => void;
}

interface IProjectEditorState {
    title: string;
    subtitle: string;
    teammates: string[];
    description: string;
    images: string[];
}

export default class ProjectEditor extends React.Component<IProjectEditorProps, IProjectEditorState> {
    imageSelect: HTMLSelectElement;
    constructor(props) {
        super(props);
        console.log('new ProjectEditor', props);
        let { meta, images, description } = props.project;
        this.state = {
            title: meta.title || '',
            subtitle: meta.subtitle || '',
            teammates: meta.teammates || [],
            images,
            description,
        }
    }
    pageSaved() {
        let p = new Project(
            this.props.project.id,
            new Meta(this.state.title, this.state.subtitle, this.state.teammates),
            this.state.images,
            this.state.description
        );
        this.props.saveHandler(p);
    }

    removeImages() {
        let selectedOptions = this.imageSelect.selectedOptions;
        let names = [];
        for (var i = 0; i < selectedOptions.length; i++) {
            names.push(selectedOptions[i].value)
        }
        this.setState((prev, props) => {
            return {
                images: prev.images.filter(n => names.indexOf(n) < 0)
            }
        });
    }

    render() {
        return (
            <div className="project-view-container">
                <div className="editors">
                    <div className="text-editors">
                        <div className="inputs">
                            <InputGroup
                                id="title"
                                label="Title"
                                value={this.state.title}
                                onChange={ev => this.setState({ title: ev.currentTarget.value })}
                            />
                            <InputGroup
                                id="sub-title"
                                label="Subtitle"
                                value={this.state.subtitle}
                                onChange={ev => this.setState({ subtitle: ev.currentTarget.value })}
                            />
                        </div>
                        <div className="content-editor">
                            <textarea
                                id="description"
                                defaultValue={this.state.description}
                                onChange={ev => this.setState({ description: ev.currentTarget.value })}></textarea>
                        </div>
                    </div>
                    <div className="image-editor">
                        <span>Images</span>
                        <select 
                            multiple={true} 
                            className="image-list"
                            ref={s => this.imageSelect = s}
                        >
                        
                            {
                                this.state.images.map((p, i) => {
                                    return (
                                        <option value={p} className="project-image" key={`image-${i}`}>{p}</option>
                                    )
                                })
                            }
                        </select>
                        <div className="button-group">
                            <button 
                                className="remove"
                                onClick={ev => this.removeImages()}
                            >Remove</button>
                            <button 
                                className="add-new"
                                onClick={ev => {}}
                            >Add</button>
                        </div>
                    </div>
                </div>
                <div className="button-group">
                    <button className="cancel"
                        onClick={ev => { this.props.cancelHandler() }}
                    >Cancel</button>
                    <button className="save"
                        onClick={ev => { this.pageSaved() }}
                    >Save</button>
                </div>
            </div>
        )
    }
}
