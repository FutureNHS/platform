import React from "react";

import { NextPage } from "next";
import { useRouter } from "next/dist/client/router";
import styled from "styled-components";
import { Client } from "urql";

import { MainHeading } from "../../../../../components/MainHeading";
import { NavHeader } from "../../../../../components/NavHeader";
import { Navigation } from "../../../../../components/Navigation";
import { PageLayout } from "../../../../../components/PageLayout";
import { UploadFileForm } from "../../../../../containers/UploadFileForm";
import {
  useGetFolderByIdQuery,
  useGetWorkspaceByIdQuery,
} from "../../../../../lib/generated/graphql";
import withUrqlClient from "../../../../../lib/withUrqlClient";

const ContentWrapper = styled.div`
  display: flex;
`;

const PageContent = styled.div`
  ${({ theme }) => `
  background-color: ${theme.colorNhsukWhite};
  min-height: 100vh;
  padding-top: 24px;
  padding-left: 10%;
  padding-right: 10%;
  .nhsuk-form-group {
    margin-bottom: 8px;
  }
  `}
`;

const UploadFile: NextPage<any> = ({ urqlClient }: { urqlClient: Client }) => {
  const router = useRouter();
  const workspaceId = (router.query.workspaceId || "unknown").toString();
  const folderId = (router.query.folderId || "unknown").toString();

  const [workspace] = useGetWorkspaceByIdQuery({
    variables: { id: workspaceId },
  });
  const [folder] = useGetFolderByIdQuery({
    variables: { id: folderId },
  });

  if (workspace.error || folder.error)
    return (
      <p>
        {" "}
        Oh no... {workspace.error?.message} {folder.error?.message}{" "}
      </p>
    );

  return (
    <PageLayout>
      <NavHeader />
      <ContentWrapper>
        <Navigation
          workspaceId={workspaceId}
          workspaceTitle={
            workspace.fetching
              ? "Loading..."
              : workspace.data?.workspace.title || "No title!"
          }
          activeFolder={folderId}
        />
        <PageContent>
          <MainHeading withBorder>
            {folder.fetching
              ? "Loading..."
              : folder.data?.folder.title || "No title!"}
          </MainHeading>
          <p> Fields marked with * are mandatory.</p>
          <UploadFileForm
            urqlClient={urqlClient}
            workspaceId={workspaceId}
            folderId={folderId}
          />
        </PageContent>
      </ContentWrapper>
    </PageLayout>
  );
};

export default withUrqlClient(UploadFile);
