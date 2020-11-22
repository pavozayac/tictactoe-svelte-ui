<script>
    import { nameX, nameO, scene } from './stores'
    import Button from './Button.svelte'

    let x = ''
    let o = ''

    $: error = x.length>2 && o.length>2 ? false : true

    const saveNames = () => {
        if (!error){
            $nameX = x
            $nameO = o
            $scene = 'board'
        }
    }

</script>

<div class="container">
    <label>
        <p><strong>Enter player X's name: {error && x.length<3 ? '(min. 3)' : ''}</strong></p>
        <input type="text" bind:value={x}/>
    </label>
    <label>
        <p><strong>Enter player O's name: {error && o.length<3 ? '(min. 3)' : ''}</strong></p>
        <input type="text" bind:value={o}/>
    </label>
    <Button nextScene="names" on:click={saveNames}>
        Save names
    </Button>
</div>

<style>
    .container {
        width: 250px;
    }

    input {
        border: 3px solid black;
    }

    input:focus {
        outline: none;
    }
</style>