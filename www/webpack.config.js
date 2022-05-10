const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  resolve: {
    extensions: [".ts", ".js"],
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /(node_modules | bower_components) /,
        use: {
          loader: 'ts-loader',
        }
      }
    ]
  },
  plugins: [
    new CopyWebpackPlugin({ patterns: ['index.html'] })
  ],
  experiments: {
    asyncWebAssembly: true,
  }
};
