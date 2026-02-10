import { toast } from "vue-sonner";

export default defineNuxtPlugin(() => {
	const apiFetch = $fetch.create({
		baseURL: "/api",
		onRequest: (): any => {
			const loadingId = toast.loading("Loading...");
			return { context: { loadingId } };
		},
		onResponse: ({ options }) => {
			const loadingId = (options as any).context?.loadingId;
			if (loadingId) {
				toast.dismiss(loadingId);
			}
		},
		onResponseError: ({ response, options }) => {
			const loadingId = (options as any).context?.loadingId;
			if (loadingId) {
				toast.dismiss(loadingId);
			}
			toast.error(`${response.status} Error!`, {
				description: response.statusText,
				duration: 5000,
			});
		},
	});

	return {
		provide: {
			apiFetch,
		},
	};
});
