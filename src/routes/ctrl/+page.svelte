<script lang="ts">
  import { Template } from "$lib/imports";
</script>

<Template>
  <div class="header" data-tauri-drag-region>
    <button type="button">_</button>
    <button type="button">O</button>
    <button type="button">X</button>
    <button type="button" data-tauri-drag-region>@</button>
  </div>
</Template>

<style lang="scss">
  @use "$lib/style/global.scss" as *;
  *:has(> .test) {
    background-color: yellow;
  }
  .header {
    display: flex;
    gap: 0.2rem;
    background-color: red;
    width: 100%;
    height: 100%;
    @include orientation(portrait) {
      flex-flow: column nowrap;
    }
    & > button {
      width: 1.2rem;
      height: 1.2rem;
      font-size: 0.8rem;
    }
  }
</style>
