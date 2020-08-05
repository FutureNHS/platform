import { AdminApi, LoginRequestMethodConfig } from "@oryd/kratos-client";

export type FormConfig = {
  action: string;
  method: "GET" | "POST";
  fields: Array<{
    name: string;
    type: string;
    required: boolean;
    value?: string;
  }>;

  messages?: Array<{
    id: number;
    text: string;
    type: string;
    context: {};
  }>;
};

export const generateFields = async (
  request: string
): Promise<LoginRequestMethodConfig> => {
  try {
    const api = new AdminApi("http://kratos-admin.kratos");
    const res = await api.getSelfServiceLoginRequest(request);

    const formattedDetails = res.body;

    const config = formattedDetails.methods.password.config;

    return config;
  } catch (error) {
    console.error(error.response.statusText);
    throw error;
  }
};
