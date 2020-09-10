import React from "react";

import Link from "next/link";
import styled from "styled-components";

const StyledImg = styled.img`
  display: block;
  height: 44px;
  width: 44px;
  border: 2px solid black;
`;

const StyledContainer = styled.div`
  display: flex;
`;

const StyledTextContainer = styled.div`
  margin: 0 25px;
`;

const StyledH2 = styled.h2`
  ${({ theme }) => `
    &:hover {
      color: ${theme.colorNhsukBlue};
      text-decoration: underline;
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
        src={require("../../public/NHS.svg")}
        alt="https://www.nhs.co.uk"
      />
      <StyledTextContainer>
        <Link href="/workspaces/[id]" as={`/workspaces/${id}`}>
          {/* <a> */}
          <StyledH2>{title}</StyledH2>
          {/* </a> */}
        </Link>
      </StyledTextContainer>
    </StyledContainer>
  );
};

export default WorkspaceDirectoryItem;
