import Website, { Meta, Project } from '../appState';

export let mockState = setupState;

let names = [
    'Building\'s lament',
    'Harrowing Tale',
    'Moose Tracks Ice Cream',
    'Juno Diaz\'s House Party',
    'Kid and Play Leave Las Vagas'
]

let contexts = [
    'Murder She Wrote',
    'Marc Maron v. Louis C.K.',
    'Peter, Paul and Mary',
    'Oxford Comma',
    'Desk'
]

const teammembers = [
    'Juno Diaz',
    'Kid and Play',
    'Bernard Shaw',
    'Ice Cube',
    'Method Man',
    'Gza',
    'Raekwon the Chef'
]

function setupState(): Website {
    let portfolio = [];
    for (var i = 0; i < 10; i++) {
        let name = names[Math.floor(Math.random() * names.length)];
        let context = contexts[Math.floor(Math.random() * contexts.length)];
        let teamSize = Math.floor(Math.random() * 6);
        let teammates = []
        for (var j = 0; j < teamSize;j++) {
            teammates.push(teammembers[Math.floor(Math.random() * teammembers.length)]);
        }
        let imageNumber = Math.ceil(Math.random() * 10);
        let images = [];
        for (var k = 0; k < imageNumber; k++) {
            images.push(`image-${k}.jpg`);
        }
        portfolio.push(new Project(
            i,
            new Meta(
                name,
                context,
                teammates
            ),
            images,
            ''
        ))
    }
    return new Website(portfolio, 'Hello, this is my webiste', 'me.jpg');
}