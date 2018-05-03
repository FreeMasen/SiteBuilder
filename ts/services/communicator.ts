import AppState, {Website, Project, Meta, Route} from '../appState';


export default class Comm {
    constructor(private stateChangeCb: (state: AppState) => void) {
        window.addEventListener('state-change', (ev: CustomEvent) => this.stateChange(ev));
    }

    public init() {
        let msg = Message.Init();
        this.sendMessage(msg);
    }

    public requestUpdate() {
        let msg = Message.Refresh();
        this.sendMessage(msg);
    }

    public build() {
        let msg = Message.Build();
        this.sendMessage(msg);
    }

    public add(name: string) {
        let msg = Message.AddPage(name);
        this.sendMessage(msg);
    }

    public updateProject(p: Project) {
        let msg = Message.UpdateProject(p);
        this.sendMessage(msg);
    }

    public updateAbout(path: string, content: string) {
        let msg = Message.UpdateAbout(path, content);
        this.sendMessage(msg);
    }

    public getDirectory(name: string) {
        let msg = Message.OpenDialog(name);
        this.sendMessage(msg);
    }

    public log(msg: string) {
        this.sendMessage(Message.Log(msg));
    }

    public changeView(route: Route, project?: Project) {
        let msg = {
            kind: "changeView", 
            route,
            project: project ? project.asJson() : null
        }
        this.sendMessage(msg);
    }

    private stateChange(ev: CustomEvent) {
        console.log('Comm.stateChange ', ev.detail);
        try {
            let parsedState = AppState.fromJson(ev.detail);
            if (!this.stateChangeCb) return this.sendMessage(Message.Error('Cannot change state w/o state change callback'))
            this.stateChangeCb(parsedState);
        } catch(e) {
            console.error(e)
            return this.sendMessage(Message.Error(`Error parsing json ${e}`));
        }
    }

    private sendMessage(message: any) {
        console.log('invoke: ', message);
        (window.external as any).invoke(JSON.stringify(message))
    }
}

class Message {
    constructor(
        public kind: Event,
        public data: string,
    ) {}

    public static Init() {
        return {
            kind: Event.Init
        }
    }

    public static Refresh() {
        return {
            kind: Event.Refresh
        }
    }

    public static Error(message: string)  {
        return {
            kind: Event.Error,
            message,
        }
    }

    public static Build() {
        return {
            kind: Event.Build
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
            project: project.asJson(),
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

    public static OpenDialog(name: string) {
        return {
            kind: Event.OpenDialog,
            name,
        }
    }
}

enum Event {
    Init = "init",
    Refresh = "refresh",
    Error = "error",
    Build = "build",
    Add = "add",
    UpdateProject = "updateProject",
    UpdateAbout = "updateAbout",
    Log = 'log',
    OpenDialog = 'openDialog'
}

