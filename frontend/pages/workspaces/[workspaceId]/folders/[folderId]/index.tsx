import React, { FC } from "react";

import { NextPage } from "next";
import Link from "next/link";
import { useRouter } from "next/router";
import styled from "styled-components";

import { Error as ErrorComponent } from "../../../../../components/Error";
import {
  IconCell,
  MobileModifiedAtCell,
  ModifiedAtCell,
} from "../../../../../components/Files";
import { Footer } from "../../../../../components/Footer";
import { Head } from "../../../../../components/Head";
import {
  DeleteIcon,
  EditIcon,
  MoveIcon,
  UploadIcon,
} from "../../../../../components/Icon";
import { MainHeading } from "../../../../../components/MainHeading";
import { Menu, MenuItem } from "../../../../../components/Menu";
import { NavHeader } from "../../../../../components/NavHeader";
import { Navigation } from "../../../../../components/Navigation";
import { PageLayout } from "../../../../../components/PageLayout";
import { MobileList, Table } from "../../../../../components/Table";
import {
  File,
  useFilesByFolderQuery,
  useGetFolderByIdQuery,
  useGetWorkspaceByIdQuery,
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

const MobileTitle = styled.h3`
  font-size: 16px;
  font-weight: normal;
  margin: 0;
  padding-bottom: 20px;
`;

const DownloadFile = styled.a`
  display: inline-block;
  padding-right: 8px;
  font-size: 16px;
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

  const accessPermitted =
    folder.error?.graphQLErrors[0]?.extensions?.details === "ACCESS_DENIED"
      ? false
      : true;

  const items: MenuItem[] = [
    {
      title: "Upload files to this folder",
      icon: <UploadIcon />,
      handler: `/workspaces/${workspaceId}/folders/${folderId}/upload-file`,
    },
    {
      title: "Edit folder details",
      icon: <EditIcon />,
      handler: `/workspaces/${workspaceId}/folders/${folderId}/update-folder`,
    },
    {
      title: "Move folder",
      icon: <MoveIcon />,
      handler: "#",
    },
    {
      title: "Delete folder",
      icon: <DeleteIcon />,
      handler: "#",
    },
  ];

  const titleCell: FC<File> = ({ id, title }) => (
    <Link
      href={`/workspaces/${workspaceId}/folders/${folderId}/files/${id}`}
      passHref
    >
      <a>
        <span>{title}</span>
      </a>
    </Link>
  );

  const downloadCell: FC<File> = ({ id }) => (
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
            {accessPermitted ? (
              <>
                <MainHeading
                  menu={
                    <Menu
                      background="light"
                      items={items}
                      dataCy="folder-options"
                    />
                  }
                >
                  {folder.data?.folder.title || ""}
                </MainHeading>
                <p>{folder.data?.folder.description}</p>
                {folder.error && <p> Oh no... {folder.error?.message} </p>}
                {files.error && <p> Oh no... {files.error?.message} </p>}
                {files.fetching || (!files.data && <p>Loading...</p>)}
                {files.data && files.data.filesByFolder.length > 0 && (
                  <>
                    <MobileList
                      tableHeading="Files"
                      icon={IconCell}
                      columns={[
                        { content: mobileTitleCell },
                        { content: MobileModifiedAtCell },
                        { content: mobileActionsCell },
                      ]}
                      data={files.data.filesByFolder as File[]}
                    />
                    <Table
                      tableHeading="Files"
                      icon={IconCell}
                      columns={[
                        { heading: "Title", content: titleCell },
                        { heading: "Last modified", content: ModifiedAtCell },
                        { heading: "Actions", content: downloadCell },
                      ]}
                      data={files.data.filesByFolder as File[]}
                    />
                  </>
                )}
              </>
            ) : (
              <>
                <ErrorComponent
                  title="You do not have permission to do this."
                  description="Please contact a Workspace Administrator to request access
                to this folder."
                />
              </>
            )}
          </PageContent>
        </ContentWrapper>
        <Footer />
      </PageLayout>
    </>
  );
};

export default withUrqlClient(FolderHomepage);
