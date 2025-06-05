import { StoryContext, StoryFn } from '@storybook/web-components-vite';

function setShoelaceTheme(container: HTMLElement, theme: 'dark' | 'light'): void {
	if (theme === 'dark') {
		container.classList.add('sl-theme-dark');
	} else {
		container.classList.remove('sl-theme-dark');
	}
}

/** theme toggle Story decorator */
export function withThemeToggle(story: StoryFn, context: StoryContext): ReturnType<StoryFn> {
	const theme = context.globals.theme || 'light';

	setShoelaceTheme(document.body, theme);

	return story(context.args, context);
}
