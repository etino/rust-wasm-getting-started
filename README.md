# Rust WASM Getting Started

## Start a new project

```
$ cargo init --name rust_wasm_getting_started --lib
```

## Install wasm-pack

```
$ cargo install wasm-pack
```

## Edit Cargo.toml

Add following
```
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.72"
```
Reference: [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html#cargotoml)

`wasm-bindgen` package has two primary proposes:
- allows to expose rust function to javascript
- allows you to run javascript function in rust

## Build

```
$ wasm-pack build
```

It create a pkg folder with the all necessary files to import in javascript project:

- `.wasm` file the compiled version of the rust code
- `package.json` the package file to import wasm into javascript project

## Create Javascript project (webpack)

Create a javascript project 

```
$ mkdir client
$ cd client
$ npm init
$ npm install webpack webpack-cli webpack-dev-server
```

Create webpack.config.js inside client project directory

```
const path = require("path");

module.exports = {
  mode: "development",
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  experiments: {
    asyncWebAssembly: true, // required by webpack 5 to enable WebAssembly
  },
};

```

Create entry point `index.js` file
(take a look to `index.js` and `init.js` code)


# Run webpack inside client directory

```
$ npx webpack
```

# Create `index.html` inside `client/dist`

Create an `index.html` to import `bundle.js` script

```
<script src="/bundle.js"></script>
```

# Run development server

Create webpack development server configuration in `webpack.config.js`:

```
  devServer: {
    contentBase: path.join(__dirname, "dist"),
    compress: true,
    port: 9000,
  },
```


Run `webpack serve` command

```
$ npx webpack serve
```

Open `http://localhost:9000`, and check the result in the console.


Reference
[Getting Started with WebAssembly and Rust: A First Look](https://www.youtube.com/watch?v=YHJjmsw_Sx0) by Engeneer Man ([Youtube Channel](https://www.youtube.com/channel/UCrUL8K81R4VBzm-KOYwrcxQ))