const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const webpack = require('webpack');

// 读取根目录的 domains.json
const domainConfig = require('./domains');

// 将 domains.json 中所有的域名数组合并成一个扁平的数组
const allDomains = Object.values(domainConfig).flat();

module.exports = {
    entry: {
        background: './src/background.ts',
        content: './src/content.ts',
        popup: './src/popup/index.tsx',
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js',
        clean: true,
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
            {
                test: /\.css$/,
                use: ['style-loader', 'css-loader','postcss-loader'/*{
                    loader: 'postcss-loader',
                    options: {
                        postcssOptions: {
                            config: path.resolve(__dirname, 'postcss.config.js'),
                        },
                    },
                }*/],
            },
            {
                test: /\.less$/,
                use: ['style-loader', 'css-loader', 'postcss-loader', 'less-loader'],
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './src/popup/popup.html',
            filename: 'popup.html',
            chunks: ['popup'],
        }),
        // 动态替换 manifest.json 中的 __CONFIG_DOMAINS__
        new CopyWebpackPlugin({
            patterns: [
                {
                    from: 'manifest.json',
                    to: 'manifest.json',
                    transform(content) {
                        const manifestStr = content.toString();
                        // 将占位符替换为合并后的域名数组字符串（去除外层括号）
                        const updatedManifest = manifestStr.replace(
                            /"__CONFIG_DOMAINS__"/g,
                            JSON.stringify(allDomains).slice(1, -1)
                        );
                        return Buffer.from(updatedManifest);
                    }
                },
                { from: 'src/icons', to: 'icons' },
            ],
        }),

        // 将合并后的域名注入到 JS 业务代码中
        new webpack.DefinePlugin({
            __CONFIG_DOMAINS__: JSON.stringify(allDomains),
        }),
    ],
};