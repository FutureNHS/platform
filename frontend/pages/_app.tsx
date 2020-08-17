import React from "react";
import App from "next/app";
import { withApplicationInsights } from "../components/next-applicationinsights";
import "../styles/global.css";
import "./_app.scss";

class MyApp extends App {
  render() {
    const { Component, pageProps } = this.props;

    return <Component {...pageProps} />;
  }
}

export default withApplicationInsights({
  instrumentationKey: process.env.NEXT_PUBLIC_INSTRUMENTATION_KEY,
  isEnabled: process.env.NODE_ENV === "production",
})(MyApp);
