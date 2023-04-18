export const csvFile = (csv: string, name: `${string}.csv`): File => new File([csv], name, { type: 'text/csv' });

export const createDownloadAnchor = (href: string, filename: string): HTMLAnchorElement => {
	const a = document.createElement('a');
	a.download = `${filename}`;
	a.href = href;
	return a;
};

export const downloadFile = (filename: string, href: string): void => {
	createDownloadAnchor(href, filename).click();
};
