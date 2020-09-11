import React from "react";

import Link from "next/link";
import styled from "styled-components";

const StyledImg = styled.img`
  display: block;
  height: 44px;
  width: 44px;
  border-radius: 4px;
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
  &:link {
    color: ${theme.colorNhsukBlack};
  }

  /* visited link */
  a:visited {
    color: green;
  }

  /* mouse over link */
  a:hover {
    color: hotpink;
  }

  /* selected link */
  a:active {
    color: blue;
  }

 `}
`;
// const StyledH2 = styled.h2`
//   ${({ theme }) => `
//     &:hover {
//       color: ${theme.colorNhsukBlue};
//       text-decoration: underline;
//     }
//   `}
// `;

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
            <h2>{title}</h2>
          </StyledLink>
        </Link>
      </StyledTextContainer>
    </StyledContainer>
  );
};

export default WorkspaceDirectoryItem;
