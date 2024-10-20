import type { PageServerLoad } from "./$types";
import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { z } from "zod";

const schema = z.object({
  url: z.string().url(),
});

export const load = (async () => {
  const form = await superValidate(zod(schema));
  return { form };
}) satisfies PageServerLoad;
