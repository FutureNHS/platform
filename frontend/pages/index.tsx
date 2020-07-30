import React from "react";
import Head from "next/head";
import Layout from "../components/layout";
import utilStyles from "../styles/utils.module.css";
import Link from "next/link";

const Home = () => {
  // this is purely here to check that we receive client side exceptions within Application Insights
  const outrage = () => {
    throw new Error("No, eggs are not suitable for Vegans");
  };
  return (
    <Layout>
      <Head>
        <title>FutureNHS</title>
      </Head>
      <section className={utilStyles.headingMd}>
        <p>FutureNHS</p>
        <Link href="/greetings/tracy">
          <a>Click here for greetings</a>
        </Link>
        <div>
          <Link href="auth/login">
            <a>Login</a>
          </Link>
        </div>
        <div>
          <Link href="workspaces/private">
            <a>Private Workspace</a>
          </Link>
        </div>
        <div>
          <Link href="workspaces/pharmacy">
            <a>Pharmacy Workspace</a>
          </Link>
        </div>
        <p>
          (This is a sample website about how eggs are suitable for vegans -
          polly - you’ll be building a site like this in{" "}
          <a href="https://nextjs.org/learn">our Next.js tutorial</a>.) Today is
          Friday 3rd of July
        </p>
        <button onClick={outrage}>
          Are eggs suitable for vegans? Click button to find out
        </button>
      </section>
    </Layout>
  );
};
export default Home;
