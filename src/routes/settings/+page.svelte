<script lang="ts">
    function toggleTheme() {
        window.document.body.classList.toggle("dark-mode");
    }
</script>

<div>
    <div class="theme">
        <h1>Change Theme</h1>
        <button
            class="theme-btn"
            on:click={toggleTheme}
            aria-label="Change the theme of the app"
        >
            Change Theme <img class="theme-icon" src="/icons/theme.svg" alt="Change theme" />
        </button>
    </div>
</div>

<style>
    .theme-btn {
        color: white;
        background: var(--panel);
        border: none;
        padding: 0.25rem;
        border-radius: 6px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .theme-btn:hover {
        background: var(--hover);
    }

    .theme-icon {
        width: 3rem;
        height: 3rem;
        filter: brightness(0) invert(1);
    }
</style>
