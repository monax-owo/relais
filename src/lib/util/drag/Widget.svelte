<script lang="ts">
  import { stopPropagation } from "svelte/legacy";

  import { draggable, type HandleElements } from "$lib/util/drag/drag";
  import IconMinus from "@tabler/icons-svelte/icons/minus";
  // import IconMaximize from "@tabler/icons-svelte/icons/minimize";
  import IconRectangle from "@tabler/icons-svelte/icons/rectangle";
  import IconX from "@tabler/icons-svelte/icons/x";

  interface Props {
    borderSize?: number;
    handleSize?: number;
    initPos?: [number, number];
    padding?: number;
    resizable?: boolean;
    size?: [number, number];
    title?: string;
    children?: import("svelte").Snippet;
  }

  let {
    borderSize = 1,
    handleSize = 12,
    initPos = [12, 12],
    padding = 0,
    resizable = true,
    size = [400, 280],
    title = "no title",
    children,
  }: Props = $props();

  const stroke = 2;

  let [left, top] = initPos;
  // let [width, height] = size;
  let target: HTMLElement | undefined = $state();
  const handles: HandleElements = $state({});

  onMount(() => {
    if (target) {
      target.style.left = left + "px";
      target.style.top = top + "px";
    }
  });
</script>

<!-- todo: dragのカーソルをどっちにするか決める -->
<!-- todo: サイズ変更等 -->
<div class="root">
  <div
    class="widget"
    role="button"
    tabindex="0"
    bind:this={target}
    style:--b={borderSize + "px"}
    style:--h-size={handleSize + "px"}>
    <div class="body">
      <div class="header" use:draggable={{ handles, padding, size, target }}>
        <div class="header-title">{title}</div>
        <div class="container" onpointerdown={stopPropagation(() => {})}>
          <button type="button"><IconMinus {stroke} /></button>
          <!-- <button type="button"><IconMaximize {stroke} /></button> -->
          <button type="button"><IconRectangle {stroke} /></button>
          <button type="button"><IconX {stroke} /></button>
        </div>
      </div>
      <div class="border"></div>
      <div class="content">
        {@render children?.()}
      </div>
    </div>
    {#if resizable}
      <div class="y t" bind:this={handles.top}></div>
      <div class="x r" bind:this={handles.right}></div>
      <div class="y b" bind:this={handles.bottom}></div>
      <div class="x l" bind:this={handles.left}></div>

      <div class="t r nesw" bind:this={handles.top_right}></div>
      <div class="r b nwse" bind:this={handles.right_bottom}></div>
      <div class="b l nesw" bind:this={handles.bottom_left}></div>
      <div class="l t nwse" bind:this={handles.left_top}></div>
    {/if}
  </div>
</div>

<style lang="scss">
  .root {
    position: fixed;
    top: 0;
    left: 0;
  }

  .widget {
    position: relative;
    box-sizing: content-box;
    --b: 0;
    --h-size: 0;
    --hs: calc(var(--h-size) + var(--b));
    --nhs: calc(calc(-1 * var(--hs)) + 4px);
    --pd: calc(var(--hs) + var(--nhs));

    & .body {
      display: flex;
      position: absolute;
      flex-flow: column nowrap;
      box-sizing: content-box;
      border: var(--b) solid var(--b-bg);
      border-radius: var(--b-radius);
      background-color: var(--bg);
      width: 100%;
      height: 100%;
      overflow: clip;
      color: var(--text);
      & .header {
        display: flex;
        flex-flow: row nowrap;
        justify-content: space-between;
        cursor: grab;
        padding: 0.2rem 0.4rem;
        & .container {
          display: flex;
          flex-flow: row nowrap;
          justify-content: center;
          gap: 0.4rem;
          & button {
            display: flex;
            flex-flow: row nowrap;
            align-items: center;
          }
        }
      }

      & .border {
        background-color: var(--b-bg);
        height: var(--b);
      }

      & .content {
        flex: 1;
        background-color: #fff !important;
        padding: 0 var(--pd) var(--pd);
        height: auto;
      }
    }
    & > :is(.t, .r, .b, .l) {
      position: absolute;
    }

    // & > :is(.t, .b):is(.r, .l) {
    //
    // }

    & .y {
      left: 0;
      cursor: ns-resize;
      width: 100%;
    }

    & .x {
      top: 0;
      cursor: ew-resize;
      height: 100%;
    }

    & .t {
      top: var(--nhs);
      height: var(--hs);
    }

    & .r {
      right: var(--nhs);
      width: var(--hs);
    }

    & .b {
      bottom: var(--nhs);
      height: var(--hs);
    }

    & .l {
      left: var(--nhs);
      width: var(--hs);
    }

    & .nesw {
      cursor: nesw-resize;
    }

    & .nwse {
      cursor: nwse-resize;
    }
  }
</style>
