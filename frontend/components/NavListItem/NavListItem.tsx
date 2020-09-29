import React, { FC } from "react";

import Link from "next/link";
import styled from "styled-components";

import { Folder } from "../../lib/generated/graphql";

type ListItem = Pick<Folder, "id" | "title">;

interface Props {
  active: boolean;
  altText: string;
  item: ListItem;
  imgSrc: string;
  className?: string;
  href: string;
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

const NavListItem: FC<Props> = ({
  active,
  altText,
  item,
  imgSrc,
  className,
  href,
}: Props) => (
  <ListItem active={active} className={className}>
    <Link href={href}>
      <a>
        <img src={imgSrc} alt={altText} />
        <div>{item.title}</div>
      </a>
    </Link>
  </ListItem>
);

export default NavListItem;
