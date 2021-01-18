<script lang='ts'>
	import { MaterialApp, Button, AppBar, Card, ProgressLinear, Icon } from 'svelte-materialify';
	import { onMount } from 'svelte';
	import { slide, fade, blur } from 'svelte/transition'
	import { ffmpeg, processVideo, videoClearer, gifClearer } from './utils/ffmpeg';
	import { toggleTheme, initTheme } from './utils/theme';
	import type { TTheme } from './utils/theme';
	import type { TFFMPEGStatus, TVideoFile } from './utils/ffmpeg';


	let videoFile: TVideoFile;
	let gifFile: string = '';
	let ffmpegStatus: TFFMPEGStatus = {
		ready: false,
		progress: 0,
		converting: false
	}
	let theme: TTheme = initTheme();

	$: maxWindowWidth = window.innerWidth < 640 ? window.innerWidth : 640;

	function handleThemeToggle() {
		theme = toggleTheme(theme);
	}


	async function loadFFMPEG() {
		await ffmpeg.load();
		ffmpeg.setProgress(({ratio}) => ffmpegStatus.progress = ratio * 100) ;
		ffmpegStatus.ready = true;
	}

	onMount(() => {
		document.body.setAttribute('class', `theme--${theme}`);

		return () => {
			clearVideo();
		}
	});



	function handleLoadingVideo(e: Event) {
		if(e.target) {
			if(!ffmpegStatus.ready)
				loadFFMPEG();
			//@ts-expect-error
			const target: HTMLInputElement = e.target;
			videoFile = target.files?.item(0) ? target.files?.item(0) : null;
		}
	}

	async function convertToGif() {
		if(videoFile) {
			ffmpegStatus.converting = true;
			gifFile = await processVideo(videoFile);
			ffmpegStatus = {...ffmpegStatus, converting: false, progress: 0};
		}
	}

	function clearGif() {
		gifFile = gifClearer();
	}

	function clearVideo() {
		videoFile = null;
		gifFile = videoClearer(gifFile);
	}

</script>

<style lang="scss">
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

			{#if videoFile && ffmpegStatus.ready}
				<Button 
					on:click={clearVideo} 
					class="red"
				>
					<Icon class="mdi mdi-close" />
					Clear Video
				</Button>
			{/if}
		</div>

		{#if videoFile && ffmpegStatus.ready}
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
					{#if ffmpegStatus.converting}
						<div class="progressBar" in:fade out:blur>
							<ProgressLinear 
								color="blue" 
								backgroundColor="secondary" 
								value={ffmpegStatus.progress} 
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
  