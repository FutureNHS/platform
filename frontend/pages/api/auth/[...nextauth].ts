// @ts-ignore
import NextAuth from "next-auth";

const FUSIONAUTH_PROVIDER = {
  id: "fusionauth",
  name: "FusionAuth",
  type: "oauth",
  version: "2.0",
  // fill this in once we have some roles?
  //scope: "",
  params: { grant_type: "authorization_code" },
  accessTokenUrl: "http://fusionauth-spike.fusionauth:9011/oauth2/token",
  requestTokenUrl: "http://fusionauth-spike.fusionauth:9011/oauth2/authorize",
  authorizationUrl:
    "http://fusionauth-spike.fusionauth:9011/oauth2/authorize?response_type=code",
  profileUrl: "http://fusionauth-spike.fusionauth:9011/oauth2/userinfo",
  profile: (profile: any) => {
    return {
      id: profile.sub,
      email: profile.email,
    };
  },
  clientId: "f2c83afd-bb8b-443a-99f6-e5b7763f7336",
  clientSecret: "PH0gL9kHjwIv9SDyU4Qq-adJFuEGVJtmdaQsGK39yyA",
};

const options = {
  providers: [FUSIONAUTH_PROVIDER],
  // A database is optional, but required to persist accounts in a database
  // database: process.env.DATABASE_URL,
};

export default (req: any, res: any) => NextAuth(req, res, options);
