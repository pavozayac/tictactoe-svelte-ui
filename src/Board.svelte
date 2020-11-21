<script>
    import { size, board } from './stores'
    import { fade } from 'svelte/transition'
    import Button from './Button.svelte'
    import {onMount} from 'svelte'

    $board = Array.from(Array($size), () => new Array($size))

    const cellClick = (x, y) => {
        $board[y][x] = 1
	}

</script>

<div transition:fade class="boardContainer" style={`    
    grid-template-columns: repeat(${$size}, ${350/$size}px);
    grid-template-rows: repeat(${$size}, ${350/$size}px);
`}>
	{#each $board as item, y}
		{#each item as cell, x}
			<div class="cell" on:click={()=>cellClick(x, y)} style={`
                border-bottom: ${y < $size-1 ? '3px solid black' : 'none'};
                border-right: ${x < $size-1 ? '3px solid black' : 'none'};
                cursor: ${cell ? 'default' : 'pointer'};
            `}><div class="inner">
                {#if cell == 1}
                <svg width="80" height="80" transition:fade>
                  <circle cx="40" cy="40" r="35" stroke="black" stroke-width="4" fill="white"/>
                </svg>
                {:else if cell == -1}
                <svg width="80" height="80" transition:fade={{duration: 200}}>
                    <line x1="5" y1="5" x2="75" y2="75" stroke="black" stroke-width="4" />
                    <line x1="5" y1="75" x2="75" y2="5" stroke="black" stroke-width="4" />
                </svg>
                {/if}

                </div>
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