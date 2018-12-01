// For more comments about what's going on here, check out the `hello_world`
// example.
import('./wasm/hello_webgl.js')
  .then(webgl => webgl.draw())
  .catch(console.error);
