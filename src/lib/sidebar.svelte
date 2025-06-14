<script lang="ts">
    import { page } from "$app/state";

    let isSidebarCollapsed = false;

    function toggleSidebar() {
        isSidebarCollapsed = !isSidebarCollapsed;
    }

    function toggleTheme() {
        window.document.body.classList.toggle("dark-mode");
    }
</script>

<aside class="sidebar" class:collapsed={isSidebarCollapsed}>
    <button
        class="toggle-btn"
        style={`left: ${isSidebarCollapsed ? "50px" : "240px"};`}
        on:click={toggleSidebar}
        aria-label="Toggle sidebar"
    >
        <img
            class="arrow"
            class:collapsed={isSidebarCollapsed}
            src="https://api.iconify.design/mdi:chevron-left.svg"
            alt="toggle sidebar"
        />
    </button>
    <nav class="nav-list">
        <a
            class="nav-item {page.url.pathname === '/passwords'
                ? 'active'
                : ''}"
            href="/passwords"
        >
            <img class="icon" src="/icons/key.svg" alt="Passwords icon" />
            <span>Passwords</span>
        </a>
        <a
            class="nav-item {page.url.pathname === '/one-time-codes' ? 'active' : ''}"
            href="/one-time-codes"
        >
            <img
                class="icon"
                src="/icons/clock.svg"
                alt="One-Time Codes icon"
            />
            <span>One-Time Codes</span>
        </a>
        <a
            class="nav-item {page.url.pathname ===
            '/check-passwords'
                ? 'active'
                : ''}"
            href="/check-passwords"
        >
            <img
                class="icon"
                src="/icons/shield.svg"
                alt="Check Passwords icon"
            />
            <span>Check Passwords</span>
        </a>
    </nav>
    <button
        class="theme-btn"
        on:click={toggleTheme}
    >
        <img class="theme-icon" src="/icons/theme.svg" alt="Change theme" />
        <span>Change Theme</span>
    </button>
</aside>

<style>
    /* .sidebar {
        width: 240px;
        background: #191919;
        display: flex;
        flex-direction: column;
        padding: 0.75rem;
        transition: width 0.3s ease;
    }

    .sidebar.collapsed {
        width: 50px;
    } */

    .toggle-btn {
        position: absolute;
        left: 19.4rem;
        top: 50%;
        transform: translate(0%, -50%);
        transition: left 0.3s ease;
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background: #191919;
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        cursor: pointer;
        z-index: 10;
    }

    .arrow {
        width: 1.5rem;
        height: 1.5rem;
        filter: invert(1);
        transition: transform 0.3s ease;
    }

    .arrow.collapsed {
        transform: rotate(180deg);
    }

    .nav-list {
        padding-top: 1.6rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .nav-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        background: transparent;
        border: none;
        color: inherit;
        font-size: 1.05rem;
        font-weight: 600;
        padding: 1rem;
        border-radius: 6px;
        cursor: pointer;
        transition: background 0.2s;
        text-decoration: none;
    }

    .nav-item:hover {
        background: #3a3a3a;
    }

    .nav-item.active {
        background: #4a4a4a;
    }

    .icon {
        width: 1.25rem;
        height: 1.25rem;
        flex-shrink: 0;
        filter: brightness(0) invert(1);
    }

    /* .sidebar.collapsed .nav-item span,
    .sidebar.collapsed .theme-btn span {
        display: none;
    } */

    /* .sidebar.collapsed .nav-item {
        justify-content: center;
        padding: 1rem;
    } */

    /* .sidebar.collapsed .icon {
        margin: 0;
    } */

    .theme-btn {
        color: white;
        background: transparent;
        border: none;
        padding: 0.75rem;
        border-radius: 6px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: left;
        gap: 0.75rem;
        font-size: 1rem;
        font-weight: 600;
        transition: background 0.2s;
        margin-top: auto;
        text-decoration: none;
    }

    .theme-btn:hover {
        background: #3a3a3a;
    }

    .theme-icon {
        width: 1.5rem;
        height: 1.5rem;
        filter: brightness(0) invert(1);
    }
</style>
