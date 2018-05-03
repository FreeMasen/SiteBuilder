
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
        public image: string = ''
    ) {

    }

    static fromJson(json: any): Website {
        return new Website(
            json.portfolio.map(Project.fromJson),
            json.about,
            json.image,
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
        public images: string[] = [],
        public description: string,
    ) { }

    public static fromJson(json: any): Project {
        return new Project(
            json.id,
            Meta.fromJson(json.meta),
            json.images,
            json.description
        )
    }

    asJson(): any {
        return {
            id: this.id,
            meta: this.meta.asJson(),
            images: this.images,
            description: this.description,
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