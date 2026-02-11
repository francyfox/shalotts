import { Elysia } from "elysia";
import { html } from "@elysiajs/html";
export const core = new Elysia();
core.use(html());
