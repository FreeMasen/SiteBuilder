import * as React from 'react';


interface IListBoxProps {
    options: string[];
    changeHandler: (value: string) => void;
}

interface IListBoxState {
    selected: number;
}

export default class ListBox extends React.Component<IListBoxProps, {}> {
    render() {
        <div class="list-box-container">
            {
                this.props.options.map((o, i) => {
                    return (
                        <div
                            key={`${o.id}-option`}
                            onClick={ev => this.changeHandler(o)}
                            class={this.state.selected == i ? 'list-box-option selected' : 'list-box-option'}
                        >
                            <span class="list-box-option-text">{o}</span>
                        </div>
                    )
                })
            }
        </div>
    }

    changeHandler(idx: number) {
        this.setState({selected: idx});
        this.props.changeHandler(idx);
    }
}