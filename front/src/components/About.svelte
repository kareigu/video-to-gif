<script lang="ts">
  import { NavigationDrawer, Overlay } from 'svelte-materialify';
  import { aboutOpen } from '../stores/utilStore';

  $: active = $aboutOpen;

</script>

<style lang="scss">

</style>

<NavigationDrawer absolute {active}>
  <p>Yes</p>
</NavigationDrawer>
<Overlay index={1} {active} on:click={() => aboutOpen.set(false)} absolute />
