import { LitElement, html, css, nothing, CSSResult, TemplateResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { Requirement } from '../../poe.types.js';

@customElement('poe-item-info-requirements')
export class PoeItemInfoRequirementsElement extends LitElement {
	@property({ type: Array }) requirements: Requirement[] = [];

	protected render(): TemplateResult {
		return html`Requires
		${this.requirements.map((requirement, index) => {
			switch (requirement.displayMode) {
				case 0: {
					if (!requirement.values.length) {
						return requirement.name;
					}

					return html`${requirement.name} <span class="value">${requirement.values[0][0]}</span>${index ===
						this.requirements.length - 1
							? nothing
							: ', '}`;
				}
				case 1: {
					return html`<span class="value">${requirement.values[0][0]}</span> ${requirement.name}
						${index === this.requirements.length - 1 ? nothing : ', '}`;
				}
				default: {
					console.warn(
						`Unexpected displayMode for Requirement. Exptected 0 | 1, got: ${requirement.displayMode}`
					);
					return nothing;
				}
			}
		})}`;
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			color: #7f7f7f;
		}

		.value {
			color: #fff;
		}
	`;
}

declare global {
	interface HTMLElementTagNameMap {
		'poe-item-info-requirements': PoeItemInfoRequirementsElement;
	}
}
