<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";

  const em = async (event: unknown) => {
    await appWindow.emit("ctrl", event);
  };
  const handleMini = async () => {
    await em("mini");
  };
  const handleMaxi = async () => {
    await em("maxi");
  };
  const handleClose = async () => {
    await em("close");
  };
</script>

<div class="header" data-tauri-drag-region>
  <button type="button" on:pointerdown={handleMini}>_</button>
  <button type="button" on:pointerdown={handleMaxi}>O</button>
  <button type="button" on:pointerdown={handleClose}>X</button>
  <button type="button" data-tauri-drag-region>@</button>
</div>

<style lang="scss">
  @use "$lib/style/global.scss" as *;
  .header {
    display: flex;
    flex-wrap: wrap;
    align-content: center;
    justify-content: center;
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
    }
  }
</style>
