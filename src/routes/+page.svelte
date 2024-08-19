<script lang="ts">
  // import type { PageData } from './$types';
  // export let data: PageData;
  import { Template } from "$lib/imports";
  import type { SubmitFunction } from "@sveltejs/kit";
  import { listen, TauriEvent, type UnlistenFn } from "@tauri-apps/api/event";
  import { dev } from "$app/environment";
  import { base } from "$app/paths";

  let stroke: number = 2;
</script>

<!--  -->
<Template>
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
