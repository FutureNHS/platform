import React from "react";

import { GraphQLClient } from "graphql-request";
import { GetServerSideProps } from "next";
import styled from "styled-components";

import { Head } from "../../components/Head";
import { Header } from "../../components/Header";
import { MainHeading } from "../../components/MainHeading";
import { Navigation } from "../../components/Navigation";
import { PageLayout } from "../../components/PageLayout";
import { requireAuthentication } from "../../lib/auth";
import { getSdk, Folder } from "../../lib/generated/graphql";

export const getServerSideProps: GetServerSideProps<Props> = requireAuthentication(
  async (context) => {
    const client = new GraphQLClient(
      "http://workspace-service.workspace-service/graphql"
    );
    const sdk = getSdk(client);
    const workspaceID = (context.params?.workspaceId as string) || "";

    const { workspace } = await sdk.GetWorkspaceByID({ id: workspaceID });
    const { foldersByWorkspace } = await sdk.FoldersByWorkspace({
      workspace: workspaceID,
    });

    return {
      props: {
        workspace,
        folders: foldersByWorkspace,
      },
    };
  }
);

const PageContent = styled.section`
  flex-grow: 3;
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
    id: string;
  };
  folders: Array<Pick<Folder, "title" | "id">>;
}

const ContentWrapper = styled.div`
  display: flex;
`;
const WorkspaceHomepage = ({ workspace, folders }: Props) => (
  <>
    <Head title={workspace.title} />
    <PageLayout>
      <Header />
      <ContentWrapper>
        <Navigation workspace={workspace} folders={folders} />
        <PageContent>
          <MainHeading>{workspace.title}</MainHeading>
          <H2>Most recent items</H2>
        </PageContent>
      </ContentWrapper>
    </PageLayout>
  </>
);

export default WorkspaceHomepage;
