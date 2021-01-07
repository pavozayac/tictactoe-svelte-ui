<script>
    import { nameX, nameO, scene } from './stores'
    import Button from './Button.svelte'

    let nameOne = ''
    let nameTwo = ''

    $: errOne = nameOne.length < 3
    $: errTwo = nameTwo.length < 3

    const saveNames = () => {
        if (!errOne && !errTwo){
            if (Math.round(Math.random())){
                $nameX = nameOne
                $nameO = nameTwo
            } else {
                $nameX = nameTwo
                $nameO = nameOne
            }
            
            $scene = 'signs'
        } else {

        }
    }

</script>

<div class="container">
    <label>
        <p><strong>Enter 1st player's name: <span style={`color: ${errOne ? 'red' : '#333'};`}>(min. 3)</span></strong></p>
        <input type="text" bind:value={nameOne} placeholder="Player one"/>
    </label>
    <label>
        <p><strong>Enter 2nd player's name: <span style={`color: ${errTwo ? 'red' : '#333'};`}>(min. 3)</span></strong></p>
        <input type="text" bind:value={nameTwo} placeholder="Player two"/>
    </label>
    <Button nextScene="names" on:click={saveNames}>
        Save names
    </Button>
    <Button nextScene="start">
        Return to menu
    </Button>
</div>

<style>
    .container {
        width: 250px;
    }

    label {
        width: 100%;
    }

    input {
        width: 100%;
        border: 3px solid black;
    }

    input:focus {
        outline: none;
    }

    span {
        transition-duration: 200ms;
    }
</style>