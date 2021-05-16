import App from './App.svelte';
import { board, scene, winner, nodes, loading } from './stores'

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
	b[x][y] = 1
	board.set(b)
}

app.gameEnd = (sign) => {
	winner.set(sign)
}

app.passNodes = (n) => {
	nodes.set(n)
}

app.changeScene = (s) => {
	scene.set(s)
}

app.setLoading = (l) => {
	loading.set(l)
}

window.onclose = () => {
	console.log("bruh")
}

window.close()

export default app;