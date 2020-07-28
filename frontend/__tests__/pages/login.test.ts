import axios, { AxiosResponse } from "axios";
import { GetServerSidePropsContext } from "next";
import { ServerResponse } from "http";
import { getServerSideProps } from "../../pages/auth/login";

jest.mock("axios");

const mockedAxios = axios as jest.Mocked<typeof axios>;

describe("getServerSideProps", () => {
  const token = "hello123";
  const testJson = {
    expires_at: "2020-07-27T14:40:28.4155578Z",
    forced: false,
    id: "4a62445d-e238-46dd-95ca-611a2df94960",
    issued_at: "2020-07-27T14:30:28.4155723Z",
    messages: null,
    methods: {
      password: {
        config: { fields: [{ name: "csrf_token", value: token }] },
        method: "password",
      },
    },
    request_url: "http://localhost:4455/self-service/browser/flows/login",
  };
  const axiosResponse: AxiosResponse = {
    data: testJson,
    status: 200,
    statusText: "OK",
    config: {},
    headers: {},
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
    mockedAxios.get.mockResolvedValue(axiosResponse);

    const props = await getServerSideProps(context);

    expect(props).toEqual({
      props: { request: requestValue, csrfToken: token },
    });
  });
});
