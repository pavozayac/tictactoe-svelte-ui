<script>
	import { scene, size, mode, winner } from './stores'
	import Board from './Board.svelte'
	import SizeChooser from './SizeChooser.svelte'
	import Button from './Button.svelte'
	import NameChooser from './NameChooser.svelte'
	import Final from './Final.svelte'
	import SignReveal from './SignReveal.svelte'
	import { fade } from './effects'

	export let name;

</script>

<main>
	{#if $scene == 'start'}
		<div transition:fade class="start">
			<Button nextScene="size" on:click={()=>$mode='single'}>
				Single player
			</Button>
			<Button nextScene="size" on:click={()=>$mode='multi'}>
				Multiplayer
			</Button>
			<Button on:click={()=>window.external.invoke(JSON.stringify({ msg: 'exit'}))}>
				Exit
			</Button>
		</div>
	{:else if $scene == 'size'}
		<div transition:fade class="chooser">
			<SizeChooser/>
		</div>
	{:else if $scene == 'board' && $size != 0}
		<div transition:fade class="board">
			<Board/>
		</div>
		{#if $winner != 'none'}
		<div transition:fade class="final">
			<Final/>
		</div>
		{/if}
	{:else if $scene == 'names'}
		<div transition:fade class="names">
			<NameChooser />
		</div>
	{:else if $scene == 'signs'}
		<div transition:fade class="names">
			<SignReveal />
		</div>
	{/if}
</main>

<style>
	:global(body){
		margin: 0;
		padding: 0;
		background: white;
		-moz-user-select: none;
        -webkit-user-select: none;
        -ms-user-select: none;
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

	.start, .chooser, .board, .final, .names {
		position: absolute;
	}

	.start {
		width: 150px;
	}

	.final {
		opacity: 0.8;
	}
</style>