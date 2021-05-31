<script>
    import { size, board, mode, winner, nameX, nameO, moveLocked } from './stores'
    import Button from './Button.svelte'
    import { onDestroy, onMount } from 'svelte';
    import { fade } from './effects'

    $board = Array.from(Array(10), () => new Array(10))
    $moveLocked = false

    const cellClick = (x, y) => {
        if ($board[y][x] != 1 && $board[y][x] != -1 && $moveLocked == false && $winner == 'none'){
            $moveLocked = true
            $board[y][x] = currentPlayer
            window.external.invoke(JSON.stringify({msg: "move", x: x, y: y}))
            if ($mode == 'multi'){
                switchPlayer()
                $moveLocked = false
            } else {
                window.external.invoke(JSON.stringify({msg: "computeMove"}))
                window.external.invoke(JSON.stringify({msg: "refreshBoard"}))
                $moveLocked = false
            }
        }
    }
    
    let currentPlayer = -1

    const switchPlayer = () => {
        currentPlayer = -1 * currentPlayer
    }

    onMount(()=>{
        $board = Array.from(Array(10), () => new Array(10))

        window.external.invoke(JSON.stringify({msg: "init", size: $size, player_one: $nameX, player_two: $nameO}))

        $moveLocked = false

        setInterval(()=>{
            window.external.invoke(JSON.stringify({msg: "refreshBoard"}))
        }, 1000)

    })

    onDestroy(()=>{
        $board = Array.from(Array(10), () => new Array(10))
        $winner = 'none'
        $moveLocked = false
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