const path = require('path');

const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
	entry: './web/index.ts',
	output: {
		filename: '[name].[contenthash].js',
		path: path.resolve(__dirname, 'dist')
	},
	
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				use: 'ts-loader',
				exclude: '/node_modules/'
			},
			{
				test: /\.s[ac]ss$/i,
				use: [
					// Creates `style` nodes from JS strings
					'style-loader',
					// Translates CSS into CommonJS
					'css-loader',
					// Compiles Sass to CSS
					'sass-loader',
				],
			},
		]
	},
	
	resolve: {
		extensions: ['.tsx', '.ts', '.js']
	},
	
	plugins: [
		
		new HtmlWebpackPlugin({
			template: './web/index.html'
		}),
		
		new WasmPackPlugin({
			crateDirectory: path.resolve(__dirname, '.')
		})
	]
}