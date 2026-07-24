import { html, css, LitElement, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { Directive, DirectiveParameters, DirectiveResult, directive } from 'lit/directive.js';
import { ElementPart, render } from 'lit';

// Positioning library
import { computePosition, autoPlacement, offset, shift } from '@floating-ui/dom';

// Events to turn on/off the tooltip
const enterEvents = ['pointerenter', 'focus', 'click'];
const leaveEvents = ['pointerleave', 'blur'];

@customElement('simple-tooltip')
export class SimpleTooltip extends LitElement {
	// Lazy creation
	static lazy(target: Element, callback: (target: SimpleTooltip) => void): void {
		const createTooltip = () => {
			const tooltip = document.createElement('simple-tooltip') as SimpleTooltip;
			callback(tooltip);
			target.parentNode!.insertBefore(tooltip, target.nextSibling);
			tooltip.show();
			// We only need to create the tooltip once, so ignore all future events.
			enterEvents.forEach(eventName => target.removeEventListener(eventName, createTooltip));
		};
		enterEvents.forEach(eventName => target.addEventListener(eventName, createTooltip));
	}

	static styles: CSSResult = css`
		:host {
			/* Position fixed to help ensure the tooltip is "on top" */
			position: fixed;
			/*border: 1px solid darkgray;
			background-color: rgba(0, 0, 0, 0.8);
            
            */
			padding: 4px;
			border-radius: 4px;
			display: inline-block;
			pointer-events: none;
			z-index: 900;

			opacity: 0;
		}

		:host([showing]) {
			opacity: 1;
		}
	`;

	// Attribute for styling "showing"
	@property({ reflect: true, type: Boolean })
	showing = false;

	// Position offset
	@property({ type: Number })
	offset = 4;

	constructor() {
		super();
		// Finish hiding at end of animation
		this.addEventListener('transitionend', this.finishHide);
	}

	connectedCallback(): void {
		super.connectedCallback();
		// Setup target if needed
		this.target ??= this.previousElementSibling;
		// Ensure hidden at start
		this.finishHide();
	}

	// Target for which to show tooltip
	_target: Element | null = null;
	get target() {
		return this._target;
	}
	set target(target: Element | null) {
		// Remove events from existing target
		if (this.target) {
			enterEvents.forEach(name => this.target!.removeEventListener(name, this.show));
			leaveEvents.forEach(name => this.target!.removeEventListener(name, this.hide));
		}
		if (target) {
			// Add events to new target
			enterEvents.forEach(name => target!.addEventListener(name, this.show));
			leaveEvents.forEach(name => target!.addEventListener(name, this.hide));
		}
		this._target = target;
	}

	show = async (): Promise<void> => {
		await new Promise(resolve => setTimeout(resolve));
		this.style.cssText = '';
		computePosition(this.target!, this, {
			strategy: 'fixed',
			middleware: [offset(this.offset), shift(), autoPlacement({ allowedPlacements: ['top', 'bottom'] })],
		}).then(({ x, y }: { x: number; y: number }) => {
			this.style.left = `${x}px`;
			this.style.top = `${y}px`;
		});
		this.showing = true;
	};

	hide = async (): Promise<void> => {
		await new Promise(resolve => setTimeout(resolve));
		this.showing = false;
	};

	finishHide = (): void => {
		if (!this.showing) {
			this.style.display = 'none';
		}
	};

	render(): TemplateResult {
		return html`<slot></slot>`;
	}
}

class TooltipDirective extends Directive {
	didSetupLazy = false;
	tooltipContent?: unknown;
	part?: ElementPart;
	tooltip?: SimpleTooltip;
	render(_: unknown = ''): void {}
	update(part: ElementPart, [tooltipContent]: DirectiveParameters<this>): void {
		this.tooltipContent = tooltipContent;
		this.part = part;
		if (!this.didSetupLazy) {
			this.setupLazy();
		}
		if (this.tooltip) {
			this.renderTooltipContent();
		}
	}
	setupLazy(): void {
		this.didSetupLazy = true;
		SimpleTooltip.lazy(this.part!.element, (tooltip: SimpleTooltip) => {
			this.tooltip = tooltip;
			this.renderTooltipContent();
		});
	}
	renderTooltipContent(): void {
		render(this.tooltipContent, this.tooltip!, this.part!.options);
	}
}

export const tooltip: () => DirectiveResult<typeof TooltipDirective> = directive(TooltipDirective);

declare global {
	interface HTMLElementTagNameMap {
		'simple-tooltip': SimpleTooltip;
	}
}
