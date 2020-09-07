import React from "react";
interface Props {
  children: string;
}

/**
 * This component should be used on every page as the main heading.
 * It's a workaround for a known next.js accessibility issue that doesn't read out
 * headlines on navigation
 */
const MainHeading = ({ children }: Props) => {
  return <h1 aria-live="polite">{children}</h1>;
};

export default MainHeading;
