const MOCK_URL = "/api";

export type ToastEvent =
	| { type: "loading" }
	| { type: "dismiss" }
	| { type: "error"; status: number; statusText: string }

export const toastBus = useEventBus<ToastEvent>("api-toast")
export const currentLoadingId = ref<string | number>("")

const api = $fetch.create({
	baseURL: MOCK_URL,
	onRequest: () => {
		toastBus.emit({ type: "loading" });
	},
	onResponse: () => {
		toastBus.emit({ type: "dismiss" });
	},
	onResponseError: (err) => {
		console.log(err);
		toastBus.emit({ type: "dismiss" });
		toastBus.emit({
			type: "error",
			status: err.response.status,
			statusText: err.response.statusText,
		});
	},
});

export const useApi: typeof useFetch = (url, opts?) =>
	useFetch(url, { $fetch: api, ...opts });
