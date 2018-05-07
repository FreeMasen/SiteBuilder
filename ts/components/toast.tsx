import * as React from 'react';

interface IToastProps {
    message: string,
    isError: boolean
}

export default class Toast extends React.Component<IToastProps, {}> {
    render() {
        let className = `toast-message${this.props.isError ? ' error' : ''}`;
        return (
            <div className={className}>
                <span>{this.props.message}</span>
            </div>
        )
    }
}