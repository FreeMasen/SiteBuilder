
export default class AppState {
    constructor(
        public portfolio: Page[],
        public about: string,
        public image: string = ''
    ) {

    }

    static fromJson(json: any): AppState {
        return new AppState(
            json.portfolio.map(Page.fromJson),
            json.about,
            json.image,
        )
    }
}

export class Page {
    constructor(
        public title: string,
        public project: Project,
        public meta: Meta,
    ){}

    static fromJson(json: any): Page {
        return new Page(
            json.title,
            Project.fromJson(json.project),
            Meta.fromJson(json.meta),
        )
    }
}

export class Project {
    constructor(
        public name: string = '',
        public images: string[] = [],
    ) {}

    static fromJson(json: any): Project {
        return new Project(
            json.name,
            json.images,
        )
    }
}

export class Meta {
    constructor(
        public title: string = '',
        public context: string = '',
        public teammates: string[] = [],
    ) {}

    static fromJson(json: any): Meta {
        return new Meta(
            json.title,
            json.context,
            json.teammates,
        )
    }
}