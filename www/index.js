import('./wasm/hello_webgl.js')
  .then(webgl => webgl.draw())
  .catch(console.error);
alert('Hello');