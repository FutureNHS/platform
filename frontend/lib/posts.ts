import fetch from "node-fetch";

export async function getGreeting(name: string) {
  const res = await fetch(`http://hello-world.hello-world/hello/${name}`);

  const jsonFormatted = await res.text();
  return jsonFormatted;
}
