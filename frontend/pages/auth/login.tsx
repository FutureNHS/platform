import React from "react";
import { GetServerSideProps } from "next";
import { getLoginFields } from "../../lib/auth";
import { sendEvent } from "../../lib/events";
import { LoginRequestMethodConfig } from "@oryd/kratos-client";
import { redirect } from "../../utils/pages/redirect";
import { PageLayout } from "../../components/PageLayout";
import { Header } from "../../components/Header";

type LoginProps = {
  request: string;
  formConfig: LoginRequestMethodConfig;
};

export const getServerSideProps: GetServerSideProps = async (
  context
): Promise<{ props: LoginProps }> => {
  const request = context.query.request;

  if (!request || Array.isArray(request)) {
    return redirect(
      context,
      "/.ory/kratos/public/self-service/browser/flows/login"
    );
  }

  const formattedDetails = await getLoginFields(request);
  const formConfig = formattedDetails.methods.password.config;
  const messages = formConfig.messages?.map((msg) => msg.text ?? "") ?? null;

  // TODO: This is just an example event. We need to figure out the schema for custom events and change this to events we really need.
  await sendEvent({
    subject: "frontend",
    eventType: "frontend.login.attempt",
    data: { messages },
    dataVersion: "1",
  });

  return {
    props: {
      request,
      formConfig,
    },
  };
};

const Login = ({ request, formConfig }: LoginProps) => {
  return (
    <PageLayout>
      <Header
        imageRight="NHS.png"
        imageRightURL="https://www.nhs.co.uk"
        imageRightAltText="NHS logo"
      />
      <div>
        {formConfig.messages?.map(({ text }) => {
          return (
            <>
              <div>{text}</div>
            </>
          );
        })}
        {request && (
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
                      defaultValue={(value as unknown) as string}
                    />
                  </div>
                </>
              );
            })}
            <div>
              <input type="submit" value="Submit!" />
            </div>
          </form>
        )}
      </div>
    </PageLayout>
  );
};

export default Login;
