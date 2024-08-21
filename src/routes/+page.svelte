<script lang="ts">
  import { openUrl } from "$lib/generated/specta/bindings";
  // import type { PageData } from './$types';
  // export let data: PageData;
  import { Template } from "$lib/imports";
  // let stroke: number = 2;
  let value: string;
  const handleOpen = async () => {
    await openUrl("aaaaaaaaaaaaaaaaa", value);
  };
</script>

<!--  -->
<Template>
  <div class="container">
    <input type="text" bind:value />
    <button type="button" on:click={handleOpen}>OPEN</button>
  </div>
</Template>

<style lang="scss">
  // :global(:root) {}
  .container {
    display: flex;
    flex-flow: row nowrap;
    align-content: center;
    justify-content: center;
    height: 1.6rem;
    & input {
      width: 12rem;
    }
  }
</style>
