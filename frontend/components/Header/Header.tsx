import React from "react";
import styled from "styled-components";

type HeaderProps = {
  imageLeft: string;
  imageRight: string;
  imageRightURL?: string;
};

const StyledHeader = styled.header`
  background-color: ${({ theme }) => theme.colorNhsukWhite};
  padding: 20px;
  display: flex;
  justify-content: space-between;

  .imageleft: {
    height: 40px;
  }
`;

const Header = ({ imageLeft, imageRight, imageRightURL }: HeaderProps) => {
  return (
    <StyledHeader>
      <img src={`/${imageLeft}`} alt={imageLeft} className="imageLeft" />
      <a href={imageRightURL}>
        <img src={`/${imageRight}`} alt={imageRight} className="imageRight" />
      </a>
    </StyledHeader>
  );
};

export default Header;
