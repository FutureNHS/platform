import React, { FC } from "react";

import { GraphQLClient } from "graphql-request";
import { GetServerSideProps } from "next";
import styled from "styled-components";

import { Head } from "../../../../components/Head";
import { Header } from "../../../../components/Header";
import { MainHeading } from "../../../../components/MainHeading";
import { Navigation } from "../../../../components/Navigation";
import { PageLayout } from "../../../../components/PageLayout";
import { requireAuthentication } from "../../../../lib/auth";
import { getSdk, Folder, Workspace } from "../../../../lib/generated/graphql";

export const getServerSideProps: GetServerSideProps<Props> = requireAuthentication(
  async (context) => {
    const client = new GraphQLClient(
      "http://workspace-service.workspace-service/graphql"
    );
    const sdk = getSdk(client);
    const folderId = (context.params?.folderId as string) || "";
    const workspaceId = (context.params?.workspaceId as string) || "";

    const { folder } = await sdk.GetFolderById({ id: folderId });
    const { foldersByWorkspace } = await sdk.FoldersByWorkspace({
      workspace: workspaceId,
    });
    const { workspace } = await sdk.GetWorkspaceByID({ id: workspaceId });

    return {
      props: {
        workspaceFolders: foldersByWorkspace,
        folder,
        workspace,
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
const ContentWrapper = styled.div`
  display: flex;
`;

interface Props {
  folder: Folder;
  workspaceFolders: Array<Pick<Folder, "id" | "title">>;
  workspace: Pick<Workspace, "id" | "title">;
}

const FolderHomepage: FC<Props> = ({ folder, workspaceFolders, workspace }) => (
  <>
    <Head title={folder.title} />
    <PageLayout>
      <Header />
      <ContentWrapper>
        <Navigation
          folders={workspaceFolders}
          workspace={workspace}
          activeFolder={folder.id}
        />
        <PageContent>
          <MainHeading withBorder>{folder.title}</MainHeading>
          <p>{folder.description}</p>
        </PageContent>
      </ContentWrapper>
    </PageLayout>
  </>
);

export default FolderHomepage;
