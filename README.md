# `syntect` Wasm demo

A little demo of [`syntect`](https://github.com/trishume/syntect), cross-compiled to WebAssembly and running in the browser.

### Usage

Build the Wasm package with:

```sh
wasm-pack build
```

Install the Node dependencies:

```sh
cd www/
npm install
```

Run the local server

```
npm start
```

You should now be able to see the demo running at http://localhost:8080/
