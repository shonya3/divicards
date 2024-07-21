export const ACTIVE_LEAGUE = 'Settlers';
export const downloadText = (filename: string, text: string = 'empty') => {
	const file = new File([text], filename);
	const a = document.createElement('a');
	a.download = filename;
	a.href = URL.createObjectURL(file);
	a.click();
};
