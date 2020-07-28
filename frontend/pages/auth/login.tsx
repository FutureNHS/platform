import React from "react";
import { GetServerSideProps } from "next";
import axios from "axios";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const request = context.query.request;

  if (!request && context.res) {
    context.res.writeHead(302, {
      Location: "/.ory/kratos/public/self-service/browser/flows/login",
    });
    context.res.end();
  }

  const res = await axios.get(
    `http://127.0.0.1:4434/self-service/browser/flows/requests/login?request=${request}`
  );
  const formattedDetails = res.data;

  const csrfToken = formattedDetails.methods.password.config.fields.find(
    (element: any) => element.name === "csrf_token"
  ).value;

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
