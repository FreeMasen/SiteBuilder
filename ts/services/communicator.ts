import AppState, {Page} from '../appState';
export default class Comm {
    constructor(private stateChangeCb: (state: AppState) => void) {
        window.addEventListener('state-change', (ev: CustomEvent) => this.stateChange(ev));
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
        (window.external as any).invoke(JSON.stringify(message))
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

    public static UpdatePage(page: Page):Message {
        return new Message(
            Event.UpdatePage,
            JSON.stringify(page),
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

