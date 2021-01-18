<script lang='ts'>
	import { MaterialApp, Button, AppBar, Card, ProgressLinear, Icon } from 'svelte-materialify';
	import { onMount } from 'svelte';
	import { slide, fade, blur } from 'svelte/transition'
	import { fetchFile } from '@ffmpeg/ffmpeg';
	import { ffmpeg, VIDEO, GIF } from './utils/ffmpeg';
	import { toggleTheme, initTheme } from './utils/theme';
	import type { TTheme } from './utils/theme';


	let videoFile: File | null;
	let ffmpegReady = false;
	let gifFile: string = '';
	let convertingGif = false;
	let conversionProgress = 0;
	let theme: TTheme = initTheme();

	$: maxWindowWidth = window.innerWidth < 640 ? window.innerWidth : 640;

	function handleThemeToggle() {
		theme = toggleTheme(theme);
	}


	async function loadFFMPEG() {
		await ffmpeg.load();
		ffmpeg.setProgress(({ratio}) => conversionProgress = ratio * 100) ;
		ffmpegReady = true;
	}

	onMount(() => {
		document.body.setAttribute('class', `theme--${theme}`);

		return () => {
			clearVideo();
		}
	});



	function handleLoadingVideo(e: Event) {
		if(e.target) {
			if(!ffmpegReady)
				loadFFMPEG();
			const target: HTMLInputElement = e.target;
			videoFile = target.files?.item(0) ? target.files?.item(0) : null;
		}
	}

	async function convertToGif() {
		if(videoFile) {
			convertingGif = true;
			ffmpeg.FS('writeFile', VIDEO, await fetchFile(videoFile));
			await ffmpeg.run('-i', VIDEO, GIF);

			const data = ffmpeg.FS('readFile', GIF);
			gifFile = URL.createObjectURL(new Blob([data.buffer], { type: 'image/gif' }));
			convertingGif = false;
			conversionProgress = 0;
			ffmpeg.FS('unlink', VIDEO)
		}
	}

	async function clearGif() {
		ffmpeg.FS('unlink', GIF);
		gifFile = '';
	}

	async function clearVideo() {
		videoFile = null;
		document.getElementById('hiddenFileInput')!.value = null;
		if(gifFile !== '') {
			ffmpeg.FS('unlink', GIF);
			gifFile = '';
		}
	}

</script>
  
<style type="text/scss">
	@import 'App.scss';
</style>
  
<MaterialApp theme={theme}>
	<AppBar class="secondary-color theme--dark">
		<span slot="title">
			<Icon class="mdi mdi-camera-image" />
			Convert video to GIF
		</span>
		<div style="flex-grow:1" />
		<Button 
			on:click={handleThemeToggle}
			icon
			class={theme === 'dark' ? 'deep-purple yellow-text' : 'blue yellow-text'}
			size="default"
		>
			<Icon class={theme === 'dark' ? 'mdi mdi-moon-waxing-crescent' : 'mdi mdi-weather-sunny'} />
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
				outlined={videoFile ? false : true}
				disabled={videoFile ? true : false}
				on:click={() => document.getElementById('hiddenFileInput')?.click()}
			>
				<Icon class="mdi mdi-paperclip" />
				Open video
			</Button>

			{#if videoFile && ffmpegReady}
				<Button 
					on:click={clearVideo} 
					class="red"
				>
					<Icon class="mdi mdi-close" />
					Clear Video
				</Button>
			{/if}
		</div>

		{#if videoFile && ffmpegReady}
			<Card raised>
				<video 
					src={URL.createObjectURL(videoFile)} 
					controls={true} 
					transition:slide 
					width={maxWindowWidth}
				/>
			</Card>
			
			<div class="gifStuff" in:slide out:blur>
				<div class="gifTopControls" in:fade out:blur>
					{#if convertingGif}
						<div class="progressBar" in:fade out:blur>
							<ProgressLinear 
								color="blue" 
								backgroundColor="secondary" 
								value={conversionProgress} 
							/>
						</div>
					{/if}
					<Button 
						on:click={convertToGif}
						disabled={gifFile === '' ? false : true}
					>
						<Icon class="mdi mdi-file-restore" />
						Convert to GIF
					</Button>
					{#if gifFile !== ''}
						<Button 
							on:click={clearGif} 
							class="red"
						>
							<Icon class="mdi mdi-close" />
							Clear GIF
						</Button>
					{/if}
				</div>
				{#if gifFile !== ''}
					<Card raised>
						<img 
							src={gifFile} 
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
  