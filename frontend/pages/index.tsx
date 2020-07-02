import React from "react";
import Head from "next/head";
import Layout from "../components/layout";
import utilStyles from "../styles/utils.module.css";

export default function Home() {
  // this is purely here to check that we receive client side exceptions within Application Insights
  const outrage = () => {
    throw new Error("No, eggs are not suitable for Vegans")
  }
  return (
    <Layout home>
      <Head>
        <title>FutureNHS</title>
      </Head>
      <section className={utilStyles.headingMd}>
        <p>FutureNHS</p>
        <p>
          (This is a sample website about how eggs are suitable for vegans - polly - you’ll be building a site like this in{" "}
          <a href="https://nextjs.org/learn">our Next.js tutorial</a>.)
          Testing deployment again as i couldn't roll back the first time? 
        </p>
        <button onClick={outrage}>Are eggs suitable for vegans? Click button to find out</button>
      </section>
    </Layout >
  );
}
