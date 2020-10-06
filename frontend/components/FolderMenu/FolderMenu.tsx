import React, { FC, useEffect, useState } from "react";

import styled from "styled-components";

import { FolderMenuButton } from ".";
import {
  DeleteIcon,
  EditIcon,
  LockIcon,
  MeatballIcon,
  MoveIcon,
  UploadIcon,
} from "../Icon";
import FolderMenuList from "./FolderMenuList";
import { MenuItem } from "./FolderMenuListItem";

interface Props {
  startHidden?: boolean;
  workspaceId: string;
  folderId: string;
}

const Container = styled.div`
  align-items: center;
  position: relative;
  height: 100%;
  border-radius: 4px;
  margin-left: 7px;

  .tooltip {
    left: 36px;
    top: -7px;
  }

  ${({ theme }) => `
    .folder-menu-item {
      display: none;

      a {
        color: ${theme.colorNhsukWhite};
        &:focus {
          color: ${theme.colorNhsukBlack};
        }
      }
    }

    @media (min-width: ${theme.mqBreakpoints.largeDesktop}) {
      justify-content: space-between;
      display: flex;

      .folder-menu-item {
        display: flex;
      }
    }
  `}
`;

const FolderMenu: FC<Props> = ({ startHidden, workspaceId, folderId }) => {
  const [menuOpen, setMenuOpen] = useState(false);
  const container = React.useRef<HTMLDivElement>(null);

  useEffect(() => {
    const pageClickEvent = ({ target }: MouseEvent) => {
      if (!container.current?.contains(target as Node)) {
        setMenuOpen(false);
      }
    };

    if (menuOpen) {
      window.addEventListener("click", pageClickEvent);
    }

    return () => window.removeEventListener("click", pageClickEvent);
  }, [menuOpen]);

  const items: MenuItem[] = [
    {
      title: "Upload file to this folder",
      icon: <UploadIcon />,
      href: `/workspaces/${workspaceId}/folders/${folderId}/upload-file`,
    },
    {
      title: "Edit folder details",
      icon: <EditIcon />,
      href: "#",
    },
    {
      title: "Move folder",
      icon: <MoveIcon />,
      href: "#",
    },
    {
      title: "View folder permissions",
      icon: <LockIcon />,
      href: "#",
    },
    {
      title: "Delete folder",
      icon: <DeleteIcon />,
      href: "#",
    },
  ];

  return (
    <>
      <Container ref={container}>
        <FolderMenuButton
          menuOpen={menuOpen}
          setMenuOpen={setMenuOpen}
          startHidden={startHidden || false}
        >
          <MeatballIcon />
        </FolderMenuButton>
        {menuOpen && (
          <FolderMenuList startHidden={startHidden || false}>
            {items}
          </FolderMenuList>
        )}
      </Container>
    </>
  );
};

export default FolderMenu;