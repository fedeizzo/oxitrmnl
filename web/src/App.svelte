<script lang="ts">
    import Accordion from "./lib/Accordion.svelte";
    import { Button } from "bits-ui";
</script>

<main>
    <div class="card">
        <Accordion />
    </div>
    <div class="m-1">Hello</div>
    <Button.Root class="h-12 w-full bg-blue-500 hover:bg-blue-600"
        >Click me</Button.Root
    >
</main>
