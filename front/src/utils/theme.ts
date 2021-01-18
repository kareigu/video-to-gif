export type TTheme = 'dark' | 'light';

export function initTheme() {
  return localStorage.getItem('theme') === 'light' ? 'light' : 'dark';
}

export function toggleTheme(theme: TTheme) {
  theme = theme === 'dark' ? 'light' : 'dark';
  document.body.setAttribute('class', `theme--${theme}`);
  localStorage.setItem('theme', theme);
  return theme;
}
