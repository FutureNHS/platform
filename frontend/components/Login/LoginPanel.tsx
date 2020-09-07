import React from "react";
import { Panel } from "nhsuk-react-components";
import styled from "styled-components";
import { MainHeading } from "../../components/MainHeading";

type LoginPanelProps = {
  label: string;
  text: string;
};

const StyledLoginPanel = styled(Panel)`
  ${({ theme }) => `
    background-color: transparent;
    padding: 0px;

    display: flex;
    flex-direction: column;
    justify-content: center;

    @media (min-width: 1200px) {
      padding-left: 108px;
    }

    h2 {
      color: ${theme.colorNhsukGrey1};
    }
  `}
`;

const LoginPanel = ({ label, text }: LoginPanelProps) => {
  return (
    <StyledLoginPanel>
      <MainHeading>{label}</MainHeading>
      <h2>{text}</h2>
    </StyledLoginPanel>
  );
};

export default LoginPanel;
