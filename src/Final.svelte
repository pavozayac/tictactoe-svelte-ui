<script>
    import { winner, nameO, nameX } from './stores'
    let message

    $: {
        if ($winner == 'o'){
            message = `${$nameO} has won!`
        } else if ($winner == 'x') {
            message = `${$nameX} has won!`
        } else{
            message = "It's a tie!"
        } 
    }
</script>

<div class="container">
<div>
    <h1>{message}</h1>
</div>
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
    }
</style>