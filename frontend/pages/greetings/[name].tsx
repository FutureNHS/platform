import React, { useState } from "react";
import Link from "next/link";
import fetch from "node-fetch";

import { GetServerSideProps } from "next";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const res = await fetch(
    `http://hello-world.hello-world/hello/${context?.params?.name}`
  );

  console.log("*****8888", typeof window);

  const greeting = await res.text();

  return {
    props: {
      greeting,
    },
  };
};

const GreetingForm = ({ greeting }: { greeting: object }) => {
  const [inputValue, setInputValue] = useState("");
  return (
    <>
      <section>
        <div>Enter a name</div>

        <div>{greeting}</div>
        <input
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
        />

        <Link href="/greetings/[name]" as={`/greetings/${inputValue}`}>
          <button>greet</button>
        </Link>
      </section>
      <section>
        <Link href="/">
          <a>home</a>
        </Link>
      </section>
    </>
  );
};

export default GreetingForm;
