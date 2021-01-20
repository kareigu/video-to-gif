<script lang="ts">
  import { NavigationDrawer, Overlay, Icon, List, ListItem, Button } from 'svelte-materialify';
  import { aboutOpen } from '../stores/utilStore';

  $: active = $aboutOpen;

  function closeDrawer() {
    aboutOpen.set(false);
  }

</script>

<style lang="scss">
  .content {
    margin: 10px 8px;
  }
</style>

<NavigationDrawer absolute right {active}>
  <List>
    <ListItem disabled>
      <h3 class="title">About</h3>
    </ListItem>
    
    <div class="content">
      <p>
        This site was created using Svelte with TypeScript and Snowpack
        and it utilises the 
        <a href="https://github.com/ffmpegwasm/ffmpeg.wasm" target="_blank">
          WebAssembly port of ffmpeg 
        </a>
        for conversion
      </p>
      <p>
        Because of this the conversion can be done fully on the client 
        with the server only needing to serve files
      </p>
      <p>
        Unfortunately with WebAssembly comes the caveat of not functioning on mobile browsers
      </p>
      <p>
        <b>At least for the time being</b>
      </p>
    </div>
  </List>
  <span slot="append" class="pa-2">
    <div class="links">
      <a href="https://github.com/mxrr/video-to-gif">
        <Icon class="mdi mdi-github" />
      </a>
    </div>
    <br/>
    <Button 
      icon
      on:click={closeDrawer}
    >
      <Icon class="mdi mdi-close-circle-outline red-text" />
    </Button>
  </span>
</NavigationDrawer>
<Overlay index={1} {active} on:click={closeDrawer} absolute />
