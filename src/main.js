import App from './App.svelte';
import { board } from './stores'

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

app.bruh = (msg) => {
	alert(msg)
}

export default app;