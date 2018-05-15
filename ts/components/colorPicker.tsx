import * as React from 'react';
import { Color } from '../appState';
import StringHandler from '../services/stringHandler';
import InputGroup from './inputGroup';

interface IColorPickerProps {
    red: number,
    green: number,
    blue: number,
    alpha: number,
    colorSaved: (color: Color) => void;
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
        console.log('new ColorPicker', props);
        this.state = props;
    }

    slideHandler(newValue: number, key: string) {
        if (key != 'alpha')
            newValue = Math.round(newValue);
        this.setState((prev, props) => {
            let ret = {} as any;
            ret[key] = newValue;
            return ret;
        })
    }

    inputChanged(ev: React.ChangeEvent<HTMLInputElement>, key: string) {
        try {
            let val = parseInt(ev.currentTarget.value);
            this.slideHandler(val, key);
        } catch (e) {
            console.error('error parsing input');
        }
    }

    hexChange(value: string) {
        if (value[0] == '#') value = value.substring(1);
        if (value.length < 6) return;
        this.setState(StringHandler.fromHex(value))
    }

    render() {
        return (
            <div className="color-picker-container">
                <span className="color-picker-title">Accent Color</span>
                <div className="color-picker">
                    <div className="sliders">
                        <div className="slider-pair">
                            <InputGroup
                                id="red-color-input"
                                label="Red"
                                value={this.state.red.toFixed(0)}
                                onChange={ev => this.inputChanged(ev, 'red')}
                            />
                            <Slider
                                value={this.state.red} max={255}
                                slideHandler={v => this.slideHandler(v, 'red')}
                            />
                        </div>
                        <div className="slider-pair">
                            <InputGroup
                                id="green-color-input"
                                label="Green"
                                value={this.state.green.toFixed(0)}
                                onChange={ev => this.inputChanged(ev, 'green')}
                            />
                            <Slider 
                                value={this.state.green} 
                                max={255} 
                                slideHandler={p => this.slideHandler(p, 'green')}
                            />
                        </div>
                        <div className="slider-pair">
                            <InputGroup
                                id="blue-color-input"
                                label="Blue"
                                value={this.state.blue.toFixed(0)}
                                onChange={ev => this.inputChanged(ev, 'blue')}
                            />
                            <Slider 
                                value={this.state.blue} 
                                max={255} 
                                slideHandler={v => this.slideHandler(v, 'blue')}
                            />
                        </div>
                        <div className="slider-pair">
                            <InputGroup
                                id="alpha-color-input"
                                label="Alpha"
                                value={this.state.alpha.toFixed(3)} 
                                onChange={ev => this.inputChanged(ev, 'alpha')}
                            />
                            <Slider 
                                value={this.state.alpha} 
                                max={1} 
                                slideHandler={p => this.slideHandler(p, 'alpha')}
                            />
                        </div>
                    </div>
                    <div className="current-container">
                    <InputGroup 
                            id="hex-value"
                            label="Hex code"
                            value={StringHandler.hexColor(this.state.red, this.state.green, this.state.blue)}
                            onChange={v => this.hexChange(v.currentTarget.value)}
                        />
                        <div className="swatch"
                            style={{
                                background: StringHandler.colorString(this.state.red, this.state.green, this.state.blue, this.state.alpha),
                                width: 150,
                                height: 150,
                                marginTop: 5,
                            }}
                        ></div>
                    </div>
                </div>
                <button
                    onClick={ev => this.props.colorSaved(new Color(this.state.red, this.state.green, this.state.blue, this.state.alpha))}
                >Save</button>
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
        let barHeight = 8;
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
            >
                <div
                    className="slider-bar"
                    style={{
                        position: 'relative',
                        margin: 'auto',
                        width: 2,
                        border: '1px solid rgba(0,0,0,0.5)',
                        height: '100%',
                        
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
                        background: '#5d6e87',
                    }}
                    onMouseDown={ev => this.moveStart(ev)}
                ></div>
            </div>
        )
    }

    moveStart(ev: React.MouseEvent<HTMLDivElement>) {
        this.setState({captured: true, mouseY: ev.clientY});
        window.onmousemove = ev => this.move(ev);
        window.onmouseup = ev => this.moveEnd(ev);
    }

    moveEnd(ev: MouseEvent) {
        this.setState({captured: false});
        window.onmousemove = null;
        window.onmouseup = null;
    }

    move(ev: MouseEvent) {
        if (!this.state.captured) return;
        let movedY = this.state.mouseY - ev.clientY;
        let percent = movedY / 150;
        let value = this.props.value + (this.props.max * percent);
        if (value >= this.props.max) value = this.props.max;
        if (value <= 0) value = 0;
        this.props.slideHandler(value);
        this.setState({mouseY: ev.clientY});
    }
}