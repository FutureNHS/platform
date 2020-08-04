import axios from "axios";
import { GetServerSidePropsContext } from "next";

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
  context: GetServerSidePropsContext
): Promise<FormConfig> => {
  const request = context.query.request;
  try {
    const res = await axios.get(
      `http://kratos-admin.kratos/self-service/browser/flows/requests/login?request=${request}`
    );
    const formattedDetails = res.data;

    const config = formattedDetails.methods.password.config;

    return config;
  } catch (error) {
    console.error(error.response.statusText);
    throw error;
  }
};
