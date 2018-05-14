import * as React from 'react';
import { ServerMessage } from '../appState';

interface IToastProps {
    message: ServerMessage,
    clearMessage: (id: number) => void;
}

export default class Toast extends React.Component<IToastProps, {}> {
    timeoutHandle?: number = null;

    componentWillReceiveProps(props) {
        console.log('componentWillReceiveProps', props);
        if (props.message != null && this.timeoutHandle == null) {
            console.warn('setting timeout for message', props.message.id);
            this.timeoutHandle = setTimeout(() => {
                this.timeoutHandle = null;
                props.clearMessage(props.message.id)
            }, 3000)
        }
    }
    render() {
        if (!this.props.message) return null;
        let className = `toast-message${this.props.message.isError ? ' error' : ''}`;
        return (
            <div className={className}>
                <span>{this.props.message.content}</span>
            </div>
        )
    }
}