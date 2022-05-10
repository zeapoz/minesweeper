import { memory } from "minesweeper/minesweeper_bg.wasm";
import { Board, Tile } from "minesweeper";

const SIZE = 20;
const TILE_SIZE = 20;

const board = Board.new(SIZE, SIZE);

const canvas = document.getElementById("minesweeper-canvas") as HTMLCanvasElement;
canvas.width = SIZE * TILE_SIZE;
canvas.height = SIZE * TILE_SIZE;
const ctx = canvas.getContext("2d");

const getIndex = (x: number, y: number) => {
    return y * SIZE + x;
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = "#000000";

    for (let i = 0; i <= SIZE; i++) {
        ctx.moveTo(i * TILE_SIZE, 0)
        ctx.lineTo(i * TILE_SIZE, SIZE * TILE_SIZE)
    }

    for (let j = 0; j <= SIZE; j++) {
        ctx.moveTo(0, j * TILE_SIZE)
        ctx.lineTo(SIZE * TILE_SIZE, j * TILE_SIZE)
    }

    ctx.stroke();
}

const drawTiles = () => {
    const tilesPtr = board.tiles();
    const tiles = new Uint8Array(memory.buffer, tilesPtr, SIZE ** 2);

    for (let j = 0; j < SIZE; j++) {
        for (let i = 0; i < SIZE; i++) {
            let index = getIndex(i, j);
            if (tiles[index] === Tile.Mine) {
                ctx.beginPath();
                ctx.arc(i * SIZE + SIZE / 2, j * SIZE + SIZE / 2, SIZE / 3, 0, 2 * Math.PI);
                ctx.fill();
            }
        }
    }
}

drawGrid();
drawTiles();
