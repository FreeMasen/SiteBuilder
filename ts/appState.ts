
export default class Website {
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
}