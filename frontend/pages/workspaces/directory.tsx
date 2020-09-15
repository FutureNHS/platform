import React from "react";

import { GraphQLClient } from "graphql-request";
import { GetServerSideProps } from "next";
import styled from "styled-components";

import { Header } from "../../components/Header";
import { MainHeading } from "../../components/MainHeading";
import { PageLayout } from "../../components/PageLayout";
import WorkspaceDirectoryItem from "../../components/Workspaces/WorkspaceDirectoryItem";
import { requireAuthentication } from "../../lib/auth";
import { getSdk } from "../../lib/generated/graphql";

export const getServerSideProps: GetServerSideProps = requireAuthentication(
  async () => {
    const client = new GraphQLClient(
      "http://workspace-service.workspace-service/graphql"
    );

    const sdk = getSdk(client);

    const { workspaces } = await sdk.GetWorkspaces();

    return {
      props: {
        workspaces,
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

interface Props {
  workspaces: [{ title: string; id: string }];
}

const WorkspaceDirectory = ({ workspaces }: Props) => {
  return (
    <PageLayout>
      <Header />
      <PageContent>
        <MainHeading withBorder>My workspaces</MainHeading>
        {workspaces.map((workspace) => {
          return (
            <>
              <WorkspaceDirectoryItem
                title={workspace.title}
                id={workspace.id}
              />
            </>
          );
        })}
      </PageContent>
    </PageLayout>
  );
};

export default WorkspaceDirectory;
