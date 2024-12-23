import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  let response = await fetch("http://localhost:3000/");
  return {
    post: await response.text(),
  };
};
