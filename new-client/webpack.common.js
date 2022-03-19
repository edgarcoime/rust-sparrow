const path = require('path');

module.exports = { 
  entry: {
    app: path.join(__dirname, "src/index.js")
  },
  output: { 
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    clean: true,
  },
  // target: ["web", "es5"],
  experiments: {
    asyncWebAssembly: true,
  }
}