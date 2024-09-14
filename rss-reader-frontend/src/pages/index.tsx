import Head from "next/head";
import { Inter } from "next/font/google";
import styles from "@/styles/Home.module.css";
import { useEffect, useState } from "react";
import { Feed } from "./api/feeds";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
  const [feeds, setFeeds] = useState<Feed[]>();
  
  useEffect(() => {
    const fetchFeeds = async () => {
      try {
        const response = await fetch('/api/feeds')

        if (!response.ok) {
          throw new Error(await response.text())
        }

        const data = await response.json()
        setFeeds(data)
      } catch (error) {
        console.error("Error fetching feed data:", error)
      }
    }

    fetchFeeds()
  }, [])

  return (
    <>
      <Head>
        <title>RSS Reader</title>
        <meta charSet="UTF-8" />
        <meta name="description" content="Stay updated with curated RSS feeds from timpepper.dev blog topics and beyond.
          Discover interesting content aligned with Tim Pepper's interests, studies, and work." />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
        <h1>RSS Feeds</h1>
        { feeds && feeds.map((feed) => (
          <div key={`feed-${feed.name.toLowerCase().replace(' ', '-')}`}>
            {feed.name}
            {feed.entries.map((entry) =>
              (<><br /><a key={`entry-${entry.title.toLowerCase().replace(' ', '-')}`} href={entry.url}>{entry.title}</a><br/></>)
            )}
          </div>
        )) }
      </main>
    </>
  );
}
