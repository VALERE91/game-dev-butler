<script>
    let { children } = $props();

    import ThemeSwitch from "$lib/ThemeSwitch/ThemeSwitch.svelte";

    import { afterNavigate } from "$app/navigation";
    afterNavigate(() => {
        // @ts-ignore
        window.HSStaticMethods.autoInit();
    });
</script>

<ThemeSwitch />
{@render children()}
