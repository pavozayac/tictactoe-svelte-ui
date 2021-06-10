<script>
    import { nameX, nameO, scene, mode } from './stores'
    import Button from './Button.svelte'
    import { onMount } from 'svelte';

    $: errOne = $nameX.length < 3
    $: errTwo = $nameO.length < 3

    const saveNames = () => {
        if (!errOne && !errTwo){
            if (Math.random() > 0.5 && $mode == 'multi'){
                [$nameX, $nameO] = [$nameO, $nameX]
            }

            $scene = 'signs'
        }
    }

    onMount(()=>{
        if ($mode == 'single') {
           $nameO = 'Computer'
        }
    })

</script>

<div class="container">
    <label>
        <p><strong>Enter 1st player's name: <span style={`color: ${errOne ? 'red' : '#333'};`}>(min. 3)</span></strong></p>
        <input type="text" bind:value={$nameX} placeholder="Player one"/>
    </label>
    <label>
        <p><strong>Enter 2nd player's name: <span style={`color: ${errTwo ? 'red' : '#333'};`}>(min. 3)</span></strong></p>
        <input type="text" bind:value={$nameO} placeholder="Player two"/>
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

    input::-ms-clear {
        display: none;
    }

    span {
        transition-duration: 200ms;
    }
</style>