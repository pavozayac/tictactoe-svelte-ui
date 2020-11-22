<script>
    import { size, scene, mode } from './stores'
    import { fade } from 'svelte/transition'
    import Button from './Button.svelte'

    $: sizes = $mode == 'multi' ? [...Array(8).keys()].map(k => k + 3) : [3, 4]

    const sizeClick = (x) => {
        $size = x
        $scene = 'names'
    }
</script>

<div class="description">
    <h1>Choose the size of the board</h1>
</div>
<main transition:fade style={`
    grid-template-columns: repeat(${$mode == 'multi' ? 4 : 2}, 1fr);    
`}>
    {#each sizes as cell }
        <div class="button" on:click={()=>sizeClick(cell)}>
            <span>{cell}</span>
        </div>        
    {/each}
</main>
<Button nextScene="start">
    Return to menu
</Button>


<style>
    main {
        display: grid;
    }

    .button {
        padding: 30px;
        margin: 10px;
        border: 3px solid black;
        text-align: center;
        justify-items: center;
        transition-duration: 300ms;
        display: flex;
        justify-content: center;
    }

    span {
        text-align: center;
        font-size: 2rem;
        -moz-user-select: none;
        -webkit-user-select: none;
        -ms-user-select: none;
    }

    .button:hover {
        cursor: pointer;
        background: black;
        color: white;
    }

    .description {
        display: flex;
        justify-content: center;
        -moz-user-select: none;
        -webkit-user-select: none;
        -ms-user-select: none;
    }
</style>