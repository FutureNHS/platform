import React from "react";
import { GetServerSideProps } from "next";
import { generateFields, FormConfig } from "../../lib/login";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const request = context.query.request;
  console.log(context.req.headers.referer);

  if (!request && context.res) {
    context.res.writeHead(302, {
      Location: "/.ory/kratos/public/self-service/browser/flows/login",
    });
    context.res.end();
  }

  const formFields = await generateFields(context);
  console.log(JSON.stringify(formFields, null, 4));
  return {
    props: {
      request,
      formFields,
    },
  };
};

const Login = ({
  request,
  formFields,
}: {
  request: string;
  formFields: FormConfig;
}) => {
  return (
    <>
      {formFields.messages?.map(({ text }) => {
        return (
          <>
            <div>{text}</div>
          </>
        );
      })}
      {request ? (
        <form
          action={`/.ory/kratos/public/self-service/browser/flows/login/strategies/password?request=${request}`}
          method="POST"
        >
          {formFields.fields.map(({ name, type, required, value }) => {
            return (
              <>
                {/* do proper label */}
                <div>
                  {type !== "hidden" ? <label>{name}</label> : null}
                  <input
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
