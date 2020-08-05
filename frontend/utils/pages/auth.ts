import cookies from "next-cookies";
import { GetServerSidePropsContext } from "next";
import { PublicApi, Session } from "@oryd/kratos-client";
import http from "http";

export const requireAuthentication = async (
  context: GetServerSidePropsContext
): Promise<{
  response: http.IncomingMessage;
  body: Session;
}> => {
  const { ory_kratos_session } = cookies(context);

  if (context.res && !ory_kratos_session) {
    context.res.writeHead(302, {
      Location: `/.ory/kratos/public/self-service/browser/flows/login?return_to=${context.req.url}`,
    });
    context.res.end();
  }
  try {
    const api = new PublicApi("http://kratos-public.kratos");
    const response = await api.whoami({
      headers: { Cookie: `ory_kratos_session=${ory_kratos_session}` },
    });
    return response;
  } catch (error) {
    console.error(error);
    throw error;
  }
};
