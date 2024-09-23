<script lang="ts">
  import { commands, events, type SWindowData } from "$lib/generated/specta/bindings";
  import { Template } from "$lib/imports";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  // let stroke: number = 2;
  let url: string;
  let windows: SWindowData[] = [];
  let valid = true;

  const handleOpen = async () => {
    // try {
    //   new URL((url.startsWith("http") ? "" : "https://") + url);
    //   valid = true;
    // } catch (e) {
    //   valid = false;
    //   console.error(e);
    // }
    await commands.viewCreate(url, null);
  };

  onMount(async () => {
    const f = async () => (windows = await commands.getWindows());
    await f();

    await events.updateWindows(getCurrentWebviewWindow()).listen(async () => await f());
    ifDev(() => {
      ifThen(windows.length < 1, () => {
        console.log(windows.length);

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
        <div class="list">
          <!-- <span>{window.label}</span> -->
          <span>{window.title}</span>
          <span>{window.zoom}</span>
          <a href="/config?label={window.label}" class="btn">config</a>
        </div>
      </li>
    {/each}
  </ul>
  <div class="container"></div>
</Template>

<style lang="scss">
  @import "$lib/style/global.scss";

  // :global(:root) {}
  .container {
    display: flex;
    flex-flow: row nowrap;
    align-content: center;
    justify-content: space-between;
    gap: 0.2rem;
    height: 1.6rem;
    & input {
      margin: 0 0.2rem;
      width: 12rem;
    }
  }
  .list {
    display: flex;
    flex-flow: row wrap;
    align-content: center;
    justify-content: center;
  }
  button {
    margin: 0 0.2rem;
    border: 1px solid var(--b-bg);
    border-radius: var(--b-radius);
    padding: 0 0.4rem;
    font-size: 0.8rem;
  }
</style>
