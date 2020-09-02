import React from "react";
import styled from "styled-components";

const StyledDiv = styled.div`
  ${({ theme }) => `
  min-height: 421px;
  min-width: 287px;
  background-color: ${theme.colorNhsukGrey5};
  margin: 40px 0;
  border: 2px solid black;

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

  .localAccount {
      border: 2px solid red;
      padding: 32px 40px;
  }

  .entry-item {
      display: flex;
      flex-direction: column;
      label {
          margin-bottom: 8px;
      }
      input {
          margin-bottom: 24px;
          max-width: 400px;
      }
  }
  .password-label {

  }

  button {
      height: 56px;
      width: 154px;
  }
 a {
     top: -100;
 }
`}
`;

const Login = () => {
  return (
    <StyledDiv id="api">
      <form
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
      </form>
    </StyledDiv>
  );
};

export default Login;
