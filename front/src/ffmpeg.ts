import { createFFmpeg } from '@ffmpeg/ffmpeg';

function initCreateFFmpeg(inDev: boolean) {
  const ffmpeg = createFFmpeg({log: inDev, corePath: '/ffmpeg-core/dist/ffmpeg-core.js'});
  return ffmpeg;
}

export default initCreateFFmpeg;

