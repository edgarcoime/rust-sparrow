const { merge } = require('webpack-merge');
const common = require('./webpack.common.js');

module.exports = merge(common, {
  stats: {
    errorDetails: true,
    children: true,
  },
  mode: 'production',
});