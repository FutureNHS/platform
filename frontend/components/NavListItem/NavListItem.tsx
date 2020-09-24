import React from "react";

import Link from "next/link";
import styled from "styled-components";

import { Folder } from "../../lib/generated/graphql";

type ListItem = Pick<Folder, "id" | "title">;

interface Props {
  active: boolean;
  item: ListItem;
  workspaceId: string;
  imgSrc: string;
}

const ListItem = styled.li<{ active: boolean }>`
  background: ${({ active, theme }) =>
    active ? theme.colorNhsukYellow : "inherit"};
  border-radius: 4px;
  font-weight: ${({ active }) => (active ? 700 : "inherit")};
  list-style-type: none;
  margin-bottom: 12px;
  &:hover {
    ${({ theme }) => `
    background-color: ${theme.colorNhsukGrey4}
  `}
  }
  a {
    display: flex;
    padding-left: 8px;
    text-decoration: none;
    &:focus {
      box-shadow: none;
    }
  }
  img {
    padding-top: 2px;
    align-self: baseline;
    /* stop things jumping about while svgs load */
    width: 24px;
    flex-shrink: 0;
  }
  div {
    padding-left: 4px;
    padding-right: 4px;
    ${({ theme }) => `
    color: ${theme.colorNhsukBlack};
  `}
  }
`;

const NavListItem = ({ active, item, workspaceId, imgSrc }: Props) => (
  <ListItem active={active}>
    <Link href={`/workspaces/${workspaceId}/folders/${item.id}`}>
      <a>
        <img src={imgSrc} />
        <div>{item.title}</div>
      </a>
    </Link>
  </ListItem>
);

export default NavListItem;
