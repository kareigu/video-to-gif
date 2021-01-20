import { writable } from 'svelte/store';
import { initTheme } from '../utils/theme';
import isMobile from '../utils/isMobile';
import type { TTheme } from '../utils/theme';

export const theme = writable<TTheme>(initTheme());

export const unsupported = writable(isMobile());

export const aboutOpen = writable(false);