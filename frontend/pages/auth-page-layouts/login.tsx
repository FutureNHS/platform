import React from "react";
import { PageLayout } from "../../components/PageLayout";
import { Header } from "../../components/Header";
import { Login } from "../../components/Login";
import { GetServerSideProps } from "next";
import { Gradient } from "../../components/Login";
import { LoginPanel } from "../../components/Login";
import styled from "styled-components";

export const getServerSideProps: GetServerSideProps = async (context) => {
  context.res.setHeader(
    "Access-Control-Allow-Origin",
    "https://futurenhsplatform.b2clogin.com"
  );
  context.res.setHeader("Access-Control-Allow-Methods", "GET");
  return { props: {} };
};

export const StyledPageWrapper = styled.div`
  ${({ theme }) => `
    min-height: 100vh;
    background-color: ${theme.colorNhsukWhite};
  `}
`;

const LoginPage = () => {
  return (
    <StyledPageWrapper>
      <PageLayout>
        <Header
          imageRight={require("../../public/NHS.png")}
          imageRightURL="https://www.nhs.co.uk"
          imageRightAltText="NHS logo"
        />
        <Gradient>
          <Login />
          <LoginPanel
            label="Welcome to FutureNHS"
            text="FutureNHS connects people and helps build relationships across the health and social care sector."
          />
        </Gradient>
      </PageLayout>
    </StyledPageWrapper>
  );
};

export default LoginPage;
