const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const commonConfig = {
    devtool: 'inline-source-map',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js'
    },
    resolve: {
        extensions: ['.ts', '.tsx', '.js', '.jsx']
    },
    module: {
      rules: [
          {
            test: /\.tsx?$/,
            enforce: 'pre',
            loader: 'tslint-loader',
            options: {
              typeCheck: true,
              emitErrors: true
            }
          },
          {
            test: /\.tsx?$/,
            loader: 'ts-loader'
          },
          {
            test: /\.node$/,
            use: 'node-loader'
          }
        ]
    },
    node: {
        __dirname: false
    },
};

var main = {
    target: 'electron-main',
    entry: { main: './src/main.ts' },
    ...commonConfig
};

var renderer = {
    target: 'electron-renderer',
    entry: { renderer: './src/app/index.tsx' },
    plugins: [ new HtmlWebpackPlugin({title: "Electron App"}) ],
    ...commonConfig
};

module.exports = [ main, renderer ];