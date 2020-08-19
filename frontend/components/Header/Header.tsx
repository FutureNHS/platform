import React from "react";
import styled from "styled-components";

type HeaderProps = {
  imageLeft: string;
  imageRight: string;
};

const StyledHeader = styled.header`
  background-color: ${({ theme }) => theme.colorNhsukWhite};
  padding: 20px;
  display: flex;
  justify-content: space-between;
`;

const Header = ({ imageLeft, imageRight }: HeaderProps) => {
  return (
    <StyledHeader>
      <img src={`/${imageLeft}`} alt={imageLeft} className="imageLeft" />
      <img src={`/${imageRight}`} alt={imageRight} className="imageRight" />
    </StyledHeader>
  );
};

export default Header;
