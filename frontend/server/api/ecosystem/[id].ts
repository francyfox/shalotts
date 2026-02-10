import { Elysia, ElysiaData, testBuild } from "#server/data";

interface payload {
	id?: string;
}
export default defineEventHandler((event) => {
	const params: payload = getRouterParams(event);
	const ecosystem = Elysia.ecosystems.find((i) => i.id === params.id);

	if (!ecosystem) setResponseStatus(event, 500);
	if (ecosystem?.id === Elysia.metadata.id) return ElysiaData;
	if (ecosystem?.id === testBuild().metadata.id) return ElysiaData;
});
