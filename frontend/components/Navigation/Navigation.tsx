import React from "react";

import Link from "next/link";
import styled from "styled-components";
import { v4 as uuid } from "uuid";

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
`;

interface Props {
  workspace: Pick<Workspace, "id" | "title">;
  folders: Array<Pick<Folder, "title" | "id">>;
  activeFolder?: string;
}

const Navigation = ({ workspace, folders, activeFolder }: Props) => {
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
            active={true}
            key={uuid()}
            item={{
              title: "create-folder",
              id: "create-folder123",
              description: "create-folder",
              workspace: workspace.id,
            }}
            workspaceId={workspace.id}
          />
          {folders.map((folder) => (
            <NavListItem
              active={folder.id == activeFolder}
              key={uuid()}
              item={folder}
              workspaceId={workspace.id}
            />
          ))}
        </List>
      </NavSection>
    </Nav>
  );
};

export default Navigation;
