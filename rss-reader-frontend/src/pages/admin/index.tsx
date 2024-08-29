import React, { useEffect, useState } from "react";
import { signIn, signOut, useSession } from "next-auth/react";
import RawFeed, { RawFeedInput } from "@/components/rawFeed";

export default function Admin() {
  const { data: session } = useSession()
  const [ rawFeeds, setRawFeeds ] = useState<RawFeedInput[]>([]);
  const [ loading, setLoading ] = useState(true);

  useEffect(() => {
    if (session) {
      const fetchRawFeeds = async () => {
        try {
          const response = await fetch('/api/rawFeeds')

          if (!response.ok) {
            throw new Error("Failed to fetch feed data")
          }

          const data = await response.json()
          setRawFeeds(data)
        } catch (error) {
          console.error("Error fetching feed data:", error)
        } finally {
          setLoading(false)
        }
      }

      fetchRawFeeds()
    }
  }, [session])

  if (!session) {
    return (
      <>
        Not signed in <br />
        <button onClick={() => signIn()}>Sign in</button>
      </>
    )
  }

  if (loading) {
    return <div>Fetching feed data...</div>
  }

  return (
    <>
      Admin: {session?.user?.name} <br />
      <button onClick={() => signOut()}>Sign out</button>
      
      {rawFeeds.map((feed) => <RawFeed key={`raw-feed-${feed.id}`} rawFeed={feed} />)}
    </>
  );
}