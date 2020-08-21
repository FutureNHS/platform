import React from "react";
import styled from "styled-components";

type HeaderProps = {
  imageLeft: string;
  imageRight: string;
  imageRightURL?: string;
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

const Header = ({ imageLeft, imageRight, imageRightURL }: HeaderProps) => {
  return (
    <StyledHeader>
      <StyledImg src={`/${imageLeft}`} alt={imageLeft} className="imageLeft" />
      <a href={imageRightURL}>
        <StyledImg
          src={`/${imageRight}`}
          alt={imageRight}
          className="imageRight"
        />
      </a>
    </StyledHeader>
  );
};

export default Header;
