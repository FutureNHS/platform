import React, { HTMLProps, useState } from "react";

import { FormElementProps } from "nhsuk-react-components/lib/util/types/FormTypes";
import styled from "styled-components";

import { ChevronBottomIcon, ChevronTopIcon } from "../Icon";
import PermissionsRadio from "../PermissionsRadio/PermissionsRadio";

interface PermissionsProps
  extends HTMLProps<HTMLInputElement>,
    FormElementProps {
  inputRef?: (inputRef: HTMLInputElement | null) => any;
}

const TitleContainer = styled.div`
  display: flex;
  flex-direction: row;
  margin-bottom: 24px;

  h3 {
    margin: 0;
  }

  button {
    margin: 0 0 0 20px;
    background: none;
    color: inherit;
    border: none;
    outline: inherit;
    border-top: 4px solid transparent;
    border-bottom: 4px solid transparent;
    cursor: pointer;

    :focus {
      background-color: ${({ theme }) => theme.colorNhsukYellow};
      border-bottom: 4px solid ${({ theme }) => theme.colorNhsukBlack};
    }
  }
`;

const Description = styled.p`
  margin-bottom: 40px;
`;

const Permissions: React.FC<PermissionsProps> = ({ inputRef }) => {
  const [expanded, setExpandedState] = useState(false);
  return (
    <>
      <TitleContainer>
        <h3>Permissions</h3>
        <button
          type="button"
          data-cy="permissions-button"
          onClick={() => setExpandedState(!expanded)}
        >
          {expanded ? (
            <ChevronTopIcon title={`Hide permissions`} />
          ) : (
            <ChevronBottomIcon title={`Show permissions`} />
          )}
        </button>
      </TitleContainer>
      <Description>
        You can set permissions for this folder. All users authorised to access
        a folder can view all its contents
      </Description>
      <PermissionsRadio hidden={!expanded} inputRef={inputRef} />
    </>
  );
};

export default Permissions;
