import React from "react";

import Link from "next/link";
import styled from "styled-components";

const StyledImg = styled.img`
  display: block;
  height: 44px;
  width: 44px;
  border-radius: 4px;

  &:hover {
    & + div > a > h3 {
      ${({ theme }) => `
        color: ${theme.colorNhsukBlue};`}
      text-decoration: underline;
    }
  }
`;

const StyledContainer = styled.div`
  display: flex;
  margin-bottom: 40px;
`;

const StyledTextContainer = styled.div`
  margin: 0 25px;
`;

const StyledLink = styled.a`
  ${({ theme }) => `
    h3 {
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
 `}
`;

interface Props {
  title: string;
  id: string;
}

const WorkspaceDirectoryItem = ({ title, id }: Props) => {
  return (
    <StyledContainer>
      <StyledImg
        src={require("../../public/Placeholder_Workspace_Image.svg")}
        alt="https://www.nhs.co.uk"
      />
      <StyledTextContainer>
        <Link href="/workspaces/[id]" as={`/workspaces/${id}`}>
          <StyledLink>
            <h3>{title}</h3>
          </StyledLink>
        </Link>
      </StyledTextContainer>
    </StyledContainer>
  );
};

export default WorkspaceDirectoryItem;
