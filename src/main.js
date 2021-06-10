import App from './App.svelte';
import { board, winner, nameX, nameO, moveLocked } from './stores'

const app = new App({
	target: document.body,
});

app.passMove = (x, y) => {
	board.update(b => { let up = b; up[x][y] = 1; return up} )
	moveLocked.set(false)
	//playerIndicator.update(n => n *= -1)
}

app.gameEnd = (sign) => {
	winner.set(sign)
}

app.setNames = (player_one, player_two) => {
	nameX.set(player_one)
	nameO.set(player_two)
}	

app.refreshBoard = (refreshed) => {
	board.set(JSON.parse(refreshed))
}

document.addEventListener('contextmenu', e=>e.preventDefault())

export default app;