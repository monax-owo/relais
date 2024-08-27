<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";

  import IconMinus from "@tabler/icons-svelte/IconMinus.svelte";
  import IconX from "@tabler/icons-svelte/IconX.svelte";

  import IconPin from "@tabler/icons-svelte/IconPin.svelte";
  import IconPinnedOff from "@tabler/icons-svelte/IconPinnedOff.svelte";

  import IconZoomIn from "@tabler/icons-svelte/IconZoomIn.svelte";
  import IconZoomOut from "@tabler/icons-svelte/IconZoomOut.svelte";

  import IconArrowsMove from "@tabler/icons-svelte/IconArrowsMove.svelte";
  // import IconLockOpen from "@tabler/icons-svelte/IconLockOpen.svelte";
  import IconLock from "@tabler/icons-svelte/IconLock.svelte";
  import IconLockOpen2 from "@tabler/icons-svelte/IconLockOpen2.svelte";

  import { getTransparent } from "$lib/generated/specta/bindings";

  const stroke = 2;

  let transparent = false;
  let pinned = false;

  const em = async (event: unknown) => {
    await appWindow.emit("ctrl", event);
  };

  const handleMini = async () => {
    await em("mini");
  };
  const handleClose = async () => {
    await em("close");
  };
  const handlePin = async () => {
    // TODO
  };

  const handleZoomIn = async () => {
    await em("zoomin");
  };
  const handleZoomOut = async () => {
    await em("zoomout");
  };
  const handleTransparent = async () => {
    await em("transparent");
    transparent = await getTransparent();
  };
</script>

<!-- TODO: lock/unlock animation -->
<!-- <button type="button" on:pointerdown={}></button> -->
<div class="header">
  <button type="button" on:pointerdown={handleMini}><IconMinus {stroke}></IconMinus></button>
  <button type="button" on:pointerdown={handleClose}><IconX {stroke}></IconX></button>
  <button type="button" on:pointerdown={handlePin}>
    {#if pinned}
      <IconPinnedOff {stroke} />
    {:else}
      <IconPin {stroke} />
    {/if}
  </button>
  <button type="button" on:pointerdown={handleTransparent}>
    {#if transparent}
      <IconLockOpen2 {stroke} />
      <!-- <IconLockOpen {stroke} /> -->
    {:else}
      <IconLock {stroke} />
    {/if}
  </button>
  <button type="button" on:pointerdown={handleZoomIn}><IconZoomIn {stroke} /></button>
  <button type="button" on:pointerdown={handleZoomOut}><IconZoomOut {stroke} /></button>

  <button type="button" data-tauri-drag-region>
    <IconArrowsMove {stroke} data-tauri-drag-region />
  </button>
</div>

<style lang="scss">
  @use "$lib/style/global.scss" as *;
  .header {
    display: flex;
    flex-wrap: wrap;
    align-content: center;
    justify-content: space-evenly;
    gap: 0.4rem;
    background-color: var(--bg);
    width: 100%;
    height: 100%;
    color: var(--text);
    @include orientation(portrait) {
      flex-direction: column;
    }
    @include orientation(landscape) {
      flex-direction: row;
    }
    & > button {
      width: 1.2rem;
      height: 1.2rem;
      font-size: 0.8rem;
      text-align: center;
      &[data-tauri-drag-region] {
        cursor: move;
        pointer-events: fill;
      }
    }
  }
</style>
