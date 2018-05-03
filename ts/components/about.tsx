import * as React from 'react';
import InputGroup from './inputGroup';

interface IAboutEditorProps {
    imagePath: string;
    content: string;
    backHandler: () => void;
    saveHandler: (image: string, content: string) => void;
    imageHandler: () => void;
}

interface IAboutEditorState {
    imagePath: string;
    content: string;
}

export default class AboutEditor extends React.Component<IAboutEditorProps, IAboutEditorState> {
    constructor(props) {
        super(props);
        this.state = {
            imagePath: props.imagePath,
            content: props.content,
        }
    }
    save() {
        
    }
    render() {
        return (
            <div className="about-container">
                <div className="image-container">
                    <InputGroup
                        id="about-image"
                        label="Image"
                        onFocus={ev => this.props.imageHandler()}
                        value={this.props.imagePath}
                    />
                    <button 
                        onClick={ev => {}}
                    >Browse</button>
                </div>
                <div className="input-group content-editor">
                    <label>About Content</label>
                    <textarea 
                        defaultValue={this.props.content}
                        onChange={ev => this.setState({content: ev.currentTarget.value})}
                    ></textarea>
                </div>
                <div className="button-group">
                    <button
                        className="cancel"
                        onClick={ev => {}}
                    >Cancel</button>
                    <button
                        className="save"
                        onClick={ev => {}}
                    >Save</button>
                </div>
            </div>
        )
    }
}