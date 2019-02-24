const gameWindow: HTMLCanvasElement = document.getElementById('game_window') as HTMLCanvasElement;

const ctx = gameWindow.getContext('2d');

const clock = performance.now();

class Game {
    constructor() {

    }

    update(deltaTime: number) {

    }

    draw(ctx: CanvasRenderingContext2D) {

    }
}

const main = () => {
    const game = new Game();
    const deltaTime = clock - performance.now();
    game.update(deltaTime);
    game.draw(ctx as CanvasRenderingContext2D);
    requestAnimationFrame(main);
};
requestAnimationFrame(main);
