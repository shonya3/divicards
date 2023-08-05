export type ToastVariant = 'info' | 'success' | 'neutral' | 'warning' | 'danger';

export const toast = (variant: ToastVariant, message: string) => {
	const iconVariantRecord: Record<ToastVariant, string> = {
		info: 'info-circle',
		success: 'check2-circle',
		neutral: 'gear',
		warning: 'exclamation-triangle',
		danger: 'exclamation-octagon',
	};
	const iconName = iconVariantRecord[variant];
	const duration = variant === 'warning' || variant === 'danger' ? undefined : 5_000;

	const variantProp = variant === 'info' ? 'primary' : variant;
	const alert = Object.assign(document.createElement('sl-alert'), {
		closable: true,
		duration,
		variant: variantProp,
	});

	const icon = Object.assign(document.createElement('sl-icon'), {
		name: iconName,
		slot: 'icon',
	});

	alert.append(icon, message);
	alert.toast();
};
