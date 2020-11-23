<script>
    import { winner, nameO, nameX } from './stores'
    import Button from './Button.svelte'

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

<div>
    <h1>{message}</h1>
</div>
<Button nextScene="start">
    Return to menu
</Button>

<style>
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
        position: absolute;
    }
</style>