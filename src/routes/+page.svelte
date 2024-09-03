<script lang="ts">
  import { commands, type WindowData } from "$lib/generated/specta/bindings";
  import { Template } from "$lib/imports";
  import { listen } from "@tauri-apps/api/event";

  // let stroke: number = 2;
  let url: string;
  let windows: WindowData[] = [];
  let valid = true;

  const handleOpen = async () => {
    try {
      new URL((url.startsWith("http") ? "" : "https://") + url);
      valid = true;
    } catch (e) {
      valid = false;
      console.error(e);
    }
    await commands.viewCreate(url, null, null);
  };

  onMount(async () => {
    const f = async () => (windows = await commands.getWindows());
    await f();

    listen("update_windows", async () => await f());

    ifDev(() => {
      console.log(windows.length);

      ifThen(windows.length < 1, () => {
        url = "www.twitch.tv/stylishnoob4";
        handleOpen();
      });
    });
  });
</script>

<!--  -->
<Template>
  <form class="container" on:submit={handleOpen}>
    <span>{valid}</span>
    <input type="text" bind:value={url} />
    <button type="submit">OPEN</button>
  </form>
  <ul>
    {#each windows as window}
      <li>
        <div>
          <!-- <span>{window.label}</span> -->
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
    gap: 0.2rem;
    height: 1.6rem;
    & input {
      margin: 0 0.2rem;
      width: 12rem;
    }
  }
  button {
    margin: 0 0.2rem;
    border: 1px solid var(--b-bg);
    border-radius: var(--b-radius);
    padding: 0 0.4rem;
    font-size: 0.8rem;
  }
</style>
