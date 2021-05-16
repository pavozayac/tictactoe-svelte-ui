<script>
    import { size, scene, mode, loading } from './stores'
    import { fade } from 'svelte/transition'
    import Button from './Button.svelte'

    $: sizes = [3, 5, 6, 7, 8 ,9, 10]

    const sizeClick = (x) => {
        $size = x
        $scene = 'names'
    }
</script>

<div class="description">
    <h1>Choose the size of the board</h1>
</div>
<main transition:fade>
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
        grid-template-columns: repeat(4, 1fr);
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