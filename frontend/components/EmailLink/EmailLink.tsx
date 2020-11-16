import React from "react";

interface Props {
  emailAddress: string;
}

const EmailLink = ({ emailAddress }: Props) => (
  <a
    href={`mailto:${encodeURI(emailAddress)}`}
    target="_blank"
    rel="noreferrer"
  >
    {emailAddress}
  </a>
);

export default EmailLink;
