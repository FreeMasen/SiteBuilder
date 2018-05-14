
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

    static colorString(red: number, blue: number, green: number, alpha: number): string {
        return `rgba(${red.toFixed(0)}, ${blue.toFixed(0)}, ${green.toFixed(0)}, ${alpha.toFixed(2)}`;
    }

    static hexColor(red: number, blue: number, green: number): string {
        let ret = '#';
        ret += red.toString(16);
        ret += blue.toString(16);
        ret += green.toString(16);
        return ret;
    }
}