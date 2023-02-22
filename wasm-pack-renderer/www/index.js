import {WasmPackRenderer} from "wasm-pack-renderer"

let selectedMaterial = "sand"
let mouseDown = false
let mousePosition = {x: 0, y: 0};


window.changeMaterial = (material) => {
    console.log(material)
    selectedMaterial = material
}

let tickInterval = document.getElementById("tick_interval");

const pausedCheckbox = document.getElementById("paused-checkbox")

pausedCheckbox.onchange = (event) => {
    if (!pausedCheckbox.checked) {
        requestAnimationFrame(renderLoop);
    }
}



const canvas = document.getElementById("sandspiel-canvas");

canvas.onmousedown = () => mouseDown = true;
canvas.onmouseup = () => mouseDown = false;
canvas.onmouseleave = () => mouseDown = false;

canvas.onmousemove = (event) => {
    mousePosition.x = event.clientX;
    mousePosition.y = event.clientY;
};

const ctx = canvas.getContext('2d');

const width = 200;
const height = 200;

const renderer = WasmPackRenderer.new(width, height);

function  getMousePos() {
    let rect = canvas.getBoundingClientRect(), // abs. size of element
        scaleX = canvas.width / rect.width,    // relationship bitmap vs. element for x
        scaleY = canvas.height / rect.height;  // relationship bitmap vs. element for y

    return {
        x: (mousePosition.x - rect.left) * scaleX,   // scale mouse coordinates after they have
        y: (mousePosition.y - rect.top) * scaleY     // been adjusted to be relative to element
    }
}


const draw = () => {
    let data = renderer.get_data();
    let imgData = new ImageData(data, width, height);
    ctx.putImageData(imgData, 0, 0);
}

let currentTick = 0;

const renderLoop = () => {
    if (pausedCheckbox.checked) {
        return;
    }

    currentTick += 1;
    currentTick %= tickInterval.value;

    if (currentTick != 0) {
        requestAnimationFrame(renderLoop)
        return;
    }

    if (mouseDown) {
        let pos = getMousePos();
        renderer.add_material(selectedMaterial, pos.x, pos.y);
    }

    draw();
    renderer.tick();
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);