import cookies from "next-cookies";
import { GetServerSidePropsContext } from "next";
import axios from "axios";

export const requireAuthentication = async (
  context: GetServerSidePropsContext
) => {
  const { ory_kratos_session } = cookies(context);

  if (context.res && !ory_kratos_session) {
    context.res.writeHead(302, {
      Location: `/.ory/kratos/public/self-service/browser/flows/login?return_to=${context.req.url}`,
    });
    context.res.end();
  }
  try {
    const sessionResponse = await axios.request({
      url: "http://kratos-public.kratos/sessions/whoami",
      method: "get",
      headers: {
        Cookie: `ory_kratos_session=${ory_kratos_session}`,
      },
    });

    return sessionResponse;
  } catch (error) {
    console.error(error);
    throw error;
  }
};
