import { GetServerSidePropsContext } from "next";
import { ServerResponse } from "http";
import { getServerSideProps } from "../../pages/auth/login";
import { publicApi } from "../../utils/kratos";
import { LoginRequest } from "@oryd/kratos-client";

jest.mock("../../utils/kratos");

const mockedPublicApi = publicApi as jest.Mocked<typeof publicApi>;

describe("getServerSideProps", () => {
  const formFields = [
    { name: "identifier", type: "text" },
    { name: "password", type: "password" },
    {
      name: "csrf_token",
      type: "hidden",
    },
  ];

  const body: LoginRequest = {
    active: "password",
    expiresAt: new Date(),
    id: "598a45d2-e107-41ba-885a-c5c39e4a26d5",
    issuedAt: new Date(),
    messages: undefined,
    methods: {
      password: {
        config: {
          action: "http://url.com",
          fields: formFields,
          method: "post",
        },
        method: "password",
      },
    },
    requestUrl: "http://localhost:4455/self-service/browser/flows/login",
  };

  const requestValue = "test123";

  const context: GetServerSidePropsContext = ({
    query: {
      request: requestValue,
    },
    res: ({
      writeHead: jest.fn(),
      end: jest.fn(),
    } as unknown) as ServerResponse,
  } as unknown) as GetServerSidePropsContext;

  test("redirects when there is no request id", async () => {
    getServerSideProps({ ...context, query: {} });

    expect(context.res.writeHead).toHaveBeenCalledWith(302, {
      Location: "/.ory/kratos/public/self-service/browser/flows/login",
    });
    expect(context.res.end).toHaveBeenCalled();
  });

  test("with request id ", async () => {
    mockedPublicApi.getSelfServiceBrowserLoginRequest.mockResolvedValue(body);
    //TYPE ERROR: Argument of type 'LoginRequest' is not assignable to parameter of type '{ response: IncomingMessage; body: LoginRequest; } | Promise<{ response: IncomingMessage; body: LoginRequest; }>'.

    const props = await getServerSideProps(context);

    expect(props).toEqual({
      props: {
        request: requestValue,
        formFields: formFields,
      },
    });
  });
  test("throws error", async () => {
    mockedPublicApi.getSelfServiceBrowserLoginRequest.mockRejectedValue({
      response: { statusText: "something went wrong" },
    });

    const result = await getServerSideProps(context).catch((e) => e);

    expect(result).toEqual({
      response: { statusText: "something went wrong" },
    });
  });
});
