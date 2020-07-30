import cookies from "next-cookies";
import { GetServerSidePropsContext } from "next";

export const requireAuthentication = async (
  context: GetServerSidePropsContext
) => {
  const { ory_kratos_session } = cookies(context);
  if (!ory_kratos_session && context.res) {
    context.res.writeHead(302, {
      Location: `/.ory/kratos/public/self-service/browser/flows/login?return_to=${context.req.url}`,
    });
    context.res.end();
  }
};
