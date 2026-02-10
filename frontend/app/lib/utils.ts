import type { ClassValue } from "clsx";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import type { ConfigRoot } from "~/types/config.types";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function getDefaultSelected(registries: ConfigRoot["registry"]) {
	return registries.filter((i) => i.default).map((i) => i.id);
}
