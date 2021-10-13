const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const path = require('path');

module.exports = {
  entry: './src/index.ts',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'assets'),
  },
  resolve: {
    extensions: [".js", ".ts"],
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        include: path.resolve(__dirname, 'src'),
        use: [
          MiniCssExtractPlugin.loader,
          "css-loader", "postcss-loader",
        ],
        exclude: /node_modules/,
      },
      {
        test: /\.ts?$/,
        include: path.resolve(__dirname, 'src'),
        use: "babel-loader",
        exclude: /node_modules/,
      },
    ]
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "styles.css",
      chunkFilename: "styles.css"
    }),
  ],
};