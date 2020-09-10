import React from "react";

import styled from "styled-components";

const StyledHorizontalRule = styled.hr`
  margin-bottom: 25px;
  ${({ theme }) => `
    color: ${theme.colorNhsukGrey1};
    border-bottom: 1px solid ${theme.colorNhsukGrey1};
  `}
`;

interface Props {
  children: string;
  withBorder?: boolean;
}

/**
 * This component should be used on every page as the main heading.
 * It's a workaround for a known next.js accessibility issue that doesn't read out
 * headlines on navigation
 */
const MainHeading = ({ children, withBorder }: Props) => {
  return (
    <>
      <h1 aria-live="polite">{children}</h1>
      {withBorder && <StyledHorizontalRule />}
    </>
  );
};

export default MainHeading;
