import React from "react";

import { GraphQLClient } from "graphql-request";
import { GetServerSideProps } from "next";
import styled from "styled-components";

import { Head } from "../../components/Head";
import { Header } from "../../components/Header";
import { MainHeading } from "../../components/MainHeading";
import { PageLayout } from "../../components/PageLayout";
import { requireAuthentication } from "../../lib/auth";
import { getSdk } from "../../lib/generated/graphql";

export const getServerSideProps: GetServerSideProps<Props> = requireAuthentication(
  async (context) => {
    const client = new GraphQLClient(
      "http://workspace-service.workspace-service/graphql"
    );
    const sdk = getSdk(client);
    const workspaceID = (context.params?.id as string) || "";

    const { workspace } = await sdk.GetWorkspaceByID({ id: workspaceID });

    return {
      props: {
        workspace,
      },
    };
  }
);

const PageContent = styled.section`
  min-height: 100vh;
  padding-top: 24px;
  padding-left: 10%;
  padding-right: 10%;
  ${({ theme }) => `
  background-color: ${theme.colorNhsukWhite};
  `}
`;

const H2 = styled.h2`
  padding-top: 24px;
  margin-bottom: 8px;
  ${({ theme }) => `
  border-top: 1px solid ${theme.colorNhsukGrey1};
  color: ${theme.colorNhsukGrey1}
  `}
`;

interface Props {
  workspace: {
    title: string;
  };
}

const WorkspaceHomepage = ({ workspace }: Props) => (
  <>
    <Head title={workspace.title} />
    <PageLayout>
      <Header />
      <PageContent>
        <MainHeading>{workspace.title}</MainHeading>
        <H2>Most recent items</H2>
      </PageContent>
    </PageLayout>
  </>
);

export default WorkspaceHomepage;
