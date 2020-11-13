import React from "react";

import { NextPage } from "next";
import { useRouter } from "next/dist/client/router";
import styled from "styled-components";

import { Footer } from "../../../../../../../components/Footer";
import { Head } from "../../../../../../../components/Head";
import { MainHeading } from "../../../../../../../components/MainHeading";
import { NavHeader } from "../../../../../../../components/NavHeader";
import { Navigation } from "../../../../../../../components/Navigation";
import { PageLayout } from "../../../../../../../components/PageLayout";
import UpdateDetailsFileForm from "../../../../../../../containers/UploadFileForm/UpdateDetailsFileForm";
import {
  useGetFileByIdQuery,
  useGetWorkspaceByIdQuery,
} from "../../../../../../../lib/generated/graphql";
import withUrqlClient from "../../../../../../../lib/withUrqlClient";

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

const UpdateFileDetails: NextPage<any> = () => {
  const router = useRouter();
  const { fileId, workspaceId, folderId } = router.query;

  if (fileId === undefined || Array.isArray(fileId)) {
    throw new Error("fileId required in URL");
  }

  if (folderId === undefined || Array.isArray(folderId)) {
    throw new Error("folderId required in URL");
  }

  if (workspaceId === undefined || Array.isArray(workspaceId)) {
    throw new Error("workspaceId required in URL");
  }

  const [workspace] = useGetWorkspaceByIdQuery({
    variables: { id: workspaceId },
  });

  const [file] = useGetFileByIdQuery({
    variables: { id: fileId },
  });

  if (workspace.error || file.error)
    return (
      <p>
        Oh no... {workspace.error?.message} {file.error?.message}{" "}
      </p>
    );

  return (
    <>
      <Head title={`Edit file details - ${file.data?.file.title}`} />
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
            <MainHeading>Edit file details</MainHeading>

            <p> Fields marked with * are mandatory.</p>
            {file.data && (
              <UpdateDetailsFileForm
                workspaceId={workspaceId}
                folderId={folderId}
                fileDescription={file.data?.file.description}
                fileId={fileId}
                fileTitle={file.data?.file.title}
                latestVersionId={file.data?.file.latestVersion}
              />
            )}
          </PageContent>
        </ContentWrapper>
        <Footer />
      </PageLayout>
    </>
  );
};

export default withUrqlClient(UpdateFileDetails);
