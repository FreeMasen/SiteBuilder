import AppState, {Project, Meta} from '../appState';

import {mockState} from './mockState';

export default class Comm {
    private mockState: AppState;
    constructor(private stateChangeCb: (state: AppState) => void) {
        this.mockState = mockState();
        window.addEventListener('state-change', (ev: CustomEvent) => this.stateChange(ev));
        setTimeout(() => this.sendMessge(Message.Init()), 2000);
    }

    private stateChange(ev: CustomEvent) {
        try {
            let parsedState = JSON.parse(ev.detail);
            if (!this.stateChangeCb) return this.sendMessge(Message.Error('Cannot change state w/o state change callback'))
            this.stateChangeCb(parsedState);
        } catch(e) {
            return this.sendMessge(Message.Error(`Error parsing json ${e}`));
        }

    }

    public sendMessge(message: Message) {
        // (window.external as any).invoke(JSON.stringify(message))
        switch (message.kind) {
            case Event.Init:
            case Event.Build:
                return this.mockDispatch();
            case Event.UpdateAbout:
                this.mockState.about = message.data;
                return this.mockDispatch();
            case Event.UpdateImage:
                this.mockState.image = message.data;
                return this.mockDispatch();
            case Event.UpdatePage:
                let incoming = JSON.parse(message.data);
                this.mockState.portfolio = this.mockState.portfolio.map(p => {
                    if (p.meta.title == incoming.title) {
                        return incoming;
                    }
                    return p;
                });
                return this.mockDispatch();
            case Event.Add:
                this.mockState.portfolio.push(new Project(
                    new Meta(message.data),
                    [], ''
                ));
                return this.mockDispatch();
        }
    }

    private mockDispatch() {
        window.dispatchEvent(new CustomEvent('state-change', {detail: JSON.stringify(this.mockState)}));
    }
}

class Message {
    constructor(
        public kind: Event,
        public data: string,
    ) {}

    public static Init():Message {
        return new Message(
            Event.Init,
            ""
        )
    }

    public static Error(message: string):Message {
        return new Message (
            Event.Error,
            message,
        )
    }

    public static Build(inDir: string, outDir: string):Message {
        return new Message(
            Event.Build,
            JSON.stringify({in_dir: inDir, out_dir: outDir})
        )
    }

    public static AddPage(name: string):Message {
        return new Message(
            Event.Add,
            name
        )
    }

    public static UpdatePage(project: Project):Message {
        return new Message(
            Event.UpdatePage,
            JSON.stringify(project),
        )
    }

    public static UpdateAbout(text: string):Message {
        return new Message(
            Event.UpdateAbout,
            text
        )
    }

    public static UpdateImage(path: string):Message {
        return new Message(
            Event.UpdateImage,
            path,
        )
    }
}

enum Event {
    Init = "Init",
    Error = "Error",
    Build = "Build",
    Add = "Add",
    UpdatePage = "UpdatePage",
    UpdateAbout = "UpdateAbout",
    UpdateImage = "UpdateImage",
    UpdateSource = "UpdateSource",
    UpdateDest = "UpdateDest"
}

