
export default class AppState {
    public selectedProject?: Project;
    constructor(
        public source: string = '',
        public destination: string = '',
        public website: Website = new Website(),
        public currentView: Route = Route.All,
        selectedProject: Project = null,
    ) {
        this.selectedProject = selectedProject;
    }

    public static fromJson(json: any): AppState {
        return new AppState(
            json.source,
            json.destination,
            Website.fromJson(json.website),
            json.currentView,
            json.selectedProject,
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
        let portfolio = [];
        for (var i = 0; i < json.portfolio;i++) {
            Project.fromJson(json.portfolio[i]);
        }
        return new Website(
            portfolio,
            json.about,
            json.image,
        )
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

    toJson(): any {
        return {
            title: this.title,
            context: this.subtitle,
            teammates: this.teammates
        }
    }
}