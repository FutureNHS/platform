import React from "react";
import App from "next/app";
import { withApplicationInsights } from "../components/next-applicationinsights";
import "./_app.scss";
import { ThemeProvider } from "styled-components";

// Extract our Sass variables into a JS object
const theme = require('sass-extract-loader?{"includePaths":["."],"plugins": ["sass-extract-js"]}!../node_modules/nhsuk-frontend/packages/core/all.scss');

class MyApp extends App {
  render() {
    const { Component, pageProps } = this.props;

    return (
      <ThemeProvider theme={theme}>
        <Component {...pageProps} />
      </ThemeProvider>
    );
  }
}

export default withApplicationInsights({
  instrumentationKey: process.env.NEXT_PUBLIC_INSTRUMENTATION_KEY,
  isEnabled: process.env.NODE_ENV === "production",
})(MyApp);
