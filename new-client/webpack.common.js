const path = require('path');
const HtmlWebpackPlugin = require("html-webpack-plugin");

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
    })
  ],
}