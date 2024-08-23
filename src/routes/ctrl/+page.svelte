<script lang="ts">
  import { Template } from "$lib/imports";
</script>

<Template>
  <div class="test">test</div>
</Template>

<style lang="scss">
  *:has(> .test) {
    background-color: yellow;
  }
</style>
