
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
        let ret = '';
        ret += StringHandler.twoDigit(Math.round(red).toString(16));
        ret += StringHandler.twoDigit(Math.round(green).toString(16));
        ret += StringHandler.twoDigit(Math.round(blue).toString(16));
        return ret;
    }

    static twoDigit(val: string) {
        return ('0' + val).substr(-2);
    }

    static fromHex(hex: string): {red: number, green: number, blue: number} {
        return {
            red: parseInt(hex.substr(0,2), 16),
            green: parseInt(hex.substr(2,2), 16),
            blue: parseInt(hex.substr(4,2), 16),
        }
    }
    static parseDate(dt: Date): string {
        var m = dt.getMonth() + 1;
        var d = dt.getDate();
        var y = dt.getFullYear().toString().substr(-2);
        var h = ('0' + dt.getHours()).substr(-2);
        var min = ('0' + dt.getMinutes()).substr(-2);
        var s = ('0' + dt.getSeconds()).substr(-2);
        return m + '/' + d + '/' + y + ' ' + h + ':' + min + ':' + s;
    }
}