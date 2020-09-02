import React from "react";
import { PageLayout } from "../../components/PageLayout";
import { Header } from "../../components/Header";
import { Login } from "../../components/Login";
import { GetServerSideProps } from "next";

export const getServerSideProps: GetServerSideProps = async (context) => {
  context.res.setHeader(
    "Access-Control-Allow-Origin",
    "https://futurenhsplatform.b2clogin.com"
  );
  context.res.setHeader("Access-Control-Allow-Methods", "GET");
  return { props: {} };
};

const LoginPage = () => {
  return (
    <PageLayout>
      <Header
        imageRight={require("../../public/NHS.png")}
        imageRightURL="https://www.nhs.co.uk"
        imageRightAltText="NHS logo"
      />
      <Login />
    </PageLayout>
  );
};

export default LoginPage;
