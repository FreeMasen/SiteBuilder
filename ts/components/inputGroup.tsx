import * as React from 'react';
interface IInputGroupProps {
    id: string;
    label: string;
    value?: string;
    defaultValue?: string;
    onChange?: (ev: React.ChangeEvent<HTMLInputElement>) => void;
    onFocus?: (ev: React.FocusEvent<HTMLInputElement>) => void;
    onBlur?: (ev: React.FocusEvent<HTMLInputElement>) => void;
}

export default class InputGroup extends React.Component<IInputGroupProps, {}> {
    render() {
        return (
            <div className="input-group">
                <label>{this.props.label}</label>
                <input 
                    id={this.props.id} 
                    value={this.props.value} 
                    defaultValue={this.props.defaultValue}
                    onChange={this.props.onChange ? ev => this.props.onChange(ev) : null} 
                    onFocus={this.props.onFocus ? ev => this.props.onFocus(ev) : null}
                    onBlur={this.props.onBlur ? ev => this.props.onBlur(ev) : null}
                />
            </div>
        )
    }
}