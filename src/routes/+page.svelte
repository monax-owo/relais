<script lang="ts">
  // import type { PageData } from './$types';
  // export let data: PageData;
  import { Template } from "$lib/imports";
  // let stroke: number = 2;
  let value: string;
</script>

<!--  -->
<Template>
  <div class="container">
    <input type="text" bind:value />
    <button>OPEN</button>
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
  }
</style>
