<script lang="ts">
  import { openWindow, type WindowData } from "$lib/generated/specta/bindings";
  // import type { PageData } from './$types';
  // export let data: PageData;
  import { Template } from "$lib/imports";
  import { listen } from "@tauri-apps/api/event";
  // let stroke: number = 2;
  let url: string;
  let windows: WindowData[] = [];
  // todo label
  const handleOpen = async () => {
    let label = "";
    try {
      let link = new URL((url.startsWith("http") ? "" : "https://") + url);
      console.log(link);
      label = (link.hostname + link.pathname).replaceAll(/[./]/g, "_");
    } catch (e) {
      console.error(e);
    }
    console.log(label);
    await openWindow(label, url, null);
  };
  onMount(async () => {
    listen("update_windows", (e) => {
      windows = e.payload as WindowData[];
    });
  });
</script>

<!--  -->
<Template>
  <form class="container" on:submit={handleOpen}>
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
