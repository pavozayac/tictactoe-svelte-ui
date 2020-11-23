import App from './App.svelte';
import { board, scene, winner } from './stores'

const app = new App({
	target: document.body,
	props: {
		name: 'world',
	},
});

app.passMove = (x, y) => {
	let b
	const unsubscribe = board.subscribe(value => {
		b = value
	})
	unsubscribe()
	b[y][x] = 1
	board.set(b)
	alert("bruh")
}

app.gameEnd = (sign) => {
	winner.set(sign)
	scene.set('final')
	board.set(Array.from(Array(10), () => new Array(10)))
}

export default app;