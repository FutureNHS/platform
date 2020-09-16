import React from "react";

import styled from "styled-components";

const StyledHeader = styled.header`
  ${({ theme }) => `
  background-color: ${theme.colorNhsukWhite};
  padding: 20px;
  display: flex;
  justify-content: space-between;
  `}
`;

const StyledImg = styled.img`
  display: block;
  height: 40px;
  ${({ theme }) => `
  @media (max-width: ${theme.mqBreakpoints.tablet}) {
      height: 32px;
    }

  @media (max-width: ${theme.mqBreakpoints.mobile}) {
      height: 28px;
    }
  `}
`;

const Header = () => {
  return (
    <StyledHeader>
      <StyledImg
        src={require("../../public/FutureNHS.svg")}
        alt="FutureNHS logo"
      />
      <a href="https://www.nhs.co.uk">
        <StyledImg
          src={require("../../public/NHS.svg")}
          alt="https://www.nhs.co.uk"
        />
      </a>
    </StyledHeader>
  );
};

export default Header;
