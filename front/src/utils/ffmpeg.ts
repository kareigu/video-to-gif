import { createFFmpeg } from '@ffmpeg/ffmpeg';


export const VIDEO = 'video.mp4'
export const GIF = 'out.gif'

const inDev: boolean = import.meta.env.NODE_ENV === 'development' ? true : false;

export const ffmpeg = createFFmpeg({log: inDev, corePath: '/ffmpeg-core/dist/ffmpeg-core.js'});

