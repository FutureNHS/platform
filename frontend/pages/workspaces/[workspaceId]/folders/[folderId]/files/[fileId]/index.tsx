import React, { FC } from "react";

import Link from "next/link";
import { useRouter } from "next/router";
import styled from "styled-components";

import {
  IconCell,
  MobileModifiedAtCell,
  ModifiedAtCell,
  TitleCell,
} from "../../../../../../../components/Files";
import { Footer } from "../../../../../../../components/Footer";
import { Head } from "../../../../../../../components/Head";
import {
  DeleteIcon,
  EditIcon,
  UploadIcon,
} from "../../../../../../../components/Icon";
import { MainHeading } from "../../../../../../../components/MainHeading";
import { Menu } from "../../../../../../../components/Menu";
import { NavHeader } from "../../../../../../../components/NavHeader";
import { Navigation } from "../../../../../../../components/Navigation";
import { PageLayout } from "../../../../../../../components/PageLayout";
import { MobileList, Table } from "../../../../../../../components/Table";
import { FileInformation } from "../../../../../../../containers/FileInformation";
import {
  File,
  useDeleteFileMutation,
  useGetFileByIdQuery,
  useGetWorkspaceByIdQuery,
} from "../../../../../../../lib/generated/graphql";
import withUrqlClient from "../../../../../../../lib/withUrqlClient";

const PageContent = styled.section`
  flex-grow: 3;
  min-height: 100vh;
  padding-top: 24px;
  padding-left: 16px;
  padding-right: 16px;
  ${({ theme }) => `
    background-color: ${theme.colorNhsukWhite};
    @media (min-width: ${theme.mqBreakpoints.tablet}) {
      padding-left: 20px;
      padding-right: 20px;
    }
    @media (min-width: ${theme.mqBreakpoints.largeDesktop}) {
      padding-left: 32px;
      padding-right: 32px;
    }
  `}
`;

const ContentWrapper = styled.div`
  display: flex;
`;

const Description = styled.p`
  padding-bottom: 40px;
`;

const DownloadFile = styled.a`
  display: inline-block;
  padding-right: 8px;
  font-size: 16px;
`;

const MobileTitle = styled.h3`
  font-size: 16px;
  font-weight: normal;
  margin: 0;
  padding-bottom: 20px;
`;

const FileHomepage = () => {
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

  const [, deleteFile] = useDeleteFileMutation();

  const onClick = async () => {
    const message = "Are you sure you want to delete this file?";
    const result = window.confirm(message);
    if (result) {
      await deleteFile({ id: fileId });
      await router.push(`/workspaces/${workspaceId}/folders/${folderId}`);
    }
  };

  const actionsCell: FC<File> = ({ id }) => (
    <Link href={`/workspaces/${workspaceId}/download/${id}`} passHref>
      <DownloadFile>Download file</DownloadFile>
    </Link>
  );

  const mobileActionsCell: FC<File> = ({ id }) => (
    <Link href={`/workspaces/${workspaceId}/download/${id}`} passHref>
      <a>Download file</a>
    </Link>
  );

  const mobileTitleCell: FC<File> = ({ id, title }) => (
    <MobileTitle>
      <Link href={`/workspaces/${workspaceId}/folders/${folderId}/files/${id}`}>
        <a>{title}</a>
      </Link>
    </MobileTitle>
  );

  return (
    <>
      <Head title={`File - ${file.data?.file.title || "Loading..."}`} />
      <PageLayout>
        <NavHeader />
        <ContentWrapper>
          <Navigation
            workspaceId={workspaceId}
            workspaceTitle={workspace.data?.workspace.title || "unknown"}
            activeFolder={folderId}
          />
          <PageContent>
            <MainHeading
              menu={
                <Menu
                  background="light"
                  dataCy="file-options"
                  items={[
                    {
                      title: "Edit file details",
                      icon: <EditIcon />,
                      handler: `/workspaces/${workspaceId}/folders/${folderId}/files/${fileId}/update-file-details`,
                      dataCy: "edit-file-details",
                    },
                    {
                      title: "Upload new version",
                      icon: <UploadIcon />,
                      handler: `/workspaces/${workspaceId}/folders/${folderId}/files/${fileId}/update-file`,
                      dataCy: "update-file",
                    },
                    {
                      title: "Delete file",
                      icon: <DeleteIcon />,
                      handler: onClick,
                      dataCy: "delete-file",
                    },
                  ]}
                />
              }
            >
              {file.data?.file.title || "Loading..."}
            </MainHeading>
            <Description>
              {file.data?.file.description ?? "Loading..."}
            </Description>
            {file.error && <p> Oh no... {file.error?.message} </p>}
            {file.fetching || !file.data ? (
              "Loading..."
            ) : (
              <>
                <MobileList
                  icon={IconCell}
                  columns={[
                    { content: mobileTitleCell },
                    { content: MobileModifiedAtCell },
                    { content: mobileActionsCell },
                  ]}
                  data={[file.data.file as File]}
                />
                <Table
                  icon={IconCell}
                  columns={[
                    { heading: "Title", content: TitleCell },
                    { heading: "Last modified", content: ModifiedAtCell },
                    { heading: "Actions", content: actionsCell },
                  ]}
                  data={[file.data.file as File]}
                />
              </>
            )}
            <FileInformation workspaceId={workspaceId} />
          </PageContent>
        </ContentWrapper>
        <Footer />
      </PageLayout>
    </>
  );
};

export default withUrqlClient(FileHomepage);
