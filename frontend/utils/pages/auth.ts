import { GetServerSidePropsContext } from "next";
// @ts-ignore https://github.com/nextauthjs/next-auth/pull/220 is not merged yet.
import { getSession } from "next-auth/client";

export const requireAuthentication = async (
  context: GetServerSidePropsContext
): Promise<any> => {
  const session = await getSession(context);

  if (!session) {
    console.log("about to writeHEAD");
    context.res.writeHead(302, {
      Location: `/api/auth/signin?callbackUrl=${encodeURIComponent(
        process.env.NEXTAUTH_URL + (context.req.url ?? "")
      )}`,
    });
    console.log("about to end");
    context.res.end();
    console.log("ended");
    return;
  }
  try {
    console.log(session);
    return session;
  } catch (error) {
    console.error(error);
    throw error;
  }
};
