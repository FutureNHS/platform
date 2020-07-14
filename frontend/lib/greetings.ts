import "isomorphic-fetch";

export const getGreeting = async (name: string | string[]) => {
  const res = await fetch(`http://hello-world.hello-world/hello/${name}`);

  const jsonFormatted = await res.text();
  return jsonFormatted;
};
