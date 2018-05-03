import AppState, {Website, Project, Meta} from '../appState';


export default class Comm {
    private initted = false;
    constructor(private stateChangeCb: (state: AppState) => void) {
        window.addEventListener('state-change', (ev: CustomEvent) => this.stateChange(ev));
    }

    public requestUpdate(source: string) {
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

    public getDirectory(name: string) {
        let msg = Message.OpenDialog(name);
        this.sendMessge(msg);
    }

    public log(msg: string) {
        this.sendMessge(Message.Log(msg));
    }

    private stateChange(ev: CustomEvent) {
        console.log('Comm.stateChange ', ev.detail);
        try {
            let parsedState = AppState.fromJson(ev.detail);
            if (!this.stateChangeCb) return this.sendMessge(Message.Error('Cannot change state w/o state change callback'))
            this.stateChangeCb(parsedState);
        } catch(e) {
            console.error(e)
            return this.sendMessge(Message.Error(`Error parsing json ${e}`));
        }
    }

    private sendMessge(message: any) {
        (window.external as any).invoke(JSON.stringify(message))
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

    public static OpenDialog(name: string) {
        return {
            kind: Event.OpenDialog,
            name,
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
    OpenDialog = 'openDialog'
}

