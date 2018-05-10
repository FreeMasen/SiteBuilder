import * as React from 'react';
import {SiteOption} from '../appState';
interface ISelectSiteProps {
    options: SiteOption[]
    selectionHandler: (idx: number) => void;
    newSiteHandler: () => void;
}

export default class SelectSite extends React.Component<ISelectSiteProps> {
    render() {
        return (
            <div className="select-site-container">
                <h2>Select a site</h2>
                <div className="site-list">
                    {
                        this.props.options.map((o, i) => {
                            return (
                                <Option
                                    key={`site-option-${i}`}
                                    title={o.title}
                                    path={o.path}
                                    onClick={ev => this.props.selectionHandler(i)}
                                />
                            )
                        })
                    }
                    <Option
                        key="new-site-option"
                        title="Create new site"
                        onClick={ev => {
                            console.log("create new site");
                            this.props.newSiteHandler();
                        }}
                    />
                </div>
            </div>
        );
    }
}

interface IOptionProps extends React.HTMLProps<HTMLDivElement> {
    title: string;
    path?: string;
    onClick?: (ev: React.MouseEvent<HTMLDivElement>) => void;
}

class Option extends React.Component<IOptionProps, {}> {
    render() {
        return (
            <div className="site-option"
                onClick={this.props.onClick}
            >
                <span className="site-title">{this.props.title}</span>
                {
                    this.props.path ?
                    <span className="site-path">{this.props.path}</span> :
                    null
                }
            </div>
        );
    }
}