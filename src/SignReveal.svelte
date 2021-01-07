<script>
    import Button from './Button.svelte'
    import { nameX, nameO, scene, size } from './stores'
</script>
{@debug $scene}
{@debug $size}
<div class="container">
    <div class="starts nameSection">
        <svg class="icon">
            <line x1="7%" y1="7%" x2="93%" y2="93%" stroke="orange" stroke-width="3" />
            <line x1="7%" y1="93%" x2="93%" y2="7%" stroke="orange" stroke-width="3" />
        </svg>starts
    </div>
    <div class="names">
        <div class="nameSection">
            <svg class="icon">
                <line x1="7%" y1="7%" x2="93%" y2="93%" stroke="orange" stroke-width="4" />
                <line x1="7%" y1="93%" x2="93%" y2="7%" stroke="orange" stroke-width="4" />
            </svg>
            <div class="playerName">{$nameX}</div>
        </div>

        <div class="nameSection">
            <svg class="icon">
                <circle cx="50%" cy="50%" r="40%" stroke="aqua" stroke-width="4" fill="white"/>
            </svg>
            <div class="playerName">{$nameO}</div>
        </div>
    </div>
    
    <Button nextScene="board">
        Start game
    </Button>
    <Button nextScene="start">
        Return to menu
    </Button>
</div>

<style>
    .container {
        width: 250px;
        display: flex;
        align-items: center;
        flex-direction: column;
    }

    .names {
        width: 90%;
    }
    
    .starts {
        font-size: 1.5rem;
        border: 3px solid black;
        padding: 5px 11px 11px 15px;
        margin-bottom: 100px;
    }

    .starts > .icon {
        width: 23px;
        height: 23px;
        margin-top: 7px;
        margin-right: 10px;
    }

    .nameSection {
        display: flex;
        justify-content: center;
        align-items: center;
        max-width: 90%;
        margin-bottom: 20px;
    }

    .playerName {
        overflow: hidden;
        position: relative;
        display: inline-block;
        text-overflow: ellipsis;
        white-space: nowrap;
        font-size: 2rem;
        font-weight: 700;
        max-width: 70%;
    }

    .playerName::before {
        top: 0;
        left: 0;
        margin: none !important;
        content: "";
        position: absolute;
        background: black;
        width: 100%;
        height: 100%;
        animation: reveal forwards 1s;
    }

    .icon {
        display: inline-block;
        width: 40px;
        height: 40px;
        margin-top: 10px;
        margin-right: 20px;
    }

    @keyframes reveal {
        from {
            height: 100%;
        }

        to {
            height: 0%;
        }
    }
</style>