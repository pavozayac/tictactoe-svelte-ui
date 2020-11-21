<script>
	import { scene, size } from './stores'
	import Board from './Board.svelte'
	import SizeChooser from './SizeChooser.svelte'
	import Button from './Button.svelte'
	export let name;

	const absFade = (node, {delay = 0, duration = 250}) => {
		const o = getComputedStyle(node).opacity

		return {
			delay,
			duration,
			css: t => `opacity: ${t * o}; position: absolute;`
		}
	}
</script>

<main>
	{#if $scene == 'start'}
		<div transition:absFade class="start">
			<Button nextScene="size">
				Start
			</Button>	
		</div>
	{:else if $scene == 'size'}
		<div transition:absFade>
			<SizeChooser/>
		</div>
	{:else if $scene == 'board' && $size != 0}
		<div transition:absFade>
			<Board/>
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
</style>