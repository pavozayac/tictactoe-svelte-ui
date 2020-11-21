<script>
    import { size, scene } from './stores'
    import { fade } from 'svelte/transition'
    import Button from './Button.svelte'

    $: board = Array($size).fill(Array($size).fill(0))

    const cellClick = (x, y) => {
		alert(`${x} ${y}`)
	}

</script>

<div transition:fade class="boardContainer" style={`
    grid-template-columns: repeat(${$size}, ${350/$size}px);
    grid-template-rows: repeat(${$size}, ${350/$size}px);
`}>
	{#each board as item, y}
		{#each item as cell, x}
			<div class="cell" on:click={()=>cellClick(x, y)} style={`
                border-bottom: ${y < $size-1 ? '3px solid black' : 'none'};
                border-right: ${x < $size-1 ? '3px solid black' : 'none'};
            `}><div class="inner">{cell == 0 ? 'O' : 'X'}</div>
            </div>
		{/each}
    {/each}
</div>
<Button nextScene="start">
    Return to menu
</Button>

<style>
    .boardContainer {
		display: grid;
        width: 350px;
        height: 350px;
        margin-bottom: 70px;
        grid-auto-flow: dense;
        gap: 0;
	}

	.cell {
        display: flex;
        align-content: center;
        justify-content: center;
	}

    .inner {
        display: flex;
        align-items: center;
        justify-content: center;
        color: black;
        font-size: 3rem;
    }

</style>