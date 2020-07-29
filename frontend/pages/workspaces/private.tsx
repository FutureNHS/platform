import React from "react";
import { GetServerSideProps } from "next";
import cookies from "next-cookies";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const { ory_kratos_session } = cookies(context);
  if (!ory_kratos_session && context.res) {
    context.res.writeHead(302, {
      Location:
        "/.ory/kratos/public/self-service/browser/flows/login?return_to=/workspaces/private",
    });
    context.res.end();
  }

  return {
    props: {
      ory_kratos_session,
    },
  };
};

const PrivatePage = ({
  ory_kratos_session,
}: {
  ory_kratos_session: string;
}) => {
  return (
    <>
      <div>Private Page</div>
      <div>{ory_kratos_session}</div>
    </>
  );
};

export default PrivatePage;
