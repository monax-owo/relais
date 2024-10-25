<script lang="ts">
  import IconX from "@tabler/icons-svelte/icons/x";
  import IconMinus from "@tabler/icons-svelte/icons/minus";

  import IconPin from "@tabler/icons-svelte/icons/pin";
  import IconPinnedOff from "@tabler/icons-svelte/icons/pinned-off";

  import IconGhost from "@tabler/icons-svelte/icons/ghost";
  import IconGhostOff from "@tabler/icons-svelte/icons/ghost-off";

  import IconPointer from "@tabler/icons-svelte/icons/pointer";
  import IconPointerOff from "@tabler/icons-svelte/icons/pointer-off";

  import IconDeviceMobile from "@tabler/icons-svelte/icons/device-mobile";
  import IconDeviceDesktop from "@tabler/icons-svelte/icons/device-desktop";

  import IconZoomIn from "@tabler/icons-svelte/icons/zoom-in";
  import IconZoomOut from "@tabler/icons-svelte/icons/zoom-out";

  import IconArrowsMove from "@tabler/icons-svelte/icons/arrows-move";

  import { unwrap } from "$lib/util/wrap";
  import { commands } from "$lib/generated/specta/bindings";

  const stroke = 2;

  let pin = $state(false);
  let transparent: [boolean, number] = $state([false, 255]);
  let pointerIgnore = $state(false);
  let mobileMode = $state(false);

  onMount(async () => {
    [transparent, pin, pointerIgnore, mobileMode] = unwrap(await commands.getStatus());
  });

  const handleClose = async () => {
    unwrap(await commands.viewClose());
  };
  const handleMinimize = async () => {
    unwrap(await commands.viewMinimize());
  };
  const handlePin = async () => {
    pin = unwrap(await commands.togglePin());
  };
  const handleTransparent = async () => {
    transparent[0] = unwrap(await commands.toggleTransparent(127));
  };
  const handlePointerIgnore = async () => {
    pointerIgnore = unwrap(await commands.toggleIgnoreCursorEvents());
  };
  const handleMobileMode = async () => {
    mobileMode = unwrap(await commands.toggleUserAgent());
  };
  const handleZoomIn = async () => {
    unwrap(await commands.viewZoom(10));
  };
  const handleZoomOut = async () => {
    unwrap(await commands.viewZoom(-10));
  };
  const handleDrag = async () => {
    unwrap(await commands.viewDrag());
  };
</script>

<!-- TODO: lock/unlock animation -->
<!-- TODO: opacity level slider -->
<!-- <button type="button" on:click={}></button> -->
<div class="header">
  <button type="button" onclick={handleClose}><IconX {stroke}></IconX></button>
  <button type="button" onclick={handleMinimize}><IconMinus {stroke}></IconMinus></button>
  <button type="button" onclick={handlePin}>
    {#if pin}
      <IconPinnedOff {stroke} />
    {:else}
      <IconPin {stroke} />
    {/if}
  </button>
  <button type="button" onclick={handleTransparent}>
    {#if transparent[0]}
      <IconGhostOff {stroke} />
    {:else}
      <IconGhost {stroke} />
    {/if}
  </button>
  <button type="button" onclick={handlePointerIgnore}>
    {#if pointerIgnore}
      <IconPointerOff {stroke} />
    {:else}
      <IconPointer {stroke} />
    {/if}
  </button>
  <button type="button" onclick={handleMobileMode}>
    {#if mobileMode}
      <IconDeviceDesktop {stroke} />
    {:else}
      <IconDeviceMobile {stroke} />
    {/if}
  </button>
  <button type="button" onclick={handleZoomIn}><IconZoomIn {stroke} /></button>
  <button type="button" onclick={handleZoomOut}><IconZoomOut {stroke} /></button>

  <button type="button" class="drag" onpointerdown={handleDrag}>
    <IconArrowsMove {stroke} />
  </button>
</div>

<style lang="scss">
  @use "@monax-owo/style/global" as *;

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
