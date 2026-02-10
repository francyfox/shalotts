import { createEnv } from "@t3-oss/env-nuxt";
import { z } from "zod";

export const env = createEnv({
	client: {
		NUXT_PUBLIC_PRIVATE_URL: z.string().optional(),
		NUXT_PUBLIC_IS_PUBLIC: z.boolean().default(false),
	},
});
