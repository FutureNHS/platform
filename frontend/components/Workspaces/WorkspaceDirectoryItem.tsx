import React from "react";

import Link from "next/link";
import styled from "styled-components";

const StyledContainer = styled.div`
  display: flex;
  margin-bottom: 40px;
`;

const StyledLink = styled.a`
  ${({ theme }) => `
  display: flex;
  img {
    display: block;
    height: 44px;
    width: 44px;
    border-radius: 4px;
    cursor: pointer;

    &:hover {
      & + h3 {
        color: ${theme.colorNhsukBlue};
        text-decoration: underline;
      }
    }
  }
  h3 {
    margin-left: 25px;
    color: ${theme.colorNhsukBlack};

    &:hover {
      color: ${theme.colorNhsukBlue};
      text-decoration: underline;
      cursor: pointer;
      }

    &:active {
      color: ${theme.colorNhsukBlack};
      text-decoration: none;
      background-color: ${theme.colorNhsukYellow};
      border-bottom: 2px solid ${theme.colorNhsukBlack};
      }
    }
 `}
`;

interface Props {
  title: string;
  id: string;
}

const WorkspaceDirectoryItem = ({ title, id }: Props) => {
  return (
    <StyledContainer>
      <Link href="/workspaces/[id]" as={`/workspaces/${id}`}>
        <StyledLink>
          <img
            src={require("../../public/Placeholder_Workspace_Image.svg")}
            alt="https://www.nhs.co.uk"
          />
          <h3>{title}</h3>
        </StyledLink>
      </Link>
    </StyledContainer>
  );
};

export default WorkspaceDirectoryItem;
