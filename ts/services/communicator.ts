import AppState, {Project, Meta} from '../appState';

import {mockState} from './mockState';

export default class Comm {
    private mockState: AppState;
    private initted = false;
    constructor(source: string, private stateChangeCb: (state: AppState) => void) {
        this.mockState = mockState();
        window.addEventListener('state-change', (ev: CustomEvent) => this.stateChange(ev));
    }

    public requestUpdate(source: string) {
        if (this.initted) throw new Error('Loop error')
        let msg = Message.Init(source);
        this.initted = true;
        this.sendMessge(msg);
    }

    public build(inDir: string, outDir: string) {
        let msg = Message.Build(inDir, outDir);
        this.sendMessge(msg);
    }

    public updateProject(p: Project) {
        let msg = Message.UpdateProject(p);
        this.sendMessge(msg);
    }

    public updateAbout(path: string, content: string) {
        let msg = Message.UpdateAbout(path, content);
        this.sendMessge(msg);
    }

    public log(msg: string) {
        this.sendMessge(Message.Log(msg));
    }

    private stateChange(ev: CustomEvent) {
        this.log('Comm.stateChange ' + JSON.stringify(ev.detail));
        try {
            let parsedState = JSON.parse(ev.detail);
            if (!this.stateChangeCb) return this.sendMessge(Message.Error('Cannot change state w/o state change callback'))
            this.stateChangeCb(parsedState);
        } catch(e) {
            return this.sendMessge(Message.Error(`Error parsing json ${e}`));
        }
    }

    private sendMessge(message: any) {
        if (!(window.external as any).invoke) {
            return this.mockEventHandler(message);
        }
        (window.external as any).invoke(JSON.stringify(message))
    }
    
    private mockEventHandler(message: Message) {
        switch (message.kind) {
            case Event.Init:
            case Event.Build:
                return this.mockDispatch();
            case Event.UpdateAbout:
                let parsed = JSON.parse(message.data);
                this.mockState.image = parsed.image;
                this.mockState.about = parsed.content;
                return this.mockDispatch();
            case Event.UpdateProject:
                let incoming = JSON.parse(message.data);
                this.mockState.portfolio = this.mockState.portfolio.map(p => {
                    if (p.id == incoming.id) {
                        return incoming;
                    }
                    return p;
                });
                return this.mockDispatch();
            case Event.Add:
                this.mockState.portfolio.push(new Project(
                    this.mockState.portfolio.length,
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

    public static Init(source: string) {
        return {
            kind: Event.Init,
            source: source
        }
    }

    public static Error(message: string)  {
        return {
            kind: Event.Error,
            message,
        }
    }

    public static Build(inDir: string, outDir: string) {
        return {
            kind: Event.Build,
            source: inDir, 
            out_dir: outDir
        }
    }

    public static AddPage(name: string) {
        return {
            kind: Event.Add,
            name
        }
    }

    public static UpdateProject(project: Project) {
        return {
            kind: Event.UpdateProject,
            project,
        }
    }

    public static UpdateAbout(path: string, text: string) {
        return {
            kind: Event.UpdateAbout,
            imagePath: path, 
            content: text
        }
    }

    public static Log(msg: String) {
        return {
            kind: Event.Log,
            msg,
        }
    }
}

enum Event {
    Init = "init",
    Error = "error",
    Build = "build",
    Add = "add",
    UpdateProject = "updateProject",
    UpdateAbout = "updateAbout",
    Log = 'log',
}

