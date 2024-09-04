<script lang="ts">
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  import IconMinus from "@tabler/icons-svelte/IconMinus.svelte";
  import IconX from "@tabler/icons-svelte/IconX.svelte";

  import IconPin from "@tabler/icons-svelte/IconPin.svelte";
  import IconPinnedOff from "@tabler/icons-svelte/IconPinnedOff.svelte";

  import IconGhost from "@tabler/icons-svelte/IconGhost.svelte";
  import IconGhostOff from "@tabler/icons-svelte/IconGhostOff.svelte";

  import IconZoomIn from "@tabler/icons-svelte/IconZoomIn.svelte";
  import IconZoomOut from "@tabler/icons-svelte/IconZoomOut.svelte";

  import IconPointer from "@tabler/icons-svelte/IconPointer.svelte";
  import IconPointerOff from "@tabler/icons-svelte/IconPointerOff.svelte";

  import IconArrowsMove from "@tabler/icons-svelte/IconArrowsMove.svelte";

  import { commands, CTRL_LABEL_PREFIX } from "$lib/generated/specta/bindings";
  // import { window } from "@tauri-apps/api";

  const stroke = 2;

  const ctrl = getCurrentWebviewWindow();
  // TODO
  // const win = window.getAllWindows().find((v) => v.label === CTRL_LABEL_PREFIX + ctrl.label);
  // if (!window) throw new Error("window is not found");
  let transparent = false;
  let pinned = false;
  let pointerIgnored = false;

  const handleMini = async () => {
    await commands.viewMinimize();
  };
  const handleClose = async () => {
    await commands.viewClose(ctrl.label.replace(CTRL_LABEL_PREFIX, ""));
  };
  const handlePin = async () => {
    pinned = await commands.togglePin().then((v) => {
      switch (v.status) {
        case "ok":
          return v.data;
        case "error":
          throw err(v.error);
      }
    });
  };
  const handleTransparent = async () => {
    transparent = await commands.toggleTransparent(127).then((v) => {
      switch (v.status) {
        case "ok":
          return v.data;
        case "error":
          throw err(v.error);
      }
    });
    // transparent = await getTransparent();
  };
  const handlePointerIgnore = async () => {
    pointerIgnored = await commands.toggleIgnoreCursorEvents().then((v) => {
      switch (v.status) {
        case "ok":
          return v.data;
        case "error":
          throw err(v.error);
      }
    });
  };
  const handleZoomIn = async () => {
    await commands.viewZoomin().then((v) => {
      switch (v.status) {
        case "ok":
          return v.data;
        case "error":
          throw err(v.error);
      }
    });
  };
  const handleZoomOut = async () => {
    await commands.viewZoomout().then((v) => {
      switch (v.status) {
        case "ok":
          return v.data;
        case "error":
          throw err(v.error);
      }
    });
  };
  const handleDrag = async () => {
    // TODO
    // window.startDragging();
    await commands.viewDrag();
  };
  const err = (err: string) => {
    ctrl.emitTo(ctrl.label.replace(CTRL_LABEL_PREFIX, ""), "err", { err });
  };
</script>

<!-- TODO: lock/unlock animation -->
<!-- TODO: opacity level slider -->
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
      <IconGhostOff {stroke} />
      <!-- <IconLockOpen {stroke} /> -->
    {:else}
      <IconGhost {stroke} />
    {/if}
  </button>
  <button type="button" on:pointerdown={handlePointerIgnore}>
    {#if pointerIgnored}
      <IconPointerOff {stroke} />
      <!-- <IconLockOpen {stroke} /> -->
    {:else}
      <IconPointer {stroke} />
    {/if}
  </button>
  <button type="button" on:pointerdown={handleZoomIn}><IconZoomIn {stroke} /></button>
  <button type="button" on:pointerdown={handleZoomOut}><IconZoomOut {stroke} /></button>

  <button type="button" class="drag" on:pointerdown={handleDrag}>
    <IconArrowsMove {stroke} />
  </button>
</div>

<style lang="scss">
  @use "$lib/style/global.scss" as *;
  .header {
    display: flex;
    flex-wrap: wrap;
    align-content: center;
    justify-content: space-evenly;
    gap: 0.2rem;
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
      padding: 0.2rem;
      width: 1.2rem;
      height: 1.2rem;
      font-size: 0.8rem;
      text-align: center;
    }
    .drag {
      cursor: move;
    }
  }
</style>
