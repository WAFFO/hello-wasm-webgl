import('./wasm/hello_webgl.js')
  .then(webgl => webgl.run())
  .catch(console.error);