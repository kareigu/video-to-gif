import { createFFmpeg, fetchFile } from '@ffmpeg/ffmpeg';

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

export function videoClearer(gifFile: string) {
  const input = document.getElementById('hiddenFileInput');
  if(input)
    input.value = null;
  if(gifFile !== '') {
    ffmpeg.FS('unlink', GIF);
  }
  return '';
}

export function gifClearer() {
  ffmpeg.FS('unlink', GIF);
  return '';
}


export const VIDEO = 'video.mp4'
export const GIF = 'out.gif'

const inDev: boolean = import.meta.env.NODE_ENV === 'development' ? true : false;

export const ffmpeg = createFFmpeg({log: inDev, corePath: '/ffmpeg-core/dist/ffmpeg-core.js'});

