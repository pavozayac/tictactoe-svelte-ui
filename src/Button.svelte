<script>
    import { scene, board, size } from "./stores";
    export let nextScene = "start";

    const backClick = () => {
        if (nextScene == 'start'){
            $size = 0
            $board = Array.from(Array(10), () => new Array(10))
        }
        $scene = nextScene;
    };
</script>

<div class="container">
    <div class="button" on:click={backClick} on:click>
        <strong>
            <slot />
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
        -moz-user-select: none;
        -webkit-user-select: none;
        -ms-user-select: none;
        width: 100%;
    }

    .button:hover {
        cursor: pointer;
        background-color: black;
        color: white;
    }
</style>
