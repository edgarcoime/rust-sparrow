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
  target: ["web", "es5"],
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(__dirname, "src/index.html")
    }),
  ],
  module: {
    rules: [
      {
        test: /\.(sa|sc|c)ss$/i,
        use: [
          "style-loader",
          "css-loader",
        ],
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  }
}