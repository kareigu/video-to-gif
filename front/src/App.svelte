<script lang='ts'>
	import { MaterialApp, Button, AppBar, Card, ProgressLinear, Icon } from 'svelte-materialify';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import FFmpeg from './components/FFmpeg.svelte';
	import { slide, fade, blur } from 'svelte/transition'
	import { clearVideo, clearGif, handleLoadingVideo, convertToGif } from './utils/ffmpeg';
	import { toggleTheme } from './utils/theme';
	import { theme, unsupported } from './stores/utilStore';

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
		<Button 
			on:click={toggleTheme}
			icon
			class={$theme === 'dark' ? 'deep-purple yellow-text' : 'blue yellow-text'}
			style="margin-right: 15px"
			size="default"
		>
			<Icon class={$theme === 'dark' ? 'mdi mdi-moon-waxing-crescent' : 'mdi mdi-weather-sunny'} />
		</Button>
	</AppBar>
	<div class="pageContent">
		<FFmpeg />
	</div>
</MaterialApp>
  
