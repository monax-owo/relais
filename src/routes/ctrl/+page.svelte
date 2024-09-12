<script lang="ts">
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  import IconMinus from "@tabler/icons-svelte/IconMinus.svelte";
  import IconX from "@tabler/icons-svelte/IconX.svelte";

  import IconPin from "@tabler/icons-svelte/IconPin.svelte";
  import IconPinnedOff from "@tabler/icons-svelte/IconPinnedOff.svelte";

  import IconGhost from "@tabler/icons-svelte/IconGhost.svelte";
  import IconGhostOff from "@tabler/icons-svelte/IconGhostOff.svelte";

  import IconPointer from "@tabler/icons-svelte/IconPointer.svelte";
  import IconPointerOff from "@tabler/icons-svelte/IconPointerOff.svelte";

  import IconDeviceMobile from "@tabler/icons-svelte/IconDeviceMobile.svelte";
  import IconDeviceDesktop from "@tabler/icons-svelte/IconDeviceDesktop.svelte";

  import IconZoomIn from "@tabler/icons-svelte/IconZoomIn.svelte";
  import IconZoomOut from "@tabler/icons-svelte/IconZoomOut.svelte";

  import IconArrowsMove from "@tabler/icons-svelte/IconArrowsMove.svelte";

  import { commands, CTRL_LABEL_PREFIX, type Result } from "$lib/generated/specta/bindings";
  // import { window } from "@tauri-apps/api";

  const stroke = 2;
  const ctrl = getCurrentWebviewWindow();
  let pin = false;
  let overlay = false;
  let pointerIgnore = false;
  let mobileMode = false;

  onMount(async () => {
    [overlay, pin, pointerIgnore, mobileMode] = unwrap(await commands.getStatus());
  });

  const err = (err: string) => {
    ctrl.emitTo(ctrl.label.replace(CTRL_LABEL_PREFIX, ""), "err", { err });
  };

  const unwrap = <T,>(v: Result<T, string>): T => {
    switch (v.status) {
      case "ok":
        return v.data;
      case "error":
        throw err(v.error);
    }
  };

  const handleMinimize = async () => {
    unwrap(await commands.viewMinimize());
  };
  const handleClose = async () => {
    unwrap(await commands.viewClose(ctrl.label.replace(CTRL_LABEL_PREFIX, "")));
  };
  const handlePin = async () => {
    pin = unwrap(await commands.togglePin());
  };
  const handleOverlay = async () => {
    overlay = unwrap(await commands.toggleTransparent(127));
  };
  const handlePointerIgnore = async () => {
    pointerIgnore = unwrap(await commands.toggleIgnoreCursorEvents());
  };
  const handleMobileMode = async () => {
    mobileMode = unwrap(await commands.toggleUserAgent());
  };
  const handleZoomIn = async () => {
    unwrap(await commands.viewZoomin());
  };
  const handleZoomOut = async () => {
    unwrap(await commands.viewZoomout());
  };
  const handleDrag = async () => {
    unwrap(await commands.viewDrag());
  };
</script>

<!-- TODO: lock/unlock animation -->
<!-- TODO: opacity level slider -->
<!-- <button type="button" on:pointerdown={}></button> -->
<div class="header">
  <button type="button" on:pointerdown={handleMinimize}><IconMinus {stroke}></IconMinus></button>
  <button type="button" on:pointerdown={handleClose}><IconX {stroke}></IconX></button>
  <button type="button" on:pointerdown={handlePin}>
    {#if pin}
      <IconPinnedOff {stroke} />
    {:else}
      <IconPin {stroke} />
    {/if}
  </button>
  <button type="button" on:pointerdown={handleOverlay}>
    {#if overlay}
      <IconGhostOff {stroke} />
    {:else}
      <IconGhost {stroke} />
    {/if}
  </button>
  <button type="button" on:pointerdown={handlePointerIgnore}>
    {#if pointerIgnore}
      <IconPointerOff {stroke} />
    {:else}
      <IconPointer {stroke} />
    {/if}
  </button>
  <button type="button" on:pointerdown={handleMobileMode}>
    {#if mobileMode}
      <IconDeviceDesktop {stroke} />
    {:else}
      <IconDeviceMobile {stroke} />
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
