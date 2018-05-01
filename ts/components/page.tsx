import * as React from 'react';
import {Project} from '../appState';
interface IPageProps {
    title: string;
    subtitle: string;
    content: string;
    images: string[];
    teammates: string[];

    changeHandler: (p: Project) => void;
}

export default class PageEditor extends React.Component<IPageProps, {}> {
    pageChanged(ev: any) {
        //construct page
        //call this.props.pageHandler
    }
    render() {
        return (
            <div className="page-view-container">
                <InputGroup
                    label="Title"
                    value={this.props.title}
                    onChange={ev => this.pageChanged(ev)}
                />
                <InputGroup
                    label="Subtitle"
                    value={this.props.subtitle}
                    onChange={ev => this.pageChanged(ev)}
                />
                <div className="content-editor">
                    <textarea
                    >{this.props.content}</textarea>
                    <button
                        onClick={ev => this.pageChanged(ev)}
                    >Cancel</button>
                    <button
                        onClick={ev => this.pageChanged(ev)}
                    >Save</button>
                </div>
            </div>
        )
    }
}

interface IInputGroupProps {
    label: string;
    value: string;
    onChange: (ev: any) => void;
}

class InputGroup extends React.Component<IInputGroupProps, {}> {
    render() {
        return (
            <div className="input-group">
                <label>{this.props.label}</label>
                <input value={this.props.value} onChange={ev => this.props.onChange(ev)} />
            </div>
        )
    }
}

interface IContentEditor {

}

class ContentEditor extends React.Component {

}