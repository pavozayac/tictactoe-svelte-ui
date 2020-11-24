<script>
    import { size, board, mode, winner } from './stores'
    import { fade } from 'svelte/transition'
    import Button from './Button.svelte'
    import { onDestroy, onMount } from 'svelte';

    $board = Array.from(Array(10), () => new Array(10))

    const cellClick = async (x, y) => {
        if (!$board[y][x] && !moveLocked){
            $board[y][x] = currentPlayer
            window.external.invoke(JSON.stringify({msg: "move", x: x, y: y}))
            if ($mode == 'multi'){
                switchPlayer()
            } else {
                moveLocked = true
                window.external.invoke(JSON.stringify({msg: "computeMove"}))
                moveLocked = false
            }
        }
    }
    
    
    let currentPlayer = -1
    let moveLocked = false

    const switchPlayer = () => {
        currentPlayer = -1 * currentPlayer
    }

    onMount(()=>{
        window.external.invoke(JSON.stringify({msg: "init", size: $size}))
    })

    onDestroy(()=>{
        $board = Array.from(Array(10), () => new Array(10))
    })
</script>

<div transition:fade class="boardContainer" style={`    
    grid-template-columns: repeat(${$size}, ${350/$size}px);
    grid-template-rows: repeat(${$size}, ${350/$size}px);
`}>
	{#each $board.slice(0, $size) as item, y}
		{#each item.slice(0, $size) as cell, x}
			<div class="cell" on:click={()=>cellClick(x, y)} style={`
                border-bottom: ${y < $size-1 ? '3px solid black' : 'none'};
                border-right: ${x < $size-1 ? '3px solid black' : 'none'};
                cursor: ${cell ? 'default' : 'pointer'};
            `}><div class="inner">
                {#if cell == 1}
                <svg width="80%" height="80%" transition:fade>
                  <circle cx="50%" cy="50%" r="37%" stroke="aqua" stroke-width="4" fill="white"/>
                </svg>
                {:else if cell == -1}
                <svg width="70%" height="70%" transition:fade={{duration: 200}}>
                    <line x1="7%" y1="7%" x2="93%" y2="93%" stroke="orange" stroke-width="4" />
                    <line x1="7%" y1="93%" x2="93%" y2="7%" stroke="orange" stroke-width="4" />
                </svg>
                {/if}

                </div>
            </div>
		{/each}
    {/each}
</div>
<Button nextScene="start" on:click={()=>$winner='none'}>
    Return to menu
</Button>

<style>
    .boardContainer {
		display: grid;
        width: 350px;
        height: 350px;
        margin-bottom: 40px;
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