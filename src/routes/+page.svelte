<script lang="ts">
  import { commands, type SerDeWindowData } from "$lib/generated/specta/bindings";
  import { Template } from "$lib/imports";
  import { state } from "$lib/stores/state";

  import { superForm } from "sveltekit-superforms";
  export let data;
  const { form } = superForm(data.form);

  // let stroke: number = 2;
  let url: string;
  let windows: SerDeWindowData[] | undefined = undefined;
  let _valid = true;

  $: windows = $state?.windows;

  const handleOpen = async () => {
    // try {
    //   new URL((url.startsWith("http") ? "" : "https://") + url);
    //   valid = true;
    // } catch (e) {
    //   valid = false;
    //   console.error(e);
    // }
    await commands.viewCreate(url);
  };
</script>

<Template>
  <div class="root">
    <form class="form" on:submit={handleOpen}>
      <!-- TODO:superformsでvalidationする -->
      <!-- <span>{valid}</span> -->
      <input type="text" bind:value={url} />
      <button type="submit">OPEN</button>
    </form>
    <ul class="windows">
      {#if windows}
        {#each windows as window}
          <li class="hover-11">
            <div>{window.title}</div>
            <div>{window.url}</div>
            <a href="/config?label={window.label}" class="btn">config</a>
          </li>
        {/each}
      {:else}
        <li>window not found...</li>
      {/if}
    </ul>
  </div>
</Template>

<style lang="scss">
  @use "@monax-owo/style/global";
  .root {
    display: flex;
    flex-flow: column nowrap;
    padding: 2rem 0;
    height: auto;
  }
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
    padding: 0;
    height: 200px;
    overflow-y: scroll;
    li {
      display: flex;
      flex-flow: row nowrap;
      justify-content: space-between;
      height: 2rem;
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
