import React from "react";
import { GetServerSideProps } from "next";
import { generateCsrfToken } from "../../lib/login";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const request = context.query.request;
  console.log(context.req.headers.referer);

  if (!request && context.res) {
    context.res.writeHead(302, {
      Location: "/.ory/kratos/public/self-service/browser/flows/login",
    });
    context.res.end();
  }

  const csrfToken = await generateCsrfToken(context);

  return {
    props: {
      request,
      csrfToken,
    },
  };
};

const Login = ({
  request,
  csrfToken,
}: {
  request: string;
  csrfToken: string;
}) => {
  return (
    <>
      {!csrfToken && <div>Something has gone wrong, try again </div>}
      {request ? (
        <form
          action={`/.ory/kratos/public/self-service/browser/flows/login/strategies/password?request=${request}`}
          method="POST"
        >
          <input
            name="csrf_token"
            type="hidden"
            required={true}
            value={csrfToken}
          />
          <div>
            <label>Username: </label>
            <input type="text" name="identifier" id="identifier" required />
          </div>
          <div>
            <label>Password: </label>
            <input type="password" name="password" id="password" required />
          </div>
          <div>
            <input type="submit" value="Submit!" />
          </div>
        </form>
      ) : (
        <div>Nothing</div>
      )}
    </>
  );
};

export default Login;
