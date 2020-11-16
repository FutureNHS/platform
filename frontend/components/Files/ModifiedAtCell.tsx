import React, { FC } from "react";

import { format, parseISO } from "date-fns";
import styled from "styled-components";

import { File } from "../../lib/generated/graphql";

const ModifiedDate = styled.span`
  color: ${({ theme }) => theme.colorNhsukGrey1};
`;

export const ModifiedAtCell: FC<File> = ({ modifiedAt }) => (
  <ModifiedDate>
    {format(parseISO(modifiedAt), "d LLL yyyy kk:mm")}
  </ModifiedDate>
);

export const MobileModifiedAtCell: FC<File> = ({ modifiedAt }) => (
  <div>
    <h4>Last modified</h4>
    <p>{format(parseISO(modifiedAt), "d LLL yyyy kk:mm")}</p>
  </div>
);
