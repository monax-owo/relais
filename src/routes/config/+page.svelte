<script lang="ts">
  import { page } from "$app/stores";
  import type { SWindowData } from "$lib/generated/specta/bindings";
  import { Template } from "$lib/imports";
  import { state } from "$lib/stores/state";
  let label = $page.url.searchParams.get("label") ?? "null";
  let window: SWindowData | undefined;
  onMount(() => {
    window = $state?.windows.find((v) => v.label == label);
  });
</script>

<Template>
  <nav><a href="/" class="btn">{"←"}</a></nav>
  {#if window}
    <div class="list">
      <span>{window.title}</span>
      <span>{window.label}</span>
      <span>{window.url}</span>
      <a href="/config?label={window.label}" class="btn">config</a>
    </div>
  {:else}
    <div class="container">ERR</div>
  {/if}
</Template>

<style lang="scss">
</style>
