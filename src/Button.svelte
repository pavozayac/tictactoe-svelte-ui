<script>
    import { scene } from './stores'
    export let nextScene;

    const backClick = () => {
        $scene = nextScene
    }
</script>

<div class="container">
    <div class="button" on:click={backClick}>
        <strong>
            <slot></slot>
        </strong>
    </div>
</div>

<style>
    .container {
        margin-top: 30px;
        display: flex;
        justify-content: center;
        width: 100%;
    }

    .button {
        border: solid 3px black;
        padding: 10px;
        transition-duration: 400ms;
        user-select: none;
    }

    .button:hover {
        cursor: pointer;
        background-color: black;
        color: white;
    }
</style>