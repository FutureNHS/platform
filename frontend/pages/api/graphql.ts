import { ApolloGateway } from "@apollo/gateway";
import { ApolloServer } from "apollo-server-micro";
import { NextApiRequest, NextApiResponse } from "next";

import { User } from "../../lib/auth";

const gateway = new ApolloGateway({
  serviceList: [
    {
      name: "workspace-service",
      url: "http://workspace-service.workspace-service/graphql",
    },
  ],
});

const server = new ApolloServer({
  gateway,
  subscriptions: false,
  playground: {
    settings: {
      "request.credentials": "include",
    },
  },
});

export const config = {
  api: {
    bodyParser: false,
    cors: false,
  },
};

const handler = server.createHandler({ path: "/api/graphql" });
interface NextApiRequestWithUser extends NextApiRequest {
  user?: User;
}
export default (req: NextApiRequestWithUser, res: NextApiResponse) =>
  req.user ? handler(req, res) : res.status(401).end();
