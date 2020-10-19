import React, { FC } from "react";

import classNames from "classnames";
import styled from "styled-components";

import { TickIcon, CrossIcon } from "../Icon";

const StyledWrapper = styled.div`
  display: flex;
  flex-direction: row;
  background-color: white;
  height: 21px;

  .icon-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
  }
`;

const StyledMessage = styled.div`
  color: red;
  flex-grow: 1;
  &.success {
    color: green;
  }
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
