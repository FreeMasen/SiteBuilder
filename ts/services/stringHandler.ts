
export default class StringHandler {
    static fileName(path: string): string {
        if (!path || path == '') return 'Unset';
        let parts;
        if (path.indexOf('/') > -1) {
            parts = path.split('/');
        } else if (path.indexOf('\\') > -1) {
            parts = path.split('\\');
        }
        return parts[parts.length - 1];
    }
}