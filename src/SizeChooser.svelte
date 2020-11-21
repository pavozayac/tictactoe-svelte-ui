<script>
    import { size, scene } from './stores'
    import { fade } from 'svelte/transition'
    import Button from './Button.svelte'

    let sizes = [...Array(8).keys()].map(k => k + 3)

    const sizeClick = (x) => {
        $size = x
        $scene = 'board'
    }
</script>


<description>
    <h1>Choose the size of the board</h1>
</description>
<main transition:fade>
    {#each sizes as cell }
        <div on:click={()=>sizeClick(cell)}>
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
        grid-template-columns: repeat(4, 25%);
    }

    div {
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
    }

    div:hover {
        cursor: pointer;
        background: black;
        color: white;
    }

    description {
        display: flex;
        justify-content: center;
    }
</style>