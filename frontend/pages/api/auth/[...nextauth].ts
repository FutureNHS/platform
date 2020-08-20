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

const AAD_B2C_PROVIDER = {
  id: "aad",
  name: "AAD",
  type: "oauth",
  version: "2.0",
  // fill this in once we have some roles?
  scope: "openid",
  params: { grant_type: "authorization_code" },
  accessTokenUrl:
    "https://futurenhsplatform.b2clogin.com/futurenhsplatform.onmicrosoft.com/B2C_1_signin/oauth2/v2.0/token",
  authorizationUrl:
    "https://futurenhsplatform.b2clogin.com/futurenhsplatform.onmicrosoft.com/oauth2/v2.0/authorize?p=B2C_1_signin&nonce=defaultNonce&response_type=code",
  profile: (profile: any) => {
    return {
      id: profile.sub,
      name: profile.name,
      email: profile.email,
    };
  },
  idToken: true,
  clientId: "3d007909-ddef-4c9d-9e2a-cf4e6b4b8753",
  clientSecret: process.env.AAD_CLIENT_SECRET,
};

const options = {
  providers: [FUSIONAUTH_PROVIDER, AAD_B2C_PROVIDER],
  // A database is optional, but required to persist accounts in a database
  // database: process.env.DATABASE_URL,
  //debug: true
};

export default (req: any, res: any) => NextAuth(req, res, options);
