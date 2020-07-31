import axios from "axios";
import { GetServerSidePropsContext } from "next";

export const generateCsrfToken = async (context: GetServerSidePropsContext) => {
  const request = context.query.request;
  try {
    const res = await axios.get(
      `http://127.0.0.1:4434/self-service/browser/flows/requests/login?request=${request}`
    );
    const formattedDetails = res.data;

    const csrfToken = formattedDetails.methods.password.config.fields.find(
      (element: any) => element.name === "csrf_token"
    ).value;

    return csrfToken;
  } catch (error) {
    console.error(error.response.statusText);
    return null;
  }
};
