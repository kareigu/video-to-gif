<script lang='ts'>
	import { MaterialApp, Button, AppBar, Card, ProgressLinear, Icon } from 'svelte-materialify';
	import { onMount } from 'svelte';
	import { slide, fade, blur } from 'svelte/transition'
	import { createFFmpeg, fetchFile } from '@ffmpeg/ffmpeg';

	let theme: 'dark' | 'light' = localStorage.getItem('theme') === 'light' ? 'light' : 'dark';

	const inDev: boolean = import.meta.env.NODE_ENV === 'development' ? true : false;

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
		document.body.setAttribute('class', `theme--${theme}`);
		localStorage.setItem('theme', theme);
	}

	const VIDEO = 'video.mp4'
	const GIF = 'out.gif'

	const ffmpeg = createFFmpeg({log: inDev});
	let videoFile: File | null;
	let ffmpegReady = false;
	let gifFile: string = '';
	let convertingGif = false;
	let conversionProgress = 0;

	$: maxWindowWidth = window.innerWidth < 640 ? window.innerWidth : 640;


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
	@import url(https://cdn.materialdesignicons.com/5.4.55/css/materialdesignicons.min.css);

	:global(body) {
		overflow-x: hidden;
	}

	.pageContent {
		margin-top: 25px;
		padding-bottom: 30px;
		text-align: center;
	}

	.videoTopControls {
		margin-bottom: 20px;
	}

	.gifStuff {
		margin-top: 20px;
	}

	.gifTopControls {
		margin-bottom: 10px;
	}

	.progressBar {
		margin-bottom: 5px;
	}
</style>
  
<MaterialApp theme={theme}>
	<AppBar class="secondary-color theme--dark">
		<span slot="title">
			Convert video to GIF
		</span>
		<div style="flex-grow:1" />
		<Button 
			on:click={toggleTheme}
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
				Open video
			</Button>

			{#if videoFile && ffmpegReady}
				<Button on:click={clearVideo} class="red">Clear Video</Button>
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
						Convert to GIF
					</Button>
					{#if gifFile !== ''}
						<Button on:click={clearGif} class="red">Clear GIF</Button>
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
  