import { LoginRequest } from "@oryd/kratos-client";
import { adminApi } from "../utils/kratos";

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
): Promise<LoginRequest> => {
  try {
    const res = await adminApi.getSelfServiceBrowserLoginRequest(request);

    const formattedDetails = res.body;

    console.log("******body", formattedDetails);

    // const config = formattedDetails.methods.password.config;
    // console.log("CONFIG", config);
    return formattedDetails;
  } catch (error) {
    console.error(error);
    throw error;
  }
};
