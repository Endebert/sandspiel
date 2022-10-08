import {WasmPackRenderer} from "wasm-pack-renderer"


const canvas = document.getElementById("sandspiel-canvas");
const ctx = canvas.getContext('2d');

const width = 200;
const height = 200;

const renderer = WasmPackRenderer.new(width, height);

const draw = () => {
    let data = renderer.get_data();
    let imgData = new ImageData(data, width, height);
    ctx.putImageData(imgData, 0, 0);
}

const renderLoop = () => {
    draw();
    renderer.tick();
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);