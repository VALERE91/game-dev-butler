<script>
    import "../app.postcss";
    let { children } = $props();

    import ThemeSwitch from "$lib/ThemeSwitch.svelte";

    import { afterNavigate } from "$app/navigation";
    afterNavigate(() => {
        // @ts-ignore
        window.HSStaticMethods.autoInit();
    });
</script>

<ThemeSwitch />
{@render children()}
