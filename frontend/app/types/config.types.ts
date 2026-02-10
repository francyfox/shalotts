export interface Config extends ConfigRoot, ConfigData {}
export interface ConfigRoot {
	version: string;
	metadata: Metadata;
	ecosystems: Metadata[];
}

export interface ConfigData {
	groups: Group[];
	tags: Tag[];
	registry: Registry[];
}

export interface MarketItem {
	id: string;
	label: string;
	source: string;
	description: string;
	git?: {
		stars: number;
		lastActivity: number;
		activeIssueCount: number;
		closedIssueCount: number;
	};
}

export interface Metadata {
	id: string;
	label: string;
}

export interface Group {
	id: string;
	label: string;
	required: boolean;
}

export interface Tag {
	id: string;
	icon: string;
}

export interface Registry {
	id: string;
	label: string;
	group: string;
	target: string[];
	template?: string;
	icon: string;
	tags: string[];
	default?: boolean;
	source?: string;
}
