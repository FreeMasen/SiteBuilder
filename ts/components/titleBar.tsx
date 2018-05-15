import * as React from 'react';
import StringHandler from '../services/stringHandler';

interface ITitleBarProps {
    title: string;
    backVisible: boolean;
    lastBuilt?: Date;
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
                    ><svg viewBox="0 0 40 40">
                    <path
                        d="M 36 16
                        l -15 0
                        l 0 -8
                        l -15 13
                        l 15 13
                        l 0 -8
                        l 15 0
                        z"
                    />
                    </svg></button>
                </div>
                <h1 className="title-bar-title">{this.props.title}</h1>
                <div className="last-build-time">
                        {
                            this.props.lastBuilt != null ?
                            <span>{`Last Built: ${StringHandler.parseDate(this.props.lastBuilt)}`}</span> :
                            null
                        }
                </div>
            </header>
        )
    }
}