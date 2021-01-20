import { writable } from 'svelte/store';
import type { TVideoFile, TFFMPEGStatus } from '../utils/ffmpeg';

export const videoFile = writable<TVideoFile>(null);
export const gifFile = writable('');

export const ffmpegReady = writable(false);
export const ffmpegProgress = writable(0);
export const ffmpegConverting = writable(false);