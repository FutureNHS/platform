import React, { FC } from "react";

import classNames from "classnames";
import styled from "styled-components";

import { TickIcon, CrossIcon } from "../Icon";

const StyledWrapper = styled.div`
  display: flex;
  flex-direction: row;
  height: 26px;
  line-height: inherit;
  padding: 2px 0;
  ${({ theme }) => `
    background-color: ${theme.colorNhsukWhite};
  `}

  .icon-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    height: fit-content;
  }
`;

const StyledMessage = styled.div`
  text-transform: uppercase;
  padding: 0 4px;
  ${({ theme }) => `
    color: ${theme.colorNhsukRed};
    &.success {
      color: ${theme.colorNhsukGreen};
    }
  `}
`;
interface Props {
  successStatus: Boolean;
  successMessage?: string;
  failedMessage?: string;
  className?: string;
}
const StatusTag: FC<Props> = ({
  successStatus,
  successMessage,
  failedMessage,
  className,
}) => {
  return (
    <StyledWrapper>
      {successStatus ? <TickIcon /> : <CrossIcon />}
      <StyledMessage
        className={classNames(
          {
            success: successStatus,
          },
          className
        )}
      >
        {successStatus ? successMessage : failedMessage}
      </StyledMessage>
    </StyledWrapper>
  );
};

export default StatusTag;
