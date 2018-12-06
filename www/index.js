(async () => {
    const webGL = await import('./wasm/hello_webgl.js');

    const Engine = webGL.Engine.new();
    Engine.tick();

    const renderLoop = () => {
        Engine.tick();
        requestAnimationFrame(renderLoop);
    }

    requestAnimationFrame(renderLoop);
})();