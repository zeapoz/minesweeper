import { memory } from "minesweeper/minesweeper_bg.wasm";
import { Board, Tile, TileState } from "minesweeper";

const TILE_COLOR = "#353535";
const EMPTY_COLOR = "#C0C0C0";
const MINE_COLOR = "#000000";

const NUMBER_COLORS = new Map([
  [0, EMPTY_COLOR],
  [1, "#0100FE"],
  [2, "#008001"],
  [3, "#FE0000"],
  [4, "#00007F"],
  [5, "#670000"],
  [6, "#008081"],
  [7, MINE_COLOR],
  [8, "#808080"],
])

const SIZE = 20;
const TILE_SIZE = 36;

const board = Board.new(SIZE, SIZE);

const canvas = document.getElementById("minesweeper-canvas") as HTMLCanvasElement;
canvas.width = SIZE * TILE_SIZE;
canvas.height = SIZE * TILE_SIZE;
const ctx = canvas.getContext("2d");

// Handle left click event
canvas.addEventListener("click", event => {
  if (board.has_lost()) {
    return;
  }

  let coords = getMouseCoords(event);
  let row = coords[0];
  let col = coords[1];

  board.uncover_tile(row, col);

  draw();
});

// Handle right click event and disable context menu
canvas.addEventListener("contextmenu", event => {
  if (board.has_lost()) {
    return;
  }

  let coords = getMouseCoords(event);
  let row = coords[0];
  let col = coords[1];

  board.flag_tile(row, col);

  draw();

  // Disables context menu pop-up
  event.preventDefault();
});

const getMouseCoords = (event: MouseEvent) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / TILE_SIZE), canvas.height);
  const col = Math.min(Math.floor(canvasLeft / TILE_SIZE), canvas.width);

  return [row, col];
}

const draw = () => {
  // Clear screen
  ctx.fillStyle = TILE_COLOR;
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  drawGrid();
  drawTiles();
}

const getIndex = (x: number, y: number) => {
  return y * SIZE + x;
}

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = MINE_COLOR;

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

  const uncoveredPtr = board.uncovered();
  const uncovered = new Uint8Array(memory.buffer, uncoveredPtr, SIZE ** 2);

  const neighborsPtr = board.neighbors();
  const neighbors = new Uint8Array(memory.buffer, neighborsPtr, SIZE ** 2);

  for (let j = 0; j < SIZE; j++) {
    for (let i = 0; i < SIZE; i++) {
      let index = getIndex(i, j);
      if (uncovered[index] === TileState.Uncovered) {
        // Draw clear color
        ctx.fillStyle = EMPTY_COLOR;
        ctx.fillRect(i * TILE_SIZE + 1, j * TILE_SIZE + 1, TILE_SIZE - 2, TILE_SIZE - 2);

        if (tiles[index] === Tile.Mine) {
          // Draw mine
          ctx.fillStyle = MINE_COLOR;
          ctx.beginPath();
          ctx.arc(i * TILE_SIZE + TILE_SIZE / 2, j * TILE_SIZE + TILE_SIZE / 2, TILE_SIZE / 3, 0, 2 * Math.PI);
          ctx.fill();
        } else {
          // Draw mine neighbor count
          ctx.font = TILE_SIZE + "px Arial";
          let n = neighbors[index];
          ctx.fillStyle = NUMBER_COLORS.get(n);
          ctx.textAlign = "center";
          ctx.fillText(n.toString(), i * TILE_SIZE + TILE_SIZE / 2, j * TILE_SIZE + TILE_SIZE * 0.85);
        }
      } else if (uncovered[index] === TileState.Flagged) {
        // Draw flag icon
        ctx.fillStyle = NUMBER_COLORS.get(3);
        ctx.beginPath();
        ctx.arc(i * TILE_SIZE + TILE_SIZE / 2, j * TILE_SIZE + TILE_SIZE / 2, TILE_SIZE / 3, 0, 2 * Math.PI);
        ctx.fill();
      }
    }
  }
}

draw();
