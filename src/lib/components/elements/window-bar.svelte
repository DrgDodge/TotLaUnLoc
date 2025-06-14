<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";

    let isDragging = false;
    let isMouseOver = false;
    let isExpanded = false;

    $: isExpanded = isDragging || isMouseOver;

    const appWindow = getCurrentWindow();

    function minimize() {
        appWindow.minimize();
    }

    function close() {
        appWindow.close();
    }

    function handleMouseDown(event: MouseEvent) {
        function startDrag() {
            appWindow.startDragging();
        }
        const target = event.target as HTMLElement;

        if (target.closest("button")) return;
        isDragging = true;
        startDrag();

        const handleMouseUp = () => {
            isDragging = false;
            window.removeEventListener("mouseup", handleMouseUp);
        };

        window.addEventListener("mouseup", handleMouseUp);
    }
</script>

<div
    role="menuitem"
    tabindex="0"
    class="titlebar"
    class:expanded={isExpanded}
    on:mouseenter={() => (isMouseOver = true)}
    on:mouseleave={() => (isMouseOver = false)}
    on:mousedown={handleMouseDown}
>
    <button class="titlebar-button" id="titlebar-minimize" on:click={minimize}>
        <img
            src="https://api.iconify.design/mdi:window-minimize.svg"
            alt="minimize"
        />
    </button>
    <button class="titlebar-button" id="titlebar-close" on:click={close}>
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </button>
</div>

<style>
    .titlebar {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 10px;
    overflow: hidden;
    transition: height 0.3s ease-in-out;
    background: black;
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.25rem;
    border-bottom-right-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .titlebar.expanded {
    height: 2rem;
  }

  .titlebar-button {
    background: black;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    outline: none;
    opacity: 0;
    transition: opacity 0.3s ease-in-out;
  }

  .titlebar.expanded .titlebar-button {
    opacity: 1;
  }

  .titlebar-button img {
    width: 1.5rem;
    height: 1.5rem;
    filter: invert(1);
  }
</style>
