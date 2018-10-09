
export default class AppState {
    public site?: Site;
    public siteOptions: SiteOption[];
    public message: ServerMessage[];
    constructor(
        public currentView: Route = Route.Select,
        site: Site = null,
        siteOptions: SiteOption[] = [],
        message: ServerMessage[] = [],
        public templates: string[] = [],
    ) {
        this.site = site;
        this.siteOptions = siteOptions;
        this.message = message;
    }

    public static fromJson(json: any): AppState {
        return new AppState(
            json.currentView,
            Site.fromJson(json.site),
            json.siteOptions.map(SiteOption.fromJson),
            json.message.map(ServerMessage.fromJson),
            Object.getOwnPropertyNames(json.availableTemplates),
        )
    }

    public asJson(): any {
        let site = this.site
                ? this.site.asJson()
                : null;
        let message = this.message
                ? this.message
                : null;
        return {
            site,
            options: this.siteOptions.map(o => o.asJson),
            message: this.message.map(m => m.asJson()),
        };
    }
}

export class Site {
    public selectedProject?: Project;
    public lastBuilt?: Date;
    constructor(
        public source: string = '',
        public destination: string = '',
        public website: Website = new Website(),
        public template: string = 'Default',
        selectedProject: Project = null,
        lastBuilt: Date = null,
    ) {
        this.selectedProject = selectedProject;
        this.lastBuilt = lastBuilt;
    }

    public static fromJson(json: any): Site {
        if (!json) return;
        return new Site(
            json.source,
            json.destination,
            Website.fromJson(json.website),
            json.template,
            Project.fromJson(json.selectedProject),
            json.lastBuilt ? new Date(json.lastBuilt) : null,
        );
    }
    public asJson(): any {
        return {
            source: this.source,
            destination: this.destination,
            website: this.website.asJson(),
            template: this.template,
            selectedProject: this.selectedProject,
            lastBuilt: this.lastBuilt,
        };
    }
}

export class SiteOption {
    constructor(
        public id: number,
        public title: string,
        public path: string,
    ) {}

    public static fromJson(json: any): SiteOption {
        if (!json) return;
        return new SiteOption(
            json.id,
            json.title,
            json.path,
        );
    }

    public asJson(): any {
        return {
            id: this.id,
            title: this.title,
            path: this.path,
        }
    }
}
export enum Route {
    All = 'All',
    Project = 'Project',
    About = 'About',
    Select = 'Select',
}

export class Website {
    constructor(
        public title: string = '',
        public portfolio: Project[] = [],
        public about: string = '',
        public image: string = '',
        public fonts: Fonts = new Fonts(),
        public accentColor: Color = new Color(),
    ) {

    }

    static fromJson(json: any): Website {
        if (!json) return;
        return new Website(
            json.title,
            json.portfolio.map(Project.fromJson),
            json.about,
            json.image,
            Fonts.fromJson(json.fonts),
            Color.fromJson(json.accentColor),
        )
    }

    asJson(): any {
        return {
            title: this.title,
            portfolio: this.portfolio.map(p => p.asJson()),
            about: this.about,
            image: this.image,
            fonts: this.fonts.asJson(),
            accentColor: this.accentColor.asJson(),
        }
    }
}

export class Project {
    constructor(
        public id: number,
        public path: string,
        public meta: Meta = new Meta(),
        public images: Image[] = [],
        public description: string,
        public bwTitleImage: boolean,
    ) { }

    public static fromJson(json: any): Project {
        if (!json) return;
        return new Project(
            json.id,
            json.path,
            Meta.fromJson(json.meta),
            json.images.map(Image.fromJson),
            json.description,
            json.bwTitleImage,
        )
    }

    asJson(): any {
        return {
            id: this.id,
            path: this.path,
            meta: this.meta.asJson(),
            images: this.images.map(i => i.asJson()),
            description: this.description,
            bwTitleImage: this.bwTitleImage,
        }
    }
}

export class Image {
    constructor(
        public position: number = null,
        public path: string = null,
    ) { }

    public static fromJson(json): Image {
        return new Image(
            json.position,
            json.path,
        );
    }

    public asJson(): any {
        return {
            position: this.position,
            path: this.path,
        }
    }
}

export class Fonts {
    constructor(
        public normal: string = '',
        public bold: string = '',
    ) { }

    public static fromJson(json: any): Fonts {
        return new Fonts(
            json.normal,
            json.bold,
        )
    }

    asJson(): any {
        return {
            bold: this.bold,
            normal: this.normal
        }
    }
}

export class Meta {
    constructor(
        public title: string = '',
        public subtitle: string = '',
        public teammates: string[] = [],
    ) {}

    static fromJson(json: any): Meta {
        return new Meta(
            json.title,
            json.context,
            json.teammates,
        )
    }

    asJson(): any {
        return {
            title: this.title,
            context: this.subtitle,
            teammates: this.teammates
        }
    }
}

export class ServerMessage {
    constructor(
        public id: number = 0,
        public content: string = '',
        public isError: boolean = false
    ) {}

    public static fromJson(json: any): ServerMessage {
        if (!json) return;
        return new ServerMessage(
            json.id,
            json.content,
            json.isError,
        );
    }

    public asJson(): any {
        return {
            id: this.id,
            content: this.content,
            isError: this.isError,
        }
    }
}

export class Color {
    constructor(
        public red: number = 0,
        public green: number = 0,
        public blue: number = 0,
        public alpha: number = 1,
    ) {}

    static fromJson(json: any): Color {
        if (!json) return new Color();
        return new Color (
            json.red,
            json.green,
            json.blue,
            json.alpha
        )
    }

    asJson(): any {
        return {
            red: this.red,
            green: this.green,
            blue: this.blue,
            alpha: this.alpha,
        }
    }
}