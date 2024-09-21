<script lang="ts">
  import { page } from "$app/stores";
  import { Template } from "$lib/imports";
  let label = $page.url.searchParams.get("label") ?? "null";
  console.log(label);
</script>

<Template>
  <nav><a href="/" class="btn">{"←"}</a></nav>
</Template>

<style lang="scss">
</style>
