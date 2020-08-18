import React from "react";
import Head from "next/head";
import Layout from "../components/layout";
import hypeStyles from "../styles/hype.module.css";
import Link from "next/link";

const Home = () => {
  return (
    <Layout>
      <Head>
        <title>FutureNHS</title>
      </Head>
      <div className={hypeStyles.page}>
        <img
          src="/hype/logo.png"
          srcSet="/hype/logo@2x.png 2x, /hype/logo@3x.png 3x"
          alt="FutureNHS"
        />
        <p>The new Future is coming...</p>
      </div>
      <div>
        <Link href="/auth/login">
          <a>Login</a>
        </Link>
      </div>
    </Layout>
  );
};

export default Home;
