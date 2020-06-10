# `syntect` Wasm demo

A little demo of [`syntect`](https://github.com/trishume/syntect), compiled to WebAssembly and running in the browser.

https://syntect-wasm-demo.alexpeattie.com/

<img src="https://user-images.githubusercontent.com/636814/84265191-5b733e00-ab1a-11ea-8312-1ad424307988.png" alt="" width=600>

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
