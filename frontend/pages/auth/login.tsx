import React from "react";
import { GetServerSideProps } from "next";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const request = context.query.request;

  if (!request && context.res) {
    context.res.writeHead(301, {
      Location: "/.ory/kratos/public/self-service/browser/flows/login",
    });
    context.res.end();
  }

  return {
    props: {
      loggedIn: !!request,
    },
  };
};
const Login = ({ loggedIn }: { loggedIn: boolean }) => {
  return (
    <>
      {loggedIn ? (
        <form action="" method="get">
          <div>
            <label>Username: </label>
            <input type="text" name="username" id="username" required />
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
