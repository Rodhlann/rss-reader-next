import Head from "next/head";
import { useEffect, useState } from "react";
import { FeedContent } from "@/components/FeedContent/feedContent";
import { Filter } from "@/components/Filter/filter";
import { Feed, Entry } from "./api/feeds";

export default function Home() {
  const [loading, setLoading] = useState(true);
  const [feeds, setFeeds] = useState<Feed[]>();
  const [count, setCount] = useState(5);
  const [duration, setDuration] = useState('week');
  const [categoryFilter, setCategoryFilter] = useState<string>();

  useEffect(() => {
    const fetchFeeds = async () => {
      try {
        const response = await fetch(`/api/feeds?max_entries=${count}&duration=${duration}`);
        setLoading(false);

        if (!response.ok) {
          throw new Error(await response.text());
        }

        const data = await response.json() as Feed[];
        setFeeds(data.filter((feed) => !!feed.entries.length));
      } catch (error) {
        console.error("Error fetching feed data:", error);
      }
    };

    fetchFeeds();
  }, [count, duration]);

  return (
    <>
      <Head>
        <title>RSS Reader</title>
        <meta charSet="UTF-8" />
        <meta
          name="description"
          content="Stay updated with curated RSS feeds from timpepper.dev blog topics and beyond.
          Discover interesting content aligned with Tim Pepper's interests, studies, and work."
        />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/terminal.png" />
      </Head>
      <main>
        <h1>RSS Feeds</h1>
        <div>
          Show me the&nbsp;
          <select defaultValue={count} onChange={(e) => setCount(Number(e.target.value))}>
            <option value="5">5</option>
            <option value="10">10</option>
            <option value="50">50</option>
            <option value="100">100</option>
          </select>
          &nbsp;latest feeds
        </div>
        <div>
          From the last&nbsp;
          <select defaultValue={duration} onChange={(e) => setDuration(e.target.value)}>
            <option value="day">Day</option>
            <option value="week">Week</option>
            <option value="month">Month</option>
            <option value="year">Year</option>
          </select>
        </div>
        <br />
        {loading ? (
          <p>Loading feeds...</p>
        ) : (
          <>
            <Filter
              categories={
                Array.from(
                  feeds?.reduce(
                    (acc, next) => acc.add(next.category),
                    new Set(),
                  ) || [],
                ) as string[]
              }
              categoryFilter={categoryFilter}
              setCategoryFilter={setCategoryFilter}
            />
            {feeds &&
              feeds
                .filter((feed) =>
                  categoryFilter ? feed.category === categoryFilter : true,
                )
                .sort((a, b) => new Date(b.entries[0].created_date).getTime() - new Date(a.entries[0].created_date).getTime())
                .map((feed) => (
                  <div
                    key={`feed-${feed.name.toLowerCase().replace(" ", "-")}`}
                  >
                    <div className="feed-header">
                      <h2>{feed.name}</h2>
                      <label className="feed-category-label">
                        {feed.category}
                      </label>
                    </div>
                    {feed.entries.map((entry: Entry) => (
                      <FeedContent
                        key={`entry-${entry.title.toLowerCase().replace(" ", "-")}`}
                        data={entry}
                      />
                    ))}
                  </div>
                ))}
          </>
        )}
      </main>
    </>
  );
}
