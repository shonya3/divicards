export function emit<T>(
	element: HTMLElement,
	eventName: string,
	detail?: T,
	options: EventInit = { bubbles: true, composed: true, cancelable: true }
) {
	const event = new CustomEvent<T>(eventName, { detail, ...options });
	element.dispatchEvent(event);
}
