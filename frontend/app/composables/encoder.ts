import { BioEncoder } from "~/lib/seed";
import type { ConfigRoot } from "~/types/config.types";

export const useBioEncoder = () => {
	let encoder: {
		encode: (selectedIds: string[]) => string;
		decode: (seed: string) => string[];
	} | null = null;

	function initEncoder(v: ConfigRoot) {
		encoder = BioEncoder(v);
	}

	function getSeed(selected: string[]) {
		return encoder?.encode(selected) || ""; //
	}

	function loadFromSeed(seed: string) {
		return encoder?.decode(seed) || [];
	}

	return { getSeed, loadFromSeed, initEncoder };
};
