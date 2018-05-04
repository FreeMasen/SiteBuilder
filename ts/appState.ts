
export default class AppState {
    public selectedProject?: Project;
    public lastBuilt?: Date; 
    constructor(
        public source: string = '',
        public destination: string = '',
        public website: Website = new Website(),
        public currentView: Route = Route.All,
        selectedProject: Project = null,
        lastBuilt: Date = null,
    ) {
        this.selectedProject = selectedProject;
        this.lastBuilt = lastBuilt;
    }

    public static fromJson(json: any): AppState {
        return new AppState(
            json.source,
            json.destination,
            Website.fromJson(json.website),
            json.currentView,
            json.selectedProject,
            new Date(json.lastBuilt),
        )
    }
}
export enum Route {
    All,
    Project,
    About,
}

export class Website {
    constructor(
        public portfolio: Project[] = [],
        public about: string = '',
        public image: string = '',
        public fonts: Fonts = new Fonts(),
    ) {

    }

    static fromJson(json: any): Website {
        return new Website(
            json.portfolio.map(Project.fromJson),
            json.about,
            json.image,
            Fonts.fromJson(json.fonts),
        )
    }

    asJson(): any {
        return {
            portfolio: this.portfolio.map(p => p.asJson()),
            about: this.about,
            image: this.image,
        }
    }
}

export class Project {
    constructor(
        public id: number,
        public meta: Meta = new Meta(),
        public images: Image[] = [],
        public description: string,
    ) { }

    public static fromJson(json: any): Project {
        return new Project(
            json.id,
            Meta.fromJson(json.meta),
            json.images.map(Image.fromJson),
            json.description
        )
    }

    asJson(): any {
        return {
            id: this.id,
            meta: this.meta.asJson(),
            images: this.images.map(i => i.asJson()),
            description: this.description,
        }
    }
}

export class Image {
    constructor(
        public position: number = null,
        public path: string = null
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