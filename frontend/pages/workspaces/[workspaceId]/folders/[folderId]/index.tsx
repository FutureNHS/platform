import React from "react";

import { NextPage } from "next";
import { useRouter } from "next/router";
import styled from "styled-components";

import { MobileFileList, FileTable } from "../../../../../components/FileTable";
import { Head } from "../../../../../components/Head";
import {
  DeleteIcon,
  EditIcon,
  LockIcon,
  MoveIcon,
  UploadIcon,
} from "../../../../../components/Icon";
import { MainHeading } from "../../../../../components/MainHeading";
import { Menu, MenuItem } from "../../../../../components/Menu";
import { NavHeader } from "../../../../../components/NavHeader";
import { Navigation } from "../../../../../components/Navigation";
import { PageLayout } from "../../../../../components/PageLayout";
import {
  useGetFolderByIdQuery,
  useGetWorkspaceByIdQuery,
  useFilesByFolderQuery,
} from "../../../../../lib/generated/graphql";
import withUrqlClient from "../../../../../lib/withUrqlClient";

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

const FolderHomepage: NextPage = () => {
  const router = useRouter();
  let { workspaceId, folderId } = router.query;
  workspaceId = (workspaceId || "unknown").toString();
  folderId = (folderId || "unknown").toString();

  const [workspace] = useGetWorkspaceByIdQuery({
    variables: { id: workspaceId },
  });
  const [folder] = useGetFolderByIdQuery({
    variables: { id: folderId },
  });

  const [files] = useFilesByFolderQuery({
    variables: { folder: folderId },
  });

  const items: MenuItem[] = [
    {
      title: "Upload file to this folder",
      icon: <UploadIcon />,
      handler: `/workspaces/${workspaceId}/folders/${folderId}/upload-file`,
    },
    {
      title: "Edit folder details",
      icon: <EditIcon />,
      handler: "#",
    },
    {
      title: "Move folder",
      icon: <MoveIcon />,

      handler: "#",
    },
    {
      title: "View folder permissions",
      icon: <LockIcon />,

      handler: "#",
    },
    {
      title: "Delete folder",
      icon: <DeleteIcon />,
      handler: "#",
    },
  ];

  return (
    <>
      <Head
        title={
          folder.fetching
            ? "Loading..."
            : `Folder - ${folder.data?.folder.title}` || "No title!"
        }
      />
      <PageLayout>
        <NavHeader />
        <ContentWrapper>
          <Navigation
            workspaceId={workspaceId}
            workspaceTitle={workspace.data?.workspace.title || "unknown"}
            activeFolder={folderId}
          />
          <PageContent>
            <MainHeading withBorder menu={<Menu items={items} />}>
              {folder.data?.folder.title || ""}
            </MainHeading>
            <p>{folder.data?.folder.description}</p>
            {folder.error && <p> Oh no... {folder.error?.message} </p>}
            {files.error && <p> Oh no... {files.error?.message} </p>}
            {files.fetching || (!files.data && <p>Loading...</p>)}
            {files.data && files.data.filesByFolder.length > 0 && (
              <>
                <MobileFileList
                  files={files.data.filesByFolder}
                  workspaceId={workspaceId}
                  titleLink={true}
                  tableHeading="Files"
                ></MobileFileList>
                <FileTable
                  files={files.data.filesByFolder}
                  workspaceId={workspaceId}
                  titleLink={true}
                  tableHeading="Files"
                />
              </>
            )}
          </PageContent>
        </ContentWrapper>
      </PageLayout>
    </>
  );
};

export default withUrqlClient(FolderHomepage);
