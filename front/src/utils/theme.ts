import { get } from 'svelte/store';
import { theme } from '../stores/utilStore';
export type TTheme = 'dark' | 'light';

export function initTheme() {
  return localStorage.getItem('theme') === 'light' ? 'light' : 'dark';
}

export function toggleTheme() {
  theme.set(get(theme) === 'dark' ? 'light' : 'dark')
  document.body.setAttribute('class', `theme--${get(theme)}`);
  localStorage.setItem('theme', get(theme));
}
