import * as React from 'react';
interface IInputGroupProps {
    id: string;
    label: string;
    value: string;
    onChange?: (ev: React.ChangeEvent<HTMLInputElement>) => void;
    onFocus?: (ev: React.FocusEvent<HTMLInputElement>) => void;
}

export default class InputGroup extends React.Component<IInputGroupProps, {}> {
    render() {
        return (
            <div className="input-group">
                <label>{this.props.label}</label>
                <input 
                    id={this.props.id} 
                    value={this.props.value} 
                    onChange={this.props.onChange ? ev => this.props.onChange(ev) : null} 
                    onFocus={this.props.onFocus ? ev => this.props.onFocus(ev) : null}
                />
            </div>
        )
    }
}