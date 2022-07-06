import { Universe } from "wasm-game-beta";
import { memory } from "wasm-game-beta/wasm_game_beta_bg";
const universe = Universe.new();

const WIDTH = universe.get_width();
const HEIGHT = universe.get_height();
const TILE_SIZE = 10;

let map;
let mapPtr;

const gameContainer = document.getElementsByClassName('game-container')[0];
gameContainer.width = WIDTH * TILE_SIZE;
gameContainer.height = HEIGHT * TILE_SIZE;
var ctx = gameContainer.getContext("2d");

//========================================
// temp until sprites are inserted
const tempColors = { // for before sprites
    0: '#ffffff',
    1: '#0000ff', // player
    2: '#ff0000', // platform,
    default: '#ffffff',
    // default: '#000000', // for debug, seems like wasm memory array is contaminated somehow, proly pointer stuff
}
//========================================


let keysPressed = {};
document.addEventListener('keydown', (e) => {
    // e = e || window.event;
    keysPressed[e.key] = true;
});
document.addEventListener('keyup', (e) => {
    // e = e || window.event;
    delete keysPressed[e.key];
});

const renderLoop = () => {

    universe.tick();
    mapPtr = universe.repr_map();
    map = new Uint8Array(memory.buffer, mapPtr, universe.get_width() * universe.get_height());
    
    if (keysPressed.ArrowUp) {
        console.log("pressed up");
        universe.dispatch("PLAYER_JUMP");
    }
    if (keysPressed.ArrowRight) {
        console.log("pressed right");
        universe.dispatch("PLAYER_MOVE_RIGHT");
    } else if (keysPressed.ArrowLeft) {
        console.log("pressed left");
        universe.dispatch("PLAYER_MOVE_LEFT");
    }
    
    ctx.clearRect(0, 0, WIDTH * TILE_SIZE, HEIGHT * TILE_SIZE);

    // console.log(map); // temp
    
    map.forEach((tile, i) => {
        const x = Math.floor(i % WIDTH);
        const y = Math.floor(i / WIDTH);
        //========================================
        // temp until sprites are inserted
        const tileColor = tempColors[tile] || tempColors.default;
        ctx.fillStyle = tileColor; // temp until sprites are inserted
        //========================================
        ctx.fillRect(
            x * TILE_SIZE,
            (HEIGHT * TILE_SIZE - y * TILE_SIZE) - TILE_SIZE,
            TILE_SIZE,
            TILE_SIZE
        );
        // console.log("Player found at "+i+". x, y: "+x+","+y); // temp
    });

    setTimeout(() => {
        requestAnimationFrame(renderLoop);
      }, 30);
};

function getIndex(row, column) {
    return (row * WIDTH + column);
}

requestAnimationFrame(renderLoop);