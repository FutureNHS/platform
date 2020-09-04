import React from "react";
import styled from "styled-components";

type GradientProps = {
  children?: React.ReactNode;
};

export const StyledGradient = styled.div`
  ${({ theme }) => `
    .gradient-container {
      height: 144px;
      width: 100%;
      background-image: linear-gradient(66deg, #ffb600, #fc8600 100%);
      background-color: ${theme.colorNhsukWhite};
      position: absolute;
      left: 0;

      @media (min-width: 1200px) {
        height: 112px;
        padding: 0 118px;
      }
    }

    .container {
      max-width: 1200px;
      padding: 0 15px;
      margin: 0 auto;
      display: flex;
      flex-direction: column;

      @media (min-width: ${theme.mqBreakpoints.tablet}) and (max-width: 1199px) {
        max-width: 800px;
      }

      @media (min-width: 1200px) {
        flex-direction: row;
      }
    }
  `}
`;

const Gradient = ({ children }: GradientProps) => {
  return (
    <StyledGradient>
      <div className="gradient-container">
        <div className="container">{children}</div>
      </div>
    </StyledGradient>
  );
};

export default Gradient;
