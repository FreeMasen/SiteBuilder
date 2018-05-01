import * as React from 'react';

interface ITitleBarProps {
    title: string;
    backVisible: boolean;
    backHandler: () => void;
}

export default class TitleBar extends React.Component<ITitleBarProps, {}> {
    render() {
        return (
            <header 
                className="title-bar">
                    <div className="button-container">
                    <button 
                        className="back-button"
                        onClick={ev => this.props.backHandler()}
                        style={{
                            display: this.props.backVisible ? null : 'none',
                        }}
                    >↩</button>
                </div>
                <h1 className="title-bar-title">{this.props.title}</h1>
                <div className="button-container">
                </div>
            </header>
        )
    }
}