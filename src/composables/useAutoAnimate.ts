import autoAnimate from '@formkit/auto-animate';
import { Ref, onMounted } from 'vue';

export const useAutoAnimate = (templateRef: Ref<HTMLElement | null>) => {
	onMounted(() => {
		if (templateRef.value) {
			autoAnimate(templateRef.value);
		}
	});
};
