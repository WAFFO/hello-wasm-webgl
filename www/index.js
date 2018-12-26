// update this file name with name in Cargo.toml
import('./wasm/hello_webgl.js')
  .then(webgl => webgl.run())
  .catch(console.error);