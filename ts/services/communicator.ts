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
    public addProjectImage() {
        let msg = {
            kind: 'addProjectImage',
        }
        this.sendMessage(msg);
    }

    public updateAbout(content: string) {
        let msg = Message.UpdateAbout(content);
        this.sendMessage(msg);
    }

    moveImage(oldPos: number, newPos: number) {
        this.sendMessage({
            kind: 'changeImagePos',
            old_pos: oldPos,
            new_pos: newPos,
        })
    }

    public updateSource() {
        this.sendMessage(Message.UpdateSource());
    }

    public updateDest() {
        this.sendMessage(Message.UpdateDest());
    }

    public updateAboutImage() {
        this.sendMessage(Message.UpdateAboutImage());
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

    public selectFont(bold: boolean) {
        this.sendMessage(Message.AddFont(bold));
    }

    public deleteProject() {
        this.sendMessage({
            kind: 'deleteProject',
        })
    }
    public newSite() {
        this.sendMessage({
            kind: "addSite"
        })
    }
    public selectSite(idx: number) {
        this.sendMessage({
            kind: 'chooseSite',
            idx,
        })
    }

    public updateTitle(title: String) {
        this.sendMessage({
            kind: 'changeSiteTitle',
            title,
        });
    }

    private clearMessage() {
        this.sendMessage({
            kind: 'clearMessage',
        })
    }

    private stateChange(ev: CustomEvent) {
        console.log('Comm.stateChange ', ev.detail);
        try {
            let parsedState = AppState.fromJson(ev.detail);
            if (!this.stateChangeCb) return this.sendMessage(Message.Error('Cannot change state w/o state change callback'));
            if (parsedState.message) setTimeout(() => this.clearMessage(), 3000);
            this.stateChangeCb(parsedState);
        } catch(e) {
            console.error(e)
            return this.sendMessage(Message.Error(`Error parsing json ${e}`));
        }
    }

    private sendMessage(message: any) {
        (window as any).send(JSON.stringify(message))
    }
}

class Message {

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
            kind: Event.AddProject,
            name
        }
    }

    public static UpdateProject(project: Project) {
        return {
            kind: Event.UpdateProject,
            project: project.asJson(),
        }
    }

    public static UpdateAbout(content: string) {
        return {
            kind: Event.UpdateAbout,
            content,
        }
    }

    public static Log(msg: String) {
        return {
            kind: Event.Log,
            msg,
        }
    }

    public static UpdateAboutImage() {
        return {
            kind: Event.UpdateAboutImage,
        }
    }

    public static UpdateSource() {
        return {
            kind: Event.UpdateSource,
        }
    }

    public static UpdateDest() {
        return {
            kind: Event.UpdateDest,
        }
    }

    public static AddFont(bold: boolean) {
        return {
            kind: Event.AddFont,
            bold,
        }
    }

    public static RemoveFont(bold: boolean) {
        return {
            kind: Event.RemoveFont,
            bold,
        }
    }


}

enum Event {
    Init = "init",
    Refresh = "refresh",
    Error = "error",
    Build = "build",
    AddProject = "addProject",
    UpdateProject = "updateProject",
    UpdateAbout = "updateAbout",
    UpdateAboutImage = "updateAboutImage",
    Log = 'log',
    UpdateSource = 'updateSource',
    UpdateDest = 'updateDest',
    AddProjectImage = 'addProjectImage',
    RemoveProjectImage = 'removeProjectImage',
    ChangeView = 'changeView',
    AddFont = 'addFont',
    RemoveFont = 'removeFont',
}

