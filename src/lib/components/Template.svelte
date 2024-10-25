<script lang="ts">
  import { createBubbler, stopPropagation } from "svelte/legacy";

  const bubble = createBubbler();
  interface Props {
    test?: string;
    children?: import("svelte").Snippet;
  }

  let { test = "1536px", children }: Props = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="template"
  role="button"
  tabindex="0"
  style:--test={test}
  onclick={stopPropagation(bubble("click"))}>
  {#if children}{@render children()}{:else}<div>no content</div>{/if}
</div>

<style lang="scss">
  .template {
    padding: 0 2rem;
    :global(*) {
      scroll-behavior: smooth;
      scrollbar-gutter: auto;
      scrollbar-width: thin;
    }
  }
</style>
