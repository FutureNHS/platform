import React from "react";
import styled from "styled-components";

const StyledDiv = styled.div`
  ${({ theme }) => `
    min-height: 421px;
    min-width: 287px;
    background-color: ${theme.colorNhsukGrey5};
    border-radius: 4px;
    margin: 40px 0;

    @media (min-width: ${theme.mqBreakpoints.tablet}) {
      min-width: 435px;
    }

    @media (min-width: ${theme.mqBreakpoints.desktop}) {
      min-width: 477px;
    }

    @media (min-width: ${theme.mqBreakpoints.largeDesktop}) {
      min-width: 481px;
      min-height: 504px;
    }

    @media (min-width: 1200px) {
      input {
        padding: 6px 14px;
        }
      button {
        width: 154px;
        height: 56px;
        font-size: 19px;
        }
    }

    .localAccount {
        padding: 32px 40px;
        }
    label {
        margin-bottom: 8px;
        }

    label:after {
        content: " *";
      }
    input {
      margin-bottom: 24px;
      padding: 8px 12px;
      border: solid 2px #3d4448;
      }
    .entry-item {
        display: flex;
        flex-direction: column;
      }
    .password-label {
      display: flex;
      flex-direction: row;
      justify-content: space-between;
      }
    button {
      width: 97px;
      height: 44px;
      background-color: ${theme.colorNhsukBlue};
      color: ${theme.colorNhsukWhite};
      border-radius: 4px;
      box-shadow: 0 4px 0 0 #002a8e;
      font-size: 16px;
      font-weight: bold;
      margin-top: 20px;
      border-style: none;
      }
  `}
`;

const Login = () => {
  return <StyledDiv id="api"></StyledDiv>;
};

export default Login;
