import React from "react";
import { GetServerSideProps } from "next";
import { generateFields, FormConfig } from "../../lib/login";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const request = context.query.request;

  if (!request && context.res) {
    context.res.writeHead(302, {
      Location: "/.ory/kratos/public/self-service/browser/flows/login",
    });
    context.res.end();
  }
  if (!request || Array.isArray(request)) {
    return { props: {} };
  }

  const formattedDetails = await generateFields(request);
  const formConfig = formattedDetails.methods.password.config;

  return {
    props: {
      request,
      formConfig,
    },
  };
};

const Login = ({
  request,
  formConfig,
}: {
  request: string;
  formConfig: FormConfig;
}) => {
  return (
    <>
      {formConfig.messages?.map(({ text }) => {
        return (
          <>
            <div>{text}</div>
          </>
        );
      })}
      {request ? (
        <form action={formConfig.action} method={formConfig.method}>
          {formConfig.fields.map(({ name, type, required, value }) => {
            return (
              <>
                <div>
                  {type !== "hidden" ? (
                    <label htmlFor={name}>{name}</label>
                  ) : null}
                  <input
                    id={name}
                    name={name}
                    type={type}
                    required={required}
                    defaultValue={value}
                  />
                </div>
              </>
            );
          })}
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
