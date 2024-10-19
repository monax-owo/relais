<script lang="ts">
  import { page } from "$app/stores";
  import type { SerDeWindowData } from "$lib/generated/specta/bindings";
  import { Template } from "$lib/imports";
  import { state } from "$lib/stores/state";
  let label = $page.url.searchParams.get("label") ?? "null";
  let window: SerDeWindowData | undefined;
  let datalist: Map<string, unknown>;
  onMount(() => {
    console.log(window);

    let un = state.subscribe((v) => {
      if (v) {
        window = v.windows.find((v) => v.label == label);
        if (window) {
          datalist = new Map<string, unknown>([
            ["title", window.title],
            ["label", window.label],
            ["url", window.url],
            ["pointer_ignore", window.pointer_ignore],
            ["mobile_mode", window.mobile_mode],
            ["transparent", window.transparent],
            ["pin", window.pin],
            ["zoom", window.zoom],
          ]);
        }
        un();
      }
    });
  });
</script>

<Template>
  <nav><a href="/" class="btn">{"←"}</a></nav>
  {#if window}
    <div class="list">
      {#each datalist as data}
        <div><span class="key">{data[0]}: </span><span class="value">{data[1]}</span></div>
      {/each}
    </div>
  {/if}
</Template>

<style lang="scss">
  .list {
    display: flex;
    flex-flow: column nowrap;
    & div {
      font-size: 0.8rem;
      & .key {
        opacity: 0.6;
      }
    }
  }
</style>
