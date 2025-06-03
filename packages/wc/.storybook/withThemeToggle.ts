import { StoryContext, StoryFn } from '@storybook/web-components-vite';

/** theme toggle Story decorator */
export const withThemeToggle = (story: StoryFn, context: StoryContext) => {
	const theme = context.globals.theme || 'light';

	if (theme === 'dark') {
		document.body.classList.add('sl-theme-dark');
		document.documentElement.setAttribute('data-theme', 'dark');
	} else {
		document.body.classList.remove('sl-theme-dark');
		document.documentElement.setAttribute('data-theme', 'light');
	}

	return story(context.args, context);
};
