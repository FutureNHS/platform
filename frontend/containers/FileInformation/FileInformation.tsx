import React, { FC } from "react";

import { format, parseISO } from "date-fns";
import Link from "next/link";
import styled from "styled-components";

import { EmailLink } from "../../components/EmailLink";
import { H2 } from "../../components/H2";
import { FileIcon } from "../../components/Icon";
import { MobileList } from "../../components/Table";

// Hardcoded data until backend is done.
const fileVersions = [
  {
    id: "e92f20cb-58b0-464b-bbb3-7627c4b21628",
    title: "Version 1",
    createdAt: "2020-10-11 10:53:58.696734",
    emailAddress: "email@address.com",
    fileType: "csv",
  },
  {
    id: "55f4166f-af41-4513-a2aa-3b46519fb44e",
    title: "Version 2",
    createdAt: "2020-11-11 11:54:44.376148",
    emailAddress: "correct-grammar@address.com",
    fileType: "csv",
  },
  {
    id: "55f4166f-af41-4513-a2aa-3b46519fb44e",
    title: "Version 3",
    createdAt: "2020-12-11 19:54:44.376148",
    emailAddress: "perfectionist@address.com",
    fileType: "csv",
  },
];

const FileInfoLinks = styled.div`
  padding-bottom: 40px;
`;

interface FileVersion {
  id: string;
  title: string;
  createdAt: string;
  emailAddress: string;
  fileType: string;
}

const MobileTitle = styled.h3`
  font-size: 16px;
  font-weight: normal;
  margin: 0;
  padding-bottom: 20px;
`;

interface Props {
  workspaceId: string;
}

const FileInformation = ({ workspaceId }: Props) => {
  const previousVersions = fileVersions.slice(0, fileVersions.length - 1);
  const mobileTitleCell: FC<FileVersion> = ({ title }) => (
    <MobileTitle>{title}</MobileTitle>
  );

  const IconCell: FC<FileVersion> = ({ fileType }) => (
    <FileIcon fileType={fileType} />
  );

  const mobileEmailCell: FC<FileVersion> = ({ emailAddress }) => (
    <>
      <h4>Modified by</h4>
      <EmailLink emailAddress={emailAddress} />
    </>
  );

  const mobileCreatedAtCell: FC<FileVersion> = ({ createdAt }) => (
    <div>
      <h4>Modified on</h4>
      <p>{format(parseISO(createdAt), "d LLL yyyy kk:mm")}</p>
    </div>
  );

  const mobileDownloadCell: FC<FileVersion> = ({ id }) => (
    <Link href={`/workspaces/${workspaceId}/download/${id}`} passHref>
      <a>Download file</a>
    </Link>
  );

  return (
    <>
      <H2 title="File information" />
      <h3>Contents</h3>
      <FileInfoLinks>
        <a>&#8212; {`Versions (${previousVersions.length}`})</a>
      </FileInfoLinks>

      <MobileList
        icon={IconCell}
        tableHeading="Previous versions"
        columns={[
          { content: mobileTitleCell },
          { content: mobileEmailCell },
          { content: mobileCreatedAtCell },
          { content: mobileDownloadCell },
        ]}
        data={previousVersions}
      />
    </>
  );
};

export default FileInformation;
