<script lang='ts'>
	import { MaterialApp, Button, AppBar } from 'svelte-materialify';
	import { onMount } from 'svelte';
	import { slide, fade } from 'svelte/transition'
	import { createFFmpeg, fetchFile } from '@ffmpeg/ffmpeg';

	let theme: 'dark' | 'light' = 'dark';

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
		document.body.setAttribute('class', `theme--${theme}`);
	}

	const VIDEO = 'video.mp4'
	const GIF = 'out.gif'

	const ffmpeg = createFFmpeg({ log: true });
	let videoFile: File | null;
	let ffmpegReady = false;
	let gifFile: string = '';


	async function loadFFMPEG() {
		await ffmpeg.load();
		ffmpegReady = true;
	}

	onMount(() => {
		loadFFMPEG();

		return () => {
			clearVideo();
		}
	});



	function handleLoadingVideo(e: Event) {
		if(e.target) {
			const target: HTMLInputElement = e.target;
			videoFile = target.files?.item(0) ? target.files?.item(0) : null;
		}
	}

	async function convertToGif() {
		if(videoFile) {
			ffmpeg.FS('writeFile', VIDEO, await fetchFile(videoFile));
			await ffmpeg.run('-i', VIDEO, GIF);

			const data = ffmpeg.FS('readFile', GIF);
			gifFile = URL.createObjectURL(new Blob([data.buffer], { type: 'image/gif' }));
			ffmpeg.FS('unlink', VIDEO)
		}
	}

	async function clearGif() {
		ffmpeg.FS('unlink', GIF);
		gifFile = '';
	}

	async function clearVideo() {
		document.getElementById('hiddenFileInput')!.value = null;
		if(gifFile !== '') {
			ffmpeg.FS('unlink', GIF);
			gifFile = '';
		}
		videoFile = null;
	}

</script>
  
<style type="text/scss">

	.pageContent {
		margin-top: 25px;
		text-align: center;
		height: 100vh;
	}
</style>
  
<MaterialApp theme={theme}>
	<AppBar class="secondary-color theme--dark">
		<span slot="title">
			Convert video to GIF
		</span>
		<div style="flex-grow:1" />
		<Button on:click={toggleTheme}>Toggle theme</Button>
	</AppBar>
	<div class="pageContent">
		{#if ffmpegReady}
			<input 
				hidden
				id="hiddenFileInput"
				type="file" 
				name="video" 
				accept="video/*" 
				on:change={handleLoadingVideo}
			>

			<label for="hiddenFileInput" id="LblBrowse" transition:fade>
				<Button
					outlined
					on:click={() => document.getElementById('hiddenFileInput')?.click()}
				>
					Open video
				</Button>
			</label>

			{#if videoFile}
				<Button on:click={clearVideo}>Clear Video</Button>
				<video src={URL.createObjectURL(videoFile)} controls={true} transition:slide />

				
				<Button on:click={convertToGif}>Convert to GIF</Button>
				{#if gifFile !== ''}
					<Button on:click={clearGif}>Clear GIF</Button>
					<img src={gifFile} alt="converted gif" />
				{/if}
			{/if}
		{/if}
	</div>
</MaterialApp>
  