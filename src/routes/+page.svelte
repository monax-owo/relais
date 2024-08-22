<script lang="ts">
  import { enhance } from "$app/forms";
  import { getWindows, openWindow, type WindowData } from "$lib/generated/specta/bindings";
  // import type { PageData } from './$types';
  // export let data: PageData;
  import { Template } from "$lib/imports";
  // let stroke: number = 2;
  let url: string;
  let windows: WindowData[] = [];
  const handleOpen = async () => {
    await openWindow("aaaaaaaaaaaaaaaaa", url);
    windows = await getWindows();
  };
</script>

<!--  -->
<Template>
  <form class="container" on:submit={handleOpen} use:enhance={() => {}}>
    <input type="text" bind:value={url} />
    <button type="submit">OPEN</button>
  </form>
  <ul>
    {#each windows as window}
      <li>
        <div>
          <span>{window.label}</span>
          <span>{window.title}</span>
          <span>{window.zoom}</span>
        </div>
      </li>
    {/each}
  </ul>
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
