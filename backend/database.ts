async function readText(rs: ReadableStream<Uint8Array>) {
  const reader = rs.getReader();
  const value = (await reader.read()).value;
  reader.cancel();
  return new TextDecoder().decode(value);
}

async function readJSON(rs: ReadableStream<Uint8Array>) {
  const text = await readText(rs);

  if (text === "") return {};

  return JSON.parse(text);
}

export const restAPI_URL =
  "https://amirroyzluivtvybgpew.supabase.co/rest/v1/happy-new-year";

export async function getUsers(): Promise<{ name: string }[]> {
  const apiKey = Deno.env.get("SUPABASE_KEY")!;
  return readJSON(
    (
      await fetch(restAPI_URL + "?select=name", {
        headers: {
          apiKey,
          Authorization: "Bearer " + apiKey,
        },
      })
    ).body!
  );
}

export async function login(name: string) {
  const apiKey = Deno.env.get("SUPABASE_KEY")!;
  return readJSON(
    (
      await fetch(restAPI_URL, {
        method: "POST",
        headers: {
          apiKey,
          Authorization: "Bearer " + apiKey,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ name }),
      })
    ).body!
  );
}
