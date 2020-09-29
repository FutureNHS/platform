import React, { FC } from "react";

import Link from "next/link";
import styled from "styled-components";

import { Folder, Workspace } from "../../lib/generated/graphql";
import { NavListItem } from "../NavListItem";
import { NavSection } from "../NavSection";

const Nav = styled.nav`
  padding-top: 24px;
  padding-right: 52px;
  padding-left: 20px;
  flex-shrink: 0;
  flex-grow: 0;
  box-sizing: border-box;
  width: 270px;
  ${({ theme }) => `
  @media (min-width: ${theme.mqBreakpoints.tablet}) {
      width: 300px;
    }

  @media (min-width: ${theme.mqBreakpoints.largeDesktop}) {
      width: 360px;
    }
  `}
`;

const Header = styled.header`
  padding-bottom: 20px;
  ${({ theme }) => `
  border-bottom: 1px solid ${theme.colorNhsukGrey1};
  `};
`;

const WorkspaceTitleLink = styled.a`
  text-decoration: none;
  color: inherit;
  &:hover {
    color: inherit;
  }
  &:visited {
    color: inherit;
  }
`;

const List = styled.ul`
  padding-left: 0px;
  .nav-list-item {
    div {
      font-weight: 700;
      ${({ theme }) => `
      color: ${theme.colorNhsukBlue}`}
    }
  }
`;

interface Props {
  workspace: Pick<Workspace, "id" | "title">;
  folders: Array<Pick<Folder, "title" | "id">>;
  activeFolder?: string;
}

const Navigation: FC<Props> = ({ workspace, folders, activeFolder }) => {
  const createFolder = {
    id: "create-folder",
    title: "Create new folder",
    description: "create folder",
    workspace: workspace.id,
  };

  const icons: { [key: string]: string } = {
    closed: require("../../public/folderClosed.svg"),
    open: require("../../public/folderOpen.svg"),
  };

  const alphabetisedFolders = folders.sort((a, b) =>
    a.title.localeCompare(b.title, "en", { sensitivity: "base" })
  );

  return (
    <Nav>
      <Header>
        <Link href={`/workspaces/${workspace.id}`} passHref>
          <WorkspaceTitleLink>
            <h3>{workspace.title}</h3>
          </WorkspaceTitleLink>
        </Link>

        <Link href={`/workspaces/${workspace.id}`}>
          <a>About this workspace</a>
        </Link>
      </Header>
      <NavSection title="Folders">
        <List>
          <NavListItem
            active={createFolder.id === activeFolder}
            item={createFolder}
            imgSrc={require("../../public/createFolder.svg")}
            className="nav-list-item"
            href={`/workspaces/${workspace.id}/folders/${createFolder.id}`}
            altText=""
          />
          {alphabetisedFolders.map((folder) => (
            <NavListItem
              active={folder.id === activeFolder}
              key={folder.id}
              item={folder}
              imgSrc={
                folder.id === activeFolder ? icons["open"] : icons["closed"]
              }
              altText={
                folder.id === activeFolder ? "folder current page" : "folder"
              }
              href={`/workspaces/${workspace.id}/folders/${folder.id}`}
            />
          ))}
        </List>
      </NavSection>
    </Nav>
  );
};

export default Navigation;
