<script lang='ts'>
	import { onMount } from 'svelte';
	import { createFFmpeg, fetchFile } from '@ffmpeg/ffmpeg';

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
	}

</script>
  
<style type="text/scss">
:global(body) {
	margin: 0;
	font-family: Arial, Helvetica, sans-serif;
}

.App {
	text-align: center;
}
</style>
  
<div class="App">
	<h2>
		Convert video to GIF
	</h2>
	{#if ffmpegReady}
		<input 
			type="file" 
			name="video" 
			accept="video/*" 
			on:change={handleLoadingVideo}
		>

		{#if videoFile}
			<button on:click={clearVideo}>Clear Video</button>
			<video src={URL.createObjectURL(videoFile)} controls={true} />

			
			<button on:click={convertToGif}>Convert to GIF</button>
			{#if gifFile !== ''}
				<button on:click={clearGif}>Clear GIF</button>
				<img src={gifFile} alt="converted gif" />
			{/if}
		{/if}
	{/if}
</div>
  