<script>
	import { scene, size, mode } from './stores'
	import Board from './Board.svelte'
	import SizeChooser from './SizeChooser.svelte'
	import Button from './Button.svelte'
	import NameChooser from './NameChooser.svelte'

	export let name;

	const absFade = (node, {delay = 0, duration = 250}) => {
		const o = getComputedStyle(node).opacity

		return {
			delay,
			duration,
			css: t => `opacity: ${t * o};`
		}
	}
</script>

<main>
	{#if $scene == 'start'}
		<div transition:absFade class="start">
			<Button nextScene="size" on:click={()=>$mode='single'}>
				Singleplayer
			</Button>
			<Button nextScene="size" on:click={()=>$mode='multi'}>
				Multiplayer
			</Button>
			<Button on:click={()=>window.external.invoke(JSON.stringify({ msg: 'exit'}))}>
				Exit
			</Button>
		</div>
	{:else if $scene == 'size'}
		<div transition:absFade class="chooser">
			<SizeChooser/>
		</div>
	{:else if $scene == 'board' && $size != 0}
		<div transition:absFade class="board">
			<Board/>
		</div>
	{:else if $scene == 'names'}
		<div transition:absFade>
			<NameChooser />
		</div>
	{/if}
</main>

<style>
	:global(body){
		margin: 0;
		padding: 0;
		background: white;
	}

	main {
		text-align: center;
		margin: 0 auto;
		background: white;
		height: 100%;
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}

	.start, .chooser, .board {
		position: absolute;
	}

	.start {
		width: 150px;
	}
</style>