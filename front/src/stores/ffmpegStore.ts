import { writable } from 'svelte/store';
import type { TVideoFile } from '../utils/ffmpeg';

export const videoFile = writable<TVideoFile>(null);
export const gifFile = writable('');