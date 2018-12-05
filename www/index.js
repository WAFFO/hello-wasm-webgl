(async () => {
    const webGL = await import('./wasm/hello_webgl.js');

    const rustGL = webGL.RustGL.new();
    rustGL.draw();

    const renderLoop = () => {
        rustGL.draw();
        requestAnimationFrame(renderLoop);
    }

    requestAnimationFrame(renderLoop);
})();