import React from "react";
import styled from "styled-components";

type HeaderProps = {
  imageRight: string;
  imageRightURL?: string;
  imageRightAltText: string;
};

const StyledHeader = styled.header`
  ${({ theme }) => `
  background-color: ${theme.colorNhsukWhite};
  padding: 20px;
  display: flex;
  justify-content: space-between;
  `}
`;

const StyledImg = styled.img`
  ${({ theme }) => `
  max-height: 40px;

  @media (max-width: ${theme.mqBreakpoints.tablet}) {
      max-height: 32px;
    }

  @media (max-width: ${theme.mqBreakpoints.mobile}) {
      max-height: 28px;
    }
  `}
`;

const Header = ({
  imageRight,
  imageRightURL,
  imageRightAltText,
}: HeaderProps) => {
  return (
    <StyledHeader>
      <StyledImg
        src={require("../../public/FutureNHS.png")}
        alt="FutureNHS logo"
      />
      <a href={imageRightURL}>
        <StyledImg src={imageRight} alt={imageRightAltText} />
      </a>
    </StyledHeader>
  );
};

export default Header;
