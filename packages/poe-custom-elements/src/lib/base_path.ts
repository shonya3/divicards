/**
 * Default base path of icons and images folders.
 */
export const BASE_PATH = '/poe-ce-assets';
let _basePath = BASE_PATH;
export function setBasePath(path: string): void {
	_basePath = path;
}
/**
 * Gets the library's base path.
 * The base path is used to load assets such as icons and images.
 * @returns Base path. For example, '/poe-ce-assets'
 */
export function basePath(): string {
	return _basePath;
}
