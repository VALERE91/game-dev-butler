<script lang="ts">
    import type { LayoutData, PageData } from "./$types";

    let { data }: { data: PageData } = $props();
</script>

<div class="container h-full mx-auto flex justify-center items-center">
    <h1 class="text-3xl underline">{data.post}</h1>
</div>
