import cookie from "cookie";
import { GetServerSidePropsContext } from "next";
import http from "http";
import { publicApi } from "../kratos";

export const requireAuthentication = async (
  context: GetServerSidePropsContext
): Promise<{
  response: http.IncomingMessage;
}> => {
  const { ory_kratos_session } = cookie.parse(
    context.req?.headers?.cookie || ""
  );

  if (context.res && !ory_kratos_session) {
    context.res.writeHead(302, {
      Location: `/.ory/kratos/public/self-service/browser/flows/login?return_to=${context.req.url}`,
    });
    context.res.end();
  }
  try {
    const response = await publicApi.whoami({
      headers: { Cookie: `ory_kratos_session=${ory_kratos_session}` },
    });
    return response;
  } catch (error) {
    console.error(error);
    throw error;
  }
};
