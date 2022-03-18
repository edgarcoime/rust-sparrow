const path = require('path');
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require('webpack');

module.exports = { 
  entry: {
    app: path.join(__dirname, "src/index.js")
  },
  output: { 
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    clean: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(__dirname, "src/index.html")
    }),
    // new WasmPackPlugin({
    //   crateDirectory: path.resolve(__dirname, "../libs/simulation-wasm")
    // }),
    // new webpack.ProvidePlugin({
    //   TextEncoder: ["text-encoding", "TextEncoder"],
    //   TextDecoder: ["text-encoding", "TextDecoder"],
    // })
  ],
  experiments: {
    asyncWebAssembly: true,
  }
}