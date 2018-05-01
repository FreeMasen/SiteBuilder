const path = require('path');

module.exports = function(env) {
    let out = env == 'prod' ? 
                path.resolve(__dirname, 'src', 'assets') :
                path.resolve(__dirname);
   let config = {
       entry: {
            app: path.resolve(__dirname, 'ts', 'app.tsx')
        },
        output: {
            path: out,
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
        mode: env == 'prod' ? 'production' : 'development'
    }
    return config;
}