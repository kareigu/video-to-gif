<script lang='ts'>
	import { MaterialApp, Button, AppBar, Card } from 'svelte-materialify';
	import { onMount } from 'svelte';
	import { slide, fade, blur } from 'svelte/transition'
	import { createFFmpeg, fetchFile } from '@ffmpeg/ffmpeg';
import { outro_and_destroy_block } from 'svelte/internal';

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
		document.body.setAttribute('class', `theme--${theme}`);

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
		videoFile = null;
		document.getElementById('hiddenFileInput')!.value = null;
		if(gifFile !== '') {
			ffmpeg.FS('unlink', GIF);
			gifFile = '';
		}
	}

</script>
  
<style type="text/scss">

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

				{#if videoFile}
					<Button on:click={clearVideo} class="red">Clear Video</Button>
				{/if}
			</div>

			{#if videoFile}
				<Card raised>
					<video src={URL.createObjectURL(videoFile)} controls={true} transition:slide />
				</Card>
				
				<div class="gifStuff" transition:fade>
					<div class="gifTopControls" in:fade out:blur>
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
							<img src={gifFile} alt="converted gif" in:fade out:blur />
						</Card>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</MaterialApp>
  