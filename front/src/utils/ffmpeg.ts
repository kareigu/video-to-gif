import { createFFmpeg, fetchFile } from '@ffmpeg/ffmpeg';
import { get } from 'svelte/store';
import { videoFile, gifFile, ffmpegReady, ffmpegConverting, ffmpegProgress } from '../stores/ffmpegStore';
import { unsupported } from '../stores/utilStore';

export type TVideoFile = File | null;
export type TFFMPEGStatus = {
  ready: boolean,
  progress: number,
  converting: boolean
}

export async function processVideo(videoFile: TVideoFile) {
  ffmpeg.FS('writeFile', VIDEO, await fetchFile(videoFile!));
  await ffmpeg.run('-i', VIDEO, GIF);

  ffmpeg.FS('unlink', VIDEO)
  const data = ffmpeg.FS('readFile', GIF);
  return URL.createObjectURL(new Blob([data.buffer], { type: 'image/gif' }));
}


export function clearVideo() {
  const input = document.getElementById('hiddenFileInput');
  if(input)
    //@ts-expect-error
    input.value = null;
  if(get(gifFile) !== '') {
    clearGif();
  }
  videoFile.set(null);
}

export function clearGif() {
  ffmpeg.FS('unlink', GIF);
  gifFile.set('');
}

async function loadFFMPEG() {
  await ffmpeg.load();
  if(ffmpeg.isLoaded()) {
    ffmpeg.setProgress(({ratio}) => ffmpegProgress.set(ratio * 100));
    ffmpegReady.set(true);
  } else {
    unsupported.set(true);
  }
}

export function handleLoadingVideo(e: Event) {
  if(e.target) {
    if(!get(ffmpegReady))
      loadFFMPEG();
    //@ts-expect-error
    const target: HTMLInputElement = e.target;
    videoFile.set(target.files?.item(0) ? target.files?.item(0) : null);
  }
}


export async function convertToGif() {
  if(get(videoFile)) {
    ffmpegConverting.set(true);
    gifFile.set(await processVideo(get(videoFile)));
    ffmpegConverting.set(false);
    ffmpegProgress.set(0);
  }
}


const VIDEO = 'video.mp4'
const GIF = 'out.gif'

const inDev: boolean = import.meta.env.NODE_ENV === 'development' ? true : false;

export const ffmpeg = createFFmpeg({log: inDev, corePath: '/ffmpeg-core/dist/ffmpeg-core.js'});

