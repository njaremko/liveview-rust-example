const path = require('path');

module.exports = {
    mode: 'development',
    entry: './resources/js/liveview.js',
    devtool: 'inline-source-map',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'static/js')
    }
};