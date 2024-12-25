import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ parent }) => {
  const parentData = await parent();
  let response = await fetch(`http://${parentData.api_url}/classes`);
  return {
    post: await response.text(),
  };
};
