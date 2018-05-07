import * as React from 'react';


interface IListBoxProps {
    options: string[];
    selected?: number;
    onChange: (i: number) => void;
}

interface IListBoxState {
}

export default class ListBox extends React.Component<IListBoxProps, IListBoxState> {
    constructor(props) {
        super(props);
        this.state = {
        };
    }
    render() {
        return (
            <div className="list-box-container">
                {
                    this.props.options.map((o, i) => {
                        return (
                            <div
                                key={`${i}-option`}
                                onClick={ev => this.props.onChange(i)}
                                className={this.props.selected == i ? 'list-box-option selected' : 'list-box-option'}
                            >
                                <span className="list-box-option-text">{o}</span>
                            </div>
                        )
                    })
                }
            </div>
        );
    }
}