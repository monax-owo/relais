<script lang="ts">
  import { commands, type SWindowData } from "$lib/generated/specta/bindings";
  import { Template } from "$lib/imports";
  import { state } from "$lib/stores/state";

  // let stroke: number = 2;
  let url: string;
  let windows: SWindowData[] | undefined;
  let valid = true;

  $: windows = $state?.windows;

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
    ifDev(() => {
      let un = state.subscribe((v) => {
        if (v && v.windows.length < 1) {
          console.log(v.windows.length);
          url = "google.com";
          handleOpen();
          un();
        }
      });
    });
  });
</script>

<Template>
  <form class="form" on:submit={handleOpen}>
    <span>{valid}</span>
    <input type="text" bind:value={url} />
    <button type="submit">OPEN</button>
  </form>
  <ul class="windows">
    {#each windows ?? [] as window}
      <li class="hover-1">
        <div>{window.title}</div>
        <div>{window.url}</div>
        <a href="/config?label={window.label}" class="btn">config</a>
      </li>
    {/each}
  </ul>
</Template>

<style lang="scss">
  @import "@monax-owo/style/global";
  .form {
    display: flex;
    flex-flow: row nowrap;
    align-content: center;
    justify-content: space-between;
    gap: 0.2rem;
    height: 1.6rem;
    & input {
      display: inline-block;
      flex-grow: 1;
      margin: 0 0.2rem;
    }
  }
  .windows {
    li {
      display: flex;
      flex-flow: row nowrap;
      justify-content: space-between;
      list-style-type: none;
      & div {
        font-size: 0.6rem;
      }
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
