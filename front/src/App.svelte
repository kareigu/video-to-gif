<script lang='ts'>
	import { MaterialApp, Button, AppBar, Card, ProgressLinear, Icon } from 'svelte-materialify';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { slide, fade, blur } from 'svelte/transition'
	import { clearVideo, clearGif, handleLoadingVideo, convertToGif } from './utils/ffmpeg';
	import { toggleTheme } from './utils/theme';
	import { videoFile, gifFile, ffmpegReady, ffmpegConverting, ffmpegProgress } from './stores/ffmpegStore';
	import { theme, unsupported } from './stores/utilStore';
	$: maxWindowWidth = window.innerWidth < 640 ? window.innerWidth : 640;

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
			size="default"
		>
			<Icon class={$theme === 'dark' ? 'mdi mdi-moon-waxing-crescent' : 'mdi mdi-weather-sunny'} />
		</Button>
	</AppBar>
	<div class="pageContent">
		<div class="videoTopControls" transition:slide>
			<input 
				hidden
				id="hiddenFileInput"
				type="file" 
				name="video" 
				accept="video/*" 
				on:change={handleLoadingVideo}
			>

			<Button
				outlined={$videoFile ? false : true}
				disabled={$videoFile ? true : false}
				on:click={() => document.getElementById('hiddenFileInput')?.click()}
			>
				<Icon class="mdi mdi-paperclip" />
				Open video
			</Button>
			
			{#if $unsupported}
				<p>WebAssembly isn't supported on your browser or device</p>
			{/if}

			{#if $videoFile && $ffmpegReady}
				<Button 
					on:click={clearVideo} 
					class="red"
				>
					<Icon class="mdi mdi-close" />
					Clear Video
				</Button>
			{/if}
		</div>

		{#if $videoFile && $ffmpegReady}
			<Card raised>
				<video 
					src={URL.createObjectURL(get(videoFile))} 
					controls={true} 
					transition:slide 
					width={maxWindowWidth}
				/>
			</Card>
			
			<div class="gifStuff" in:slide out:blur>
				<div class="gifTopControls" in:fade out:blur>
					{#if $ffmpegConverting}
						<div class="progressBar" in:fade out:blur>
							<ProgressLinear 
								color="blue" 
								backgroundColor="secondary" 
								value={$ffmpegProgress} 
							/>
						</div>
					{/if}
					<Button 
						on:click={convertToGif}
						disabled={$gifFile === '' ? false : true}
					>
						<Icon class="mdi mdi-file-restore" />
						Convert to GIF
					</Button>
					{#if $gifFile !== ''}
						<Button 
							on:click={clearGif} 
							class="red"
						>
							<Icon class="mdi mdi-close" />
							Clear GIF
						</Button>
					{/if}
				</div>
				{#if $gifFile !== ''}
					<Card raised>
						<img 
							src={$gifFile} 
							alt="converted gif" 
							in:fade out:blur 
							width={maxWindowWidth}
						/>
					</Card>
				{/if}
			</div>
		{/if}
	</div>
</MaterialApp>
  
