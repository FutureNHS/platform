import React, { FC } from "react";

import classNames from "classnames";
import styled from "styled-components";

import { TickIcon, CrossIcon } from "../Icon";

const StyledWrapper = styled.div`
  width: 121px;
  height: 28px;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
`;

const StyledMessage = styled.div`
  color: green;

  &.failed {
    colour: red;
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
