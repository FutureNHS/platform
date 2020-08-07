import React from "react";
import { GetServerSideProps } from "next";
import { generateFields, FormConfig } from "../../lib/login";
import { sendEvent } from "../../lib/events";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const request = context.query.request;

  if (!request && context.res) {
    context.res.writeHead(302, {
      Location: "/.ory/kratos/public/self-service/browser/flows/login",
    });
    context.res.end();
    return { props: {} };
  }
  if (!request || Array.isArray(request)) {
    return { props: {} };
  }

  const formattedDetails = await generateFields(request);
  const formConfig = formattedDetails.methods.password.config;

  // TODO: This is just an example event. We need to figure out the schema for custom events and change this to events we really need.
  await sendEvent({
    subject: "frontend",
    eventType: "frontend.login.attempt",
    data: { messages: formConfig.messages?.map((msg) => msg.text ?? "") },
    dataVersion: "1",
  });

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
