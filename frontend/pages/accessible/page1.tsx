import React, { useEffect, createRef } from "react";
import { PageLayout } from "../../components/PageLayout";
import { Header } from "../../components/Header";
import Head from "next/head";
import Link from "next/link";
// import { SkipNavLink, SkipNavContent } from "@reach/skip-nav";
// import "@reach/skip-nav/styles.css";
// import SkipNav from 'react-skip-nav';

//
// class AccessibilityTest extends React.Component<any, any>{
//
// }

const AccessibilityTest = () => {
  const contentRef: any = createRef();
  useEffect(() => {
    console.log("use effect");
    if (contentRef && contentRef.current) {
      console.log("use effect - inside");
      contentRef.current.focus();
    }
  }, []);

  return (
    <PageLayout>
      <Header
        imageRight={require("../../public/NHS.png")}
        imageRightURL="https://www.nhs.co.uk"
        imageRightAltText="NHS logo"
        aria-hidden="true"
      />
      <Head>
        <title>FutureNHS Accessibility Page 1</title>
      </Head>
      {/*<SkipNavLink />*/}
      <nav>
        Nav Bar:
        <Link href="/accessible/page2">
          <a>Page 2</a>
        </Link>
      </nav>

      {/*<SkipNav id='main-content' text='skip to main content' targetDomId='main-content'/>*/}

      <section>
        <header>
          <h1 ref={contentRef} tabIndex={-1}>
            This is a Header - NextJS
          </h1>
        </header>

        {/*<SkipNavContent>*/}
        <main>This is NEXTJS page ONE</main>
        {/*</SkipNavContent>*/}
      </section>
    </PageLayout>
  );
};

export default AccessibilityTest;
