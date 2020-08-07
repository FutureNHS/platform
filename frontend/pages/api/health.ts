import { NextApiRequest, NextApiResponse } from "next";

const requiredEnvVars = ["EVENTGRID_TOPIC_ENDPOINT", "EVENTGRID_TOPIC_KEY"];

export default (_: NextApiRequest, res: NextApiResponse) => {
  const missingEnvVars = requiredEnvVars.filter((name) => !process.env[name]);
  if (missingEnvVars.length > 0) {
    res.status(500).json({
      status: "Error",
      message: `Missing environment variables: ${missingEnvVars.join(", ")}`,
    });
  } else {
    res.status(200).json({ status: "Healthy" });
  }
};
