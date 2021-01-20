<script lang='ts'>
	import { MaterialApp, Button, AppBar, Icon } from 'svelte-materialify';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import FFmpeg from './components/FFmpeg.svelte';
	import About from './components/About.svelte';
	import { clearVideo } from './utils/ffmpeg';
	import { toggleTheme } from './utils/theme';
	import { theme, aboutOpen } from './stores/utilStore';

	onMount(() => {
		document.body.setAttribute('class', `theme--${get(theme)}`);
		return () => {
			clearVideo();
		}
	});
</script>

<style lang="scss">
	@import './App.scss';
</style>
  
<MaterialApp theme={$theme}>
	<AppBar class="secondary-color theme--dark">
		<span slot="title">
			<Icon class="mdi mdi-camera-image" />
			Convert video to GIF
		</span>
		<div style="flex-grow:1" />
		<div style="margin-right: 5px">
			<Button 
				on:click={toggleTheme}
				icon
				class={$theme === 'dark' ? 'deep-purple yellow-text' : 'blue yellow-text'}
				size="default"
				style="margin-right: 2px"
			>
				<Icon class={$theme === 'dark' ? 'mdi mdi-moon-waxing-crescent' : 'mdi mdi-weather-sunny'} />
			</Button>
			<Button 
				on:click={() => aboutOpen.set(!get(aboutOpen))}
				icon
				class="white blue-text"
				size="default"
			>
				<Icon class="mdi mdi-alert-circle-outline" />
			</Button>
		</div>
	</AppBar>
	<div class="pageContent">
		<About />
		<FFmpeg />
	</div>
</MaterialApp>
  
