import * as React from 'react';
import StringHandler from '../services/stringHandler';
import InputGroup from './inputGroup';

interface IColorPickerProps {
    red: number,
    green: number,
    blue: number,
    alpha: number,
}

interface IColorPickerState {
    red: number,
    green: number,
    blue: number,
    alpha: number,
}

export default class ColorPicker extends React.Component<IColorPickerProps, IColorPickerState> {
    constructor(props) {
        super(props);
        this.state = props;
    }

    slideHandler(newValue: number, key: string) {
        this.setState((prev, props) => {
            let ret = {} as any;
            ret[key] = newValue;
            return ret;
        })
    }

    render() {
        return (
            <div className="color-picker-container">
                <span className="color-picker-title">Accent Color</span>
                <div className="color-picker">
                    <div className="sliders">
                        <Slider 
                            value={this.state.red} max={255} 
                            slideHandler={v => this.slideHandler(v, 'red')}
                        />
                        <InputGroup
                            id="red-color-input"
                            label="Red"
                            value={this.state.red.toFixed(2)} 
                        />
                        <Slider value={this.state.green} max={255} slideHandler={p => this.slideHandler(p, 'green')}/>
                        <input value={this.state.green.toFixed(2)} />
                        <Slider value={this.state.blue} max={255} slideHandler={p => this.slideHandler(p, 'blue')}/>
                        <input value={this.state.blue.toFixed(2)} />
                        <Slider value={this.state.alpha} max={1} slideHandler={p => this.slideHandler(p, 'alpha')}/>
                        <input value={this.state.alpha.toFixed(3)} />
                    </div>
                    <div className="current-container">
                        <div className="swatch"
                            style={{
                                background: StringHandler.colorString(this.state.red, this.state.green, this.state.blue, this.state.alpha),
                                width: 50,
                                height: 50,
                            }}
                        ></div>
                    </div>
                    <button
                        onClick={ev => console.log('saved')}
                    >Save</button>
                </div>
            </div>
        )
    }
}

interface ISliderProps {
    value: number;
    max: number;
    slideHandler: (newValue: number) => void;
}

interface ISliderState {
    value: number; 
    max: number;
    captured: boolean;
    mouseY: number
}

class Slider extends React.Component<ISliderProps, ISliderState> {
    constructor(props) {
        super(props);
        this.state = {
            value: props.value,
            max: props.max,
            captured: false,
            mouseY: 0
        }
    }

    render() {
        let barHeight = 15;
        let height = 150;
        let valuePercent = this.props.value / this.props.max;
        let valuePixels = valuePercent * height;
        let top = (height - valuePixels) - (barHeight / 2);
        return (
            <div
                className="slider-container"
                style={{
                    position: 'relative',
                    width: 50,
                    height,
                }}
                onMouseMove={ev => this.move(ev)}
            >
                <div
                    className="slider-bar"
                    style={{
                        position: 'relative',
                        margin: 'auto',
                        width: 1,
                        height: '100%',
                        background: 'lightgrey'
                    }}
                >
                </div>
                <div 
                    className="slider"
                    style={{
                        position: 'absolute',
                        top,
                        height: barHeight,
                        width: '100%',
                        background: 'green',
                    }}
                    onMouseDown={ev => {this.setState({captured: true, mouseY: ev.clientY})}}
                    onMouseUp={ev => {this.setState({captured: false, mouseY: 0})}}
                    onMouseLeave={ev => this.setState({captured: false, mouseY: 0})}
                ></div>
            </div>
        )
    }

    moveStart(ev: React.MouseEvent<HTMLDivElement>) {
        this.setState({captured: true});
        window.onmousemove = ev => this.move(ev);
        window.onmouseup = ev => this.moveEnd(ev);
    }

    moveEnd(ev: MouseEvent) {
        this.setState({captured: false});
        window.onmousemove = null;

    }

    move(ev: MouseEvent) {
        if (!this.state.captured) return;
        let movedY = -ev.movementY;
        let percent = movedY / 150;
        let value = this.props.value + (this.props.value * percent);
        if (value >= this.props.max) value = this.props.max;
        if (value <= 0) value = 0;
        console.log(`prev: ${this.props.value}, percent: ${percent}, new: ${value}`);
        this.props.slideHandler(value);
    }
}