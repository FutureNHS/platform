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
      position: relative;

      &:nth-child(2) {
        input {
          position: relative;
          top: -25px;
          left: 0;
        }
      }
    }

    .password-label {
      display: flex;
      flex-direction: column;

      a {
        position: relative;
        top: 50px;
      }
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
  return (
    <StyledDiv id="api">
      {/* <form
        id="localAccountForm"
        action="JavaScript:void(0);"
        className="localAccount"
        aria-label="Sign in with your email address"
      >
        <div className="intro">
          <h2>Sign in with your email address</h2>
        </div>
        <div className="entry">
          <div className="entry-item">
            <label htmlFor="email">Email Address</label>
            <input
              type="text"
              id="email"
              name="Email Address"
              placeholder="Email Address"
              value=""
            />
          </div>
          <div className="entry-item">
            <div className="password-label">
              <label htmlFor="password">Password</label>
              <a
                id="forgotPassword"
                href="/futurenhsplatform.onmicrosoft.com/B2C_1_logintest/api/CombinedSigninAndSignup/forgotPassword?csrf_token=Q0tQNm51aUlXcXAzamVxNTFuL3c4aVRuWlJQQ3QwSGZ6Yk1KR2YxU1E2WHhUZENta2tZeDVRZzZMbXpMUTF4dHlnQkI3Q2FXWjNNd2tab2ZmWDhYSkE9PTsyMDIwLTA4LTI2VDE0OjA3OjE1Ljg3MjU4MjVaO09lVllQYy8xOW5kdWVnTmhFMEJzMkE9PTt7Ik9yY2hlc3RyYXRpb25TdGVwIjoxfQ==&amp;tx=StateProperties=eyJUSUQiOiJjNTU3YTExOC03MTczLTRkZGQtOGQ5NS0xYTI3YWEzY2U2MjcifQ&amp;p=B2C_1_logintest"
              >
                Forgot your password?
              </a>
            </div>
            <input
              type="password"
              id="password"
              name="Password"
              placeholder="Password"
            />
          </div>
          <div className="working"></div>

          <div className="buttons">
            <button id="next" type="submit" form="localAccountForm">
              Sign in
            </button>
          </div>
        </div>
      </form> */}
    </StyledDiv>
  );
};

export default Login;
