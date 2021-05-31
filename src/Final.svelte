<script>
    import { winner, nameO, nameX } from './stores'
    let displayName

    $: {
        if ($winner == 'o'){
            displayName = $nameO
        } else if ($winner == 'x') {
            displayName = $nameX
        }
    }
</script>

<div class="container">
    {#if $winner == 'tie'}
        <h1>It's a tie!</h1>
    {:else if $winner == 'o' || $winner == 'x'}
        <h1>
            <span class="name">{displayName}</span> 
            <br/>
            has won!
        </h1>
    {/if}
</div>

<style>
    .container {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        background: black;
        color: white;
        width: 250px;
        height: auto;
        margin: 0;
        border: 3px solid black;
        overflow: hidden;
        text-overflow: ellipsis;

    }

    .name {
        overflow:hidden;
        display: inline-block;
        position: relative;
        max-width: 225px;
        white-space: nowrap;
        text-align: center;
        text-overflow: ellipsis;
    }
</style>