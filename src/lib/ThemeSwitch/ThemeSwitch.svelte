<script lang="ts">
    import { browser } from "$app/environment";

    let darkMode = true;

    function handleSwitchDarkMode() {
        darkMode = !darkMode;
        localStorage.setItem("theme", darkMode ? "dark" : "light");
        if (darkMode) {
            document.documentElement.classList.remove("light");
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
            document.documentElement.classList.add("light");
        }
    }

    if (browser) {
        if (
            localStorage.theme === "dark" ||
            (!("theme" in localStorage) &&
                window.matchMedia("(prefers-color-scheme: dark)").matches)
        ) {
            document.documentElement.classList.add("dark");
            document.documentElement.classList.remove("light");
            darkMode = true;
        } else {
            document.documentElement.classList.remove("dark");
            document.documentElement.classList.add("light");
            darkMode = false;
        }
    }
</script>

<div>
    <button
        type="button"
        class="hs-dark-mode-active:hidden block hs-dark-mode font-medium text-gray-800 rounded-full hover:bg-gray-200 focus:outline-none focus:bg-gray-200 dark:text-neutral-200 dark:hover:bg-neutral-800 dark:focus:bg-neutral-800"
        data-hs-theme-click-value="dark"
        aria-label="Toggle dark mode"
        onclick={handleSwitchDarkMode}
    >
        <span
            class="group inline-flex shrink-0 justify-center items-center size-9"
        >
            <svg
                class="shrink-0 size-4"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
            </svg>
        </span>
    </button>

    <button
        type="button"
        class="hs-dark-mode-active:block hidden hs-dark-mode font-medium text-gray-800 rounded-full hover:bg-gray-200 focus:outline-none focus:bg-gray-200 dark:text-neutral-200 dark:hover:bg-neutral-800 dark:focus:bg-neutral-800"
        data-hs-theme-click-value="light"
        aria-label="Toggle light mode"
        onclick={handleSwitchDarkMode}
    >
        <span
            class="group inline-flex shrink-0 justify-center items-center size-9"
        >
            <svg
                class="shrink-0 size-4"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="12" cy="12" r="4"></circle>
                <path d="M12 2v2"></path>
                <path d="M12 20v2"></path>
                <path d="m4.93 4.93 1.41 1.41"></path>
                <path d="m17.66 17.66 1.41 1.41"></path>
                <path d="M2 12h2"></path>
                <path d="M20 12h2"></path>
                <path d="m6.34 17.66-1.41 1.41"></path>
                <path d="m19.07 4.93-1.41 1.41"></path>
            </svg>
        </span>
    </button>
</div>

<style lang="postcss">
    :global(body) {
        transition:
            background-color 0.2s ease-in-out,
            color 0.2s ease-in-out;
        @apply bg-white dark:bg-black text-black dark:text-white text-center;
    }
</style>
