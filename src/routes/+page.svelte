<script lang="ts">
  // import type { PageData } from './$types';
  // export let data: PageData;
  import { Template } from "$lib/imports";
  // import { listen, TauriEvent, type UnlistenFn } from "@tauri-apps/api/event";
  import { dev } from "$app/environment";
  import { base } from "$app/paths";
  import Widget from "$lib/util/drag/Widget.svelte";
  import { authorize } from "$lib/generated/specta/bindings";
  // let stroke: number = 2;

  const handleAuth = async () => {
    await authorize();
  };
</script>

<!--  -->
<Template>
  <Widget>
    <button type="button" on:click={handleAuth}>button</button>
  </Widget>
  {#if dev}
    <div class="widget">
      <div class="one"><a href="{base}/dev/test">test</a></div>
    </div>
  {/if}
</Template>

<style lang="scss">
  // :global(:root) {}

  .widget {
    position: absolute;
    & > div {
      position: relative;
    }
    & .one {
      left: 10px;
    }
  }
</style>
