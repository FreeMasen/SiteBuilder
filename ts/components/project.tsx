import * as React from 'react';
import { Project, Meta, Image } from '../appState';
import InputGroup from './inputGroup';
import ListBox from './listBox';
import StringHandler from '../services/stringHandler';

interface IProjectEditorProps {
    project: Project;
    addImageHandler: () => void;
    saveHandler: (p: Project) => void;
    cancelHandler: () => void;
    moveImage: (oldPos: number, newPos: number) => void;
    deleteProject: () => void;
}

interface IProjectEditorState {
    title: string;
    subtitle: string;
    teammates: string[];
    description: string;
    images: Image[];
    selectedImage?: Image;
}

export default class ProjectEditor extends React.Component<IProjectEditorProps, IProjectEditorState> {
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
            this.props.project.path,
            new Meta(this.state.title, this.state.subtitle, this.state.teammates),
            this.state.images,
            this.state.description
        );
        this.props.saveHandler(p);
    }

    removeImages() {
        this.setState((prev, props) => {
            return {
                images: prev.images.filter((i) => i != this.state.selectedImage)
            }
        })
    }

    moveImage(up: boolean) {
        console.log('moveImage', up ? 'up' : 'down');
        let oldPos = this.state.selectedImage ? this.state.selectedImage.position : -1;
        if (oldPos < 0) return console.log('old position for this image is below 0');
        let newPos;
        if (up) {
            newPos = oldPos - 1;
        } else {
            newPos = oldPos + 1;
        }
        console.log('moveImage from ', oldPos, 'to', newPos);
        if (newPos < 0 || newPos >= this.state.images.length)
            return console.error('Unable to move image outside of array')
        let images = this.state.images;
        let mover = images[oldPos];
        images[oldPos] = images[newPos];
        images[newPos] = mover;
        console.log('new images', images);
        images.forEach((e, i) => e.position = i);
        this.setState({images});
    }

    render() {
        // alert(`ProjectEditor.render: ${this.state}`);
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
                        <div className="image-editor-title">
                            <span>Images</span>
                            <button
                                className="remove"
                                onClick={ev => this.props.deleteProject()}
                            >Delete</button>
                        </div>
                        <ListBox
                            options={this.props.project.images.map(i => StringHandler.fileName(i.path))}
                            selected={this.state.selectedImage ? this.state.selectedImage.position : null}
                            onChange={i => this.setState({selectedImage: this.state.images[i]})}
                        />
                        <div className="button-group">
                            <button
                                className="remove"
                                onClick={ev => this.removeImages()}
                            >Remove</button>
                            <button
                                className="move"
                                onClick={ev => this.moveImage(true)}
                            >↥</button>
                            <button
                                className="move"
                                onClick={ev => this.moveImage(false)}
                            >↧</button>
                            <button
                                className="add-new"
                                onClick={ev => this.props.addImageHandler()}
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
