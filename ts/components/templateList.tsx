import * as React from 'react';
import InputGroup from './inputGroup';

interface ITemplatesListProps {
    names: string[]
    removeHandler: (name: string) => void;
    updateHandler: (name: string) => void;
    newHandler: (name: string) => void;
    exportHandler: (name: string) => void;
}

export default class TemplateList extends React.Component<ITemplatesListProps> {
    render() {
        return (
            <div className="templates-list-container">
                <h2>Your Saved Templates</h2>
                <div className="site-list">
                    {
                        this.props.names.map((name, i) => {
                            return (
                                <Option
                                    key={`template-${i}`}
                                    title={name}
                                    onRemoveClick={() => this.props.removeHandler(name)}
                                    onUpdateClick={() => this.props.updateHandler(name)}
                                    onExportClick={() => this.props.exportHandler(name)}
                                />
                            )
                        })
                    }
                    <NewTemplateForm
                        newHandler={name => this.props.newHandler(name)}
                    />
                </div>
            </div>
        );
    }
}

interface IOptionProps extends React.HTMLProps<HTMLDivElement> {
    title: string;
    onRemoveClick?: (name: string) => void;
    onUpdateClick?: (name: string) => void;
    onExportClick?: (name: string) => void;
}

class Option extends React.Component<IOptionProps, {}> {
    render() {
        return (
            <div className="template">
                <span className="template-name">{this.props.title}</span>
                {
                    <div className="button-group">
                        <button className="export button"
                            onClick={() => this.props.onExportClick(this.props.name)}
                        >Export</button>
                        <button className="update button"
                            onClick={() => this.props.onUpdateClick(this.props.name)}
                        >Update</button>
                        <button className="remove button"
                            onClick={() => this.props.onRemoveClick(this.props.name)}
                        >X</button>
                    </div>
                }
            </div>
        );
    }
}

interface INewTemplateFormProps {
    newHandler: (name: string) => void;
}

interface INewTemplateFormState {
    name: string;
}

class NewTemplateForm extends React.Component<INewTemplateFormProps, INewTemplateFormState> {
    constructor(props) {
        super(props);
        this.state = {
            name: ''
        };
    }
    render() {
        return (
            <div className="new-template-form">
                <InputGroup
                    id="name"
                    label="New Template"
                    defaultValue={this.state.name}
                    onChange={ev => this.setState({name: ev.currentTarget.value,})}
                />
                <button
                    onClick={() => this.props.newHandler(this.state.name)}
                >Create</button>
            </div>
        );
    }
}