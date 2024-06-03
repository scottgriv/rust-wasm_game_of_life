import init, { Universe } from './pkg/rust_wasm_game_of_life.js';

async function run() {
    await init();
    const universe = Universe.new();
    const pre = document.getElementById('game-of-life');

    const renderLoop = () => {
        pre.textContent = universe.render();
        universe.tick(); // Progress the game state
        requestAnimationFrame(renderLoop); // Schedule the next frame
    };

    renderLoop(); // Start the loop
}

run();
