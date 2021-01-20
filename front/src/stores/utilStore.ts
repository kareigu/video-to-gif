import { writable } from 'svelte/store';
import { initTheme } from '../utils/theme';
import type { TTheme } from '../utils/theme';

export const theme = writable<TTheme>(initTheme());