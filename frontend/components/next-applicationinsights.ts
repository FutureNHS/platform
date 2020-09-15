// This file is taken from https://github.com/goenning/next-applicationinsights
// and edited to remove the 'getInitialProps' method in order to allow for
// static generation.
import * as React from "react";

import {
  ApplicationInsights,
  IConfiguration,
  IConfig,
} from "@microsoft/applicationinsights-web";
import { AppProps, default as NextApp } from "next/app";

const IS_BROWSER = typeof window !== "undefined";

interface WithApplicationInsightsProps {
  pageName: string;
}

let appInsights: ApplicationInsights;

export interface ICustomConfig {
  isEnabled: boolean;
}

export const withApplicationInsights = (
  config: IConfiguration & IConfig & ICustomConfig
) => {
  return (App: typeof NextApp) => {
    return class WithApplicationInsights extends React.Component<
      WithApplicationInsightsProps & AppProps
    > {
      public componentDidMount() {
        this.initializeAppInsights();
        this.trackPageView();
      }

      public componentDidCatch(error: Error) {
        if (appInsights) {
          appInsights.trackException({ exception: error });
        }
      }

      public initializeAppInsights() {
        if (IS_BROWSER && config.isEnabled && !appInsights) {
          appInsights = new ApplicationInsights({ config });
          appInsights.loadAppInsights();
          // @ts-ignore This is required for App Insights to work, but we don't
          // access the property anywhere else. It's easier to ignore the error
          // here than to extend to global window type.
          window.appInsights = appInsights;
        }
      }

      public trackPageView() {
        if (appInsights) {
          const name =
            this.props.Component.displayName ||
            this.props.Component.name ||
            location.pathname;
          const properties: {
            [route: string]: string | undefined | string[];
          } = {
            route: this.props.router.route,
          };
          if (this.props.router.query) {
            for (const key in this.props.router.query) {
              properties[`query.${key}`] = this.props.router.query[key];
            }
          }
          appInsights.trackPageView({ name, properties });
        }
      }

      public render() {
        this.trackPageView();
        // @ts-ignore
        return React.createElement(App, this.props);
      }
    };
  };
};
