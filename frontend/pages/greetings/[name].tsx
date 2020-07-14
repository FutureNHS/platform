import React, { useState, useEffect } from "react";
import Link from "next/link";
import axios from "axios";

import { GetServerSideProps } from "next";
import { getGreeting } from "../../lib/greetings";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const nameProp = context?.params?.name || "";
  const greeting = await getGreeting(nameProp);

  return {
    props: {
      greeting,
      nameProp,
    },
  };
};

const GreetingForm = ({
  greeting,
  nameProp,
}: {
  greeting: object;
  nameProp: string;
}) => {
  const [inputValue, setInputValue] = useState("");
  const [inputValueClient, setInputValueClient] = useState("");
  const [axiosValue, setAxiosValue] = useState("");

  const callAndSetGreeting = async (name: string) => {
    try {
      const res = await axios.get(`/hello/${name}`);

      setAxiosValue(res.data);
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    callAndSetGreeting(nameProp);
  }, []);

  return (
    <>
      <section>
        <div>Server side rendering example</div>
        <div>Enter a name</div>
        <div>{greeting}</div>
        <input
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
        />
        {/* shallow - Update the path of the current page without rerunning */}
        <Link href="/greetings/[name]" as={`/greetings/${inputValue}`}>
          <button>greet</button>
        </Link>
      </section>
      <section>
        <Link href="/">
          <a>home</a>
        </Link>
      </section>
      <section>
        <div>Client side rendering example</div>
        <div>Enter a name</div>

        <div>{axiosValue}</div>
        <input
          value={inputValueClient}
          onChange={(e) => setInputValueClient(e.target.value)}
        />

        <button onClick={() => callAndSetGreeting(inputValueClient)}>
          greet
        </button>
      </section>
    </>
  );
};

export default GreetingForm;
