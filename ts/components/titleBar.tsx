import * as React from 'react';

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
                    >↩</button>
                </div>
                <h1 className="title-bar-title">{this.props.title}</h1>
                <div className="last-build-time">
                        {
                            this.props.lastBuilt != null ?
                            <span>{`Last Built: ${this.parseDate(this.props.lastBuilt)}`}</span> :
                            null
                        }
                </div>
            </header>
        )
    }

    parseDate(dt: Date): string {
        var m = dt.getMonth() + 1;
        var d = dt.getDate();
        var y = dt.getFullYear().toString().substr(-2);
        var h = ('0' + dt.getHours()).substr(-2);
        var min = ('0' + dt.getMinutes()).substr(-2);
        var s = ('0' + dt.getSeconds()).substr(-2);
        return m + '/' + d + '/' + y + ' ' + h + ':' + min + ':' + s;
    }
}