const path = require('path');

module.exports = {
    entry: {
        app: path.resolve(__dirname, 'ts', 'app.tsx')
    },
    output: {
        path: path.resolve(__dirname, 'src'),
        filename: '[name].js'
    },
    resolve: {
        extensions: ['.ts', '.tsx', '.js', '.jsx']
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'awesome-typescript-loader'
            }
        ]
    },
    mode: 'development'
}