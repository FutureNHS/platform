import React from "react";

import { H2 } from "../H2";
import { MainHeading } from "../MainHeading";

interface ErrorProps {
  title: string;
  description: string;
}

const Error: React.FC<ErrorProps> = ({ title, description }) => {
  return (
    <>
      <MainHeading>Error</MainHeading>
      <H2 title={title} />
      <br />
      <p>{description}</p>
    </>
  );
};

export default Error;
