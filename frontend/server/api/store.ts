import { faker } from "@faker-js/faker";
import type { MarketItem } from "~/types/config.types";

interface payload {
	chunkIndex?: number;
}

const generateData = (length: number): MarketItem[] =>
	Array.from({ length }).map((_) => {
		return {
			id: `${faker.person.firstName()}/${faker.animal.cat()}`,
			label: `${faker.animal.cat()} Boilerplate`,
			trusted: faker.datatype.boolean(),
			description: faker.word.words({ count: 50 }),
			source: `https://github.com/git/${faker.person.firstName()}/${faker.git.branch()}`,
			preview:
				"https://github.com/git/${faker.person.firstName()}/${faker.git.branch()}",
			git: {
				stars: faker.number.int({ min: 0, max: 10_000 }),
				lastActivity: Date.now(),
				activeIssueCount: faker.number.int({ min: 0, max: 10_000 }),
				closedIssueCount: faker.number.int({ min: 0, max: 10_000 }),
			},
		};
	});

const chunk = (arr: any[], size: number) =>
	Array.from({ length: Math.ceil(arr.length / size) }, (_, i) =>
		arr.slice(i * size, i * size + size),
	);

const data = chunk(generateData(50), 10);
export default defineEventHandler((event) => {
	const params: payload = getRouterParams(event);

	return {
		total: data.length,
		items: data[params.chunkIndex || 0],
	};
});
